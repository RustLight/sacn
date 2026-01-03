#![no_std]
#![no_main]

mod sdram_drv;

use defmt::*;

use embassy_executor::Spawner;
use embassy_net::udp::{PacketMetadata, UdpSocket};
use embassy_net::{IpListenEndpoint, Stack, StackResources};
use embassy_stm32::bind_interrupts;
use embassy_stm32::eth::{Ethernet, GenericPhy, PacketQueue, Sma};
use embassy_stm32::fmc::Fmc;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::{ETH, ETH_SMA};
use embassy_stm32::rng::Rng;
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Timer};

extern crate alloc;
use embedded_alloc::TlsfHeap as Heap;

use sacn::packet::{
    AcnRootLayerProtocol, SynchronizationPacketFramingLayer, UniverseDiscoveryPacketFramingLayer,
    UniverseDiscoveryPacketUniverseDiscoveryLayer, universe_to_ipv4_multicast_addr,
};
use static_cell::StaticCell;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    ETH => embassy_stm32::eth::InterruptHandler;
    RNG => embassy_stm32::rng::InterruptHandler<embassy_stm32::peripherals::RNG>;
});

type Device = Ethernet<'static, ETH, GenericPhy<Sma<'static, ETH_SMA>>>;
#[embassy_executor::task]
async fn net_task(mut runner: embassy_net::Runner<'static, Device>) {
    runner.run().await;
}

fn join_multicast_group(stack: &mut Stack<'static>, universe: u16) {
    match universe_to_ipv4_multicast_addr(universe) {
        Ok(addr) => {
            let r = stack.join_multicast_group(*addr.ip());
            if let Err(e) = r {
                error!("Multicast error: {}", e);
            }
        }
        Err(e) => {
            error!(
                "Error joining multicast group {}: {}",
                universe,
                Debug2Format(&e)
            )
        }
    }
}

#[embassy_executor::task]
async fn sacn_listener(mut stack: Stack<'static>) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut rx_meta = [PacketMetadata::EMPTY; 16];
    let mut tx_meta = [PacketMetadata::EMPTY; 16];

    let mut buf = [0u8; 1024];

    let mut socket = UdpSocket::new(
        stack,
        &mut rx_meta,
        &mut rx_buffer,
        &mut tx_meta,
        &mut tx_buffer,
    );
    socket
        .bind(IpListenEndpoint {
            addr: None,
            port: sacn::packet::ACN_SDT_MULTICAST_PORT,
        })
        .unwrap();

    join_multicast_group(&mut stack, sacn::packet::E131_DISCOVERY_UNIVERSE);

    loop {
        socket.recv_from(&mut buf).await.unwrap();
        match sacn::packet::AcnRootLayerProtocol::parse(&buf) {
            Ok(acn) => {
                info!("[acn]: {}", acn);
                let pdu = acn.pdu;
                let data = pdu.data;
                match data {
                    sacn::packet::E131RootLayerData::DataPacket(data) => {
                        info!("[data]: {}", data,);
                    }
                    sacn::packet::E131RootLayerData::SynchronizationPacket(sync) => {
                        info!("[sync]: {}", sync);
                    }
                    sacn::packet::E131RootLayerData::UniverseDiscoveryPacket(disco) => {
                        info!("[disco]: {}", disco);
                        let UniverseDiscoveryPacketFramingLayer { source_name, data } = disco;
                        let UniverseDiscoveryPacketUniverseDiscoveryLayer {
                            page,
                            last_page,
                            universes,
                        } = data;
                        for universe in universes.iter() {
                            if let Ok(addr) = universe_to_ipv4_multicast_addr(*universe)
                                && !stack.has_multicast_group(*addr.ip())
                            {
                                let r = stack.join_multicast_group(*addr.ip());
                                if let Err(e) = r {
                                    error!(
                                        "Unable to join multicast group universe: {}, ip: {}, error: {}",
                                        universe,
                                        addr.ip(),
                                        e
                                    );
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                error!("sACN error: {}", Debug2Format(&e));
            }
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    // ------------------------------------------------------------------------
    // Configure clocks
    use embassy_stm32::rcc::{
        AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPDiv, PllPreDiv, PllQDiv,
        PllSource, Sysclk,
    };

    let mut config = embassy_stm32::Config::default();
    config.rcc.sys = Sysclk::PLL1_P;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV4;
    config.rcc.apb2_pre = APBPrescaler::DIV2;

    // HSE is on and ready
    config.rcc.hse = Some(Hse {
        freq: mhz(25),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll_src = PllSource::HSE;

    config.rcc.pll = Some(Pll {
        prediv: PllPreDiv::DIV25,  // PLLM
        mul: PllMul::MUL400,       // PLLN
        divp: Some(PllPDiv::DIV2), // SYSCLK = 400/2 = 200 MHz
        divq: Some(PllQDiv::DIV9), // PLLQ = 400/9 = 44.44 MHz
        divr: None,
    });
    let p = embassy_stm32::init(config);

    info!("Starting...");

    // Config SDRAM and HEAP storage
    // ----------------------------------------------------------
    // Configure MPU for external SDRAM (64 Mbit = 8 Mbyte)
    // MPU is disabled by default
    const SDRAM_SIZE: usize = 8 * 1024 * 1024;

    #[rustfmt::skip]
    let mut sdram = Fmc::sdram_a12bits_d16bits_4banks_bank1(
        p.FMC,
        // A0-A11
        p.PF0, p.PF1, p.PF2, p.PF3, p.PF4, p.PF5, p.PF12, p.PF13, p.PF14, p.PF15, p.PG0, p.PG1,
        // BA0-BA1
        p.PG4, p.PG5,
        // D0-D15
        p.PD14, p.PD15, p.PD0, p.PD1, p.PE7, p.PE8, p.PE9, p.PE10, p.PE11, p.PE12, p.PE13, p.PE14, p.PE15, p.PD8, p.PD9, p.PD10,
        // NBL0 - NBL1
        p.PE0, p.PE1,
        p.PC3,  // SDCKE0
        p.PG8,  // SDCLK
        p.PG15, // SDNCAS
        p.PH3,  // SDNE0 (!CS)
        p.PF11, // SDRAS
        p.PH5,  // SDNWE
        sdram_drv::mt48lc4m32b2_6::Mt48lc4m32b2 {},
    );

    let mut delay = embassy_time::Delay;

    unsafe {
        // Initialise controller and SDRAM
        let ram_ptr: *mut u32 = sdram.init(&mut delay) as *mut _;

        info!("SDRAM Initialized at {:x}", ram_ptr as usize);

        // Convert raw pointer to slice and initialize the HEAP
        HEAP.init(ram_ptr as usize, SDRAM_SIZE)
    };

    // Config RNG
    // ----------------------------------------------------------
    let mut rng = Rng::new(p.RNG, Irqs);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    // Config Ethernet
    // ----------------------------------------------------------
    // By default this is configured for DHCP
    let config = embassy_net::Config::dhcpv4(Default::default());
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];
    static PACKETS: StaticCell<PacketQueue<16, 16>> = StaticCell::new();
    let device = Ethernet::new(
        PACKETS.init(PacketQueue::<16, 16>::new()),
        p.ETH,
        Irqs,
        p.PA1,
        p.PA7,
        p.PC4,
        p.PC5,
        p.PG13,
        p.PG14,
        p.PG11,
        mac_addr,
        p.ETH_SMA,
        p.PA2,
        p.PC1,
    );

    // Initialise network stack resources
    // ----------------------------------------------------------
    static RESOURCES: StaticCell<StackResources<8>> = StaticCell::new();
    let (stack, runner) =
        embassy_net::new(device, config, RESOURCES.init(StackResources::new()), seed);

    // Spawn the network handler and wait for a valid configuration
    // ----------------------------------------------------------
    spawner.spawn(net_task(runner).unwrap());
    stack.wait_link_up().await;
    stack.wait_config_up().await;
    info!("Network task initialized");

    // Spawn the main sACN listener task
    // ----------------------------------------------------------
    spawner.spawn(sacn_listener(stack).unwrap());

    let mut led = Output::new(p.PI1, Level::High, Speed::Low);

    // Blink an LED, just in case
    loop {
        led.set_high();
        Timer::after(Duration::from_millis(100)).await;
        led.set_low();
        Timer::after(Duration::from_millis(900)).await;
    }
}
