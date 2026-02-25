// Copyright 2025 sacn Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// Simply listens on the given universe and shows the current data.

use crossterm::style::Print;
use crossterm::{cursor, execute, queue, terminal};
use std::io::{self, Stdout, Write};
use std::net::SocketAddr;
use std::time::{Duration, Instant};

use sacn::packet::ACN_SDT_MULTICAST_PORT;
use sacn::receive::{DMXData, SacnReceiver};

fn main() {
    let interface_ip = "127.0.0.1";
    let universe = 1;
    let duration = Duration::from_secs(12);
    let max_wait = Duration::from_secs(1);
    let mut dmx_recv = SacnReceiver::with_ip(
        SocketAddr::new(interface_ip.parse().unwrap(), ACN_SDT_MULTICAST_PORT),
        None,
    )
    .unwrap();
    dmx_recv.listen_universes(&[universe]).unwrap();

    println!("Started");

    let start = Instant::now();
    let mut remaining = duration
        .checked_sub(start.elapsed())
        .unwrap_or(Duration::from_millis(0));

    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    while remaining > Duration::from_millis(250) {
        match dmx_recv.recv(Some(max_wait)) {
            Ok(data) => {
                display_data(&mut stdout, &remaining, &data[0]);
            }
            Err(e) => {
                execute!(
                    stdout,
                    cursor::MoveTo(0, 0),
                    terminal::Clear(terminal::ClearType::All)
                )
                .unwrap();
                println!(
                    "{} - universe {}: {}ms left",
                    e,
                    universe,
                    remaining.as_millis()
                );
            }
        }
        remaining = duration
            .checked_sub(start.elapsed())
            .unwrap_or(Duration::from_millis(0));
    }
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
}

fn display_data(stdout: &mut Stdout, remaining: &Duration, data: &DMXData) {
    // Don't worry about this bit - its just for actually displaying the data
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        Print(format!(
            "Received from universe {}: {}ms left",
            data.universe,
            remaining.as_millis()
        ))
    )
    .unwrap();
    for y in 0..16 {
        let cursor_y = y + 1;
        queue!(stdout, cursor::MoveTo(0, cursor_y), Print("|")).unwrap();
        for x in 0..32 {
            let data_index = ((y * 16 + x) + 1) as usize;
            let cursor_x = x * 4 + 1;
            queue!(
                stdout,
                cursor::MoveTo(cursor_x, cursor_y),
                Print(format!("{:03}|", data.values[data_index]))
            )
            .unwrap();
        }
    }
    stdout.flush().unwrap();
}
