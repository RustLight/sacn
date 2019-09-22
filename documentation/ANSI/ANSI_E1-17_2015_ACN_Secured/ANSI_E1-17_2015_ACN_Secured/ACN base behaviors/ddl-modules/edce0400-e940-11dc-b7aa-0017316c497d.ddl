<?xml version="1.0" encoding="UTF-8"?>
<DDL
version="1.1"
xml:id="acnbase.lset.DDL"
><!--Part of BSR E1.17-200x, a revision of ANSI E1.17-2006.
ESTA TSP document ref. CP/2008-1016
$Id: acnbase-behaviors.ddl 443 2009-11-04 15:58:40Z philip $--><languageset
UUID="edce0400-e940-11dc-b7aa-0017316c497d"
date="2010-09-03"
provider="http://www.esta.org/ddl/acn-core/"
xml:id="acnbase.lset"
><UUIDname
UUID="edce0400-e940-11dc-b7aa-0017316c497d"
name="acnbase.lset"
></UUIDname
><label
key="acnbase.lset"
set="acnbase.lset"
></label
><language
lang="en-US"
><string
key="acnbase.lset"
>Labels for ACN base behaviorset</string
><string
key="acnbase.bset"
>ACN base behaviorset</string
><string
key="ESTA_OrgID"
>ESTA Manufacturer ID</string
><string
key="IEEE_OUI"
>IEEE Organization Unique Identifier</string
><string
key="ISOdate"
>ISO 8601 Format Date String</string
><string
key="FCTN"
>Fixed Component Type Name (FCTN see ACN-epi19)</string
><string
key="reference"
>Reference or pointer property</string
><string
key="accessClass"
>Access Classes</string
><string
key="accessOrder"
>Access-Order Sensitivity</string
><string
key="actionTimer"
>Action Timer</string
><string
key="algorithm"
>Algorithm Behavior</string
><string
key="angle"
>Angle</string
><string
key="angleX"
>Angle about x-axis</string
><string
key="angleY"
>Angle about y-axis</string
><string
key="angleZ"
>Angle about z-axis</string
><string
key="arraySize"
>Array Size</string
><string
key="atTime"
>At Time</string
><string
key="atomicLoad"
>Atomic action setting multiple properties</string
><string
key="atomicMaster"
>Master property of atomic load group</string
><string
key="atomicGroupMember"
>Member property of atomic load group</string
><string
key="atomicParent"
>Parent of hierarchical atomic load group</string
><string
key="atomicTrigger"
>Trigger property for atomic action</string
><string
key="atomicWithAncestor"
>Member of atomic group slaved to ancestor property</string
><string
key="atomicMasterRef"
>Reference pointing to master property of an atomic group</string
><string
key="beamShape"
>Beam Shape</string
><string
key="beamTemplate"
>Beam Template</string
><string
key="behaviorRef"
>Behavior Reference</string
><string
key="behaviorsetID"
>Behavior Set Identifier</string
><string
key="binder"
>Binding control property</string
><string
key="binObject"
>Binary Object</string
><string
key="bitmap"
>Bitmap</string
><string
key="boolean"
>Boolean</string
><string
key="case"
>Case – One of a Set of Cases</string
><string
key="character"
>Character</string
><string
key="choice"
>Choice Property</string
><string
key="CID"
>Component Identifier (CID)</string
><string
key="CIDreference"
>Reference to a Component by CID</string
><string
key="colorFilter"
>Color Filter</string
><string
key="colorSpec"
>Color Specifier</string
><string
key="componentReference"
>Reference Pointing to a Component</string
><string
key="connectedSwitch"
>Connection Dependent Switch</string
><string
key="connectionDependent"
>Connection Dependent Property</string
><string
key="transportConnection"
>Network Transport Connection Identifier</string
><string
key="connectionReporter"
>Connection reporter property</string
><string
key="connection.ESTA.SDT"
>SDT connection identifier</string
><string
key="connection.ESTA.DMP"
>DMP connection identifier</string
><string
key="connection.ESTA.SDT.ESTA.DMP"
>DMP on SDT connection identifier</string
><string
key="constant"
>Constant Access Class</string
><string
key="coordinateReference"
>Coordinate Reference</string
><string
key="countdownTime"
>Countdown Time</string
><string
key="currentTarget"
>Current Target</string
><string
key="cyclic"
>Cyclic Measure</string
><string
key="cyclicPath"
>Cyclic Path</string
><string
key="cyclicPath.decreasing"
>Decreasing Cyclic Direction</string
><string
key="cyclicPath.increasing"
>Increasing Cyclic Direction</string
><string
key="cyclicPath.scalar"
>Scalar Path Cyclic Direction</string
><string
key="cyclicPath.shortest"
>Shortest Path Cyclic Direction</string
><string
key="date"
>Date</string
><string
key="date.firmwareRev"
>Firmware Revision Date</string
><string
key="date.manufacture"
>Manufacture Date</string
><string
key="datum"
>Datum</string
><string
key="datumProperty"
>Datum Property</string
><string
key="DCID"
>Device Class Identifier (DCID)</string
><string
key="DDLpropertyRef"
>Reference to a Property defined by DDL</string
><string
key="delayTime"
>Delay Time</string
><string
key="devInfoItem"
>Device Information Item</string
><string
key="devModelName"
>Device Model Name</string
><string
key="devSerialNo"
>Device Serial Number</string
><string
key="deviceDatum"
>Datum Description</string
><string
key="deviceInfoGroup"
>Device Information Group</string
><string
key="deviceRef"
>Device Reference</string
><string
key="deviceSupervisory"
>Device Supervisory Group</string
><string
key="direction"
>Direction</string
><string
key="direction3D"
>Direction in 3 Dimensions</string
><string
key="DMPpropertyAddress"
>The address of a DMP property</string
><string
key="localPropertyAddress"
>The address of a property within the same component</string
><string
key="DMPpropertyRef"
>Reference to a property as defined by DMP</string
><string
key="driven"
>Driven Property</string
><string
key="drivenAnd"
>Property Driven as Logical And of its Drivers</string
><string
key="drivenOr"
>Property Driven as Logical Or of its Drivers</string
><string
key="driver"
>Driver Property</string
><string
key="encoding"
>Encoding</string
><string
key="enumSelector"
>Enumerated Selector Property</string
><string
key="enumeration"
>Enumeration Base Type</string
><string
key="errorReport"
>Error Report</string
><string
key="fractionalSelector"
>Fractional Selector Property</string
><string
key="fullScale"
>Full Scale</string
><string
key="globalDDLPropertyRef"
>Reference to a DDL property within another component</string
><string
key="group"
>Group Property</string
><string
key="initialization.enum"
>Enum Initialization State</string
><string
key="initializationBool"
>Boolean Initialization State</string
><string
key="initializationState"
>Initialization State</string
><string
key="label"
>Label Property</string
><string
key="labelRef"
>Label Reference</string
><string
key="labelString"
>Label String</string
><string
key="languagesetID"
>Language Set Identifier</string
><string
key="length"
>Length</string
><string
key="limit"
>Limit</string
><string
key="limitMaxExc"
>Maximum Exclusive Limit</string
><string
key="limitMaxInc"
>Maximum Inclusive Limit</string
><string
key="limitMinExc"
>Minimum Exclusive Limit</string
><string
key="limitMinInc"
>Minimum Inclusive Limit</string
><string
key="localDatum"
>Local Datum</string
><string
key="localDDLPropertyRef"
>Reference to a DDL property within the same component</string
><string
key="manufacturer"
>Manufacturer</string
><string
key="maunfacturerURL"
>Manufacturer URL</string
><string
key="maxDriven"
>Property Driven as Maximum</string
><string
key="maxDrivenPrioritized"
>Property driven as maximum of highest priority drivers</string
><string
key="measure"
>Measure Behavior</string
><string
key="measureOffset"
>Measure Offset</string
><string
key="minDriven"
>Property Driven as Minimum</string
><string
key="moveRelative"
>Move Relative</string
><string
key="moveTarget"
>Move Target</string
><string
key="multidimensionalGroup"
>Multidimensional Property Group</string
><string
key="namedPropertyRef"
>Reference to a DDL property by name (xml:id)</string
><string
key="nonLinearity"
>Non Linearity</string
><string
key="NULL"
>Null behavior</string
><string
key="ordX"
>X Ordinate</string
><string
key="ordY"
>Y Ordinate</string
><string
key="ordZ"
>Z Ordinate</string
><string
key="ordered"
>Ordered Types</string
><string
key="ordinate"
>Ordinate</string
><string
key="orientation"
>Orientation</string
><string
key="orientation3D"
>Orientation in 3 Dimensions</string
><string
key="orthogonalLength"
>Orthogonal Length</string
><string
key="paramSzArray"
>Parametrically Sized Array</string
><string
key="persistent"
>Persistent Access Class</string
><string
key="point2D"
>Point in 2 Dimensions</string
><string
key="point3D"
>Point in 3 Dimensions</string
><string
key="polarOrdinate"
>Polar Ordinate</string
><string
key="position3D"
>Position in 3 Dimensions</string
><string
key="positionalSelector"
>Positional Selector</string
><string
key="preferredValue"
>Preferred Value</string
><string
key="preferredValue.abstract"
>Abstract Preferred Value</string
><string
key="priority"
>Priority behavior</string
><string
key="progressCounter"
>Progress Counter</string
><string
key="progressIndicator"
>Progress Indicator</string
><string
key="progressTimer"
>Progress Timer</string
><string
key="propertyRef"
>Property Reference</string
><string
key="propertySet"
>Property Set</string
><string
key="propertySetSelector"
>Property Set Selector</string
><string
key="publishEnable"
>Publish Enable</string
><string
key="publishMaxTime"
>Maximum Publish Time</string
><string
key="publishMinTime"
>Minimum Publish Time</string
><string
key="publishParam"
>Publish Parameter</string
><string
key="publishThreshold"
>Publish Threshold</string
><string
key="radialLength"
>Radial Length</string
><string
key="rate"
>Rate</string
><string
key="rate1st"
>First Derivative Rate</string
><string
key="rate1stLimit"
>First Derivative Rate Limit</string
><string
key="rate2nd"
>Second Derivative Rate</string
><string
key="rate2ndLimit"
>Second Derivative Rate Limit</string
><string
key="relativeTarget"
>Relative Target</string
><string
key="repeatPrefVal"
>Repeating Preferred Value</string
><string
key="repeatPrefValOffset"
>Repeating Preferred Value Offset</string
><string
key="scalar"
>Scalar Measure</string
><string
key="scale"
>Scale</string
><string
key="selected"
>Selected Property</string
><string
key="selector"
>Selector Property</string
><string
key="sharedProps"
>Shared Property Container</string
><string
key="spatialCoordinate"
>Spatial Coordinate</string
><string
key="streamFilter"
>Stream Filters</string
><string
key="stringRef"
>String Resource Reference</string
><string
key="suspend"
>Suspend</string
><string
key="systemPropertyAddress"
>The address of a DMP property in another component</string
><string
key="syncGroupMember"
>Synchronization group member</string
><string
key="target"
>Target Property</string
><string
key="targetTimer"
>Target Timer</string
><string
key="textString"
>Text String</string
><string
key="time"
>Time</string
><string
key="timePeriod"
>Time Period</string
><string
key="timePoint"
>Time Point</string
><string
key="trippable"
>Trippable property</string
><string
key="type.NCName"
>“No Colon Name” String</string
><string
key="type.bitmap"
>Bitmap Type</string
><string
key="type.boolean"
>Boolean Type</string
><string
key="type.char"
>Character Type</string
><string
key="type.char.UTF-16"
>UTF-16 Big Endian Encoded Character</string
><string
key="type.char.UTF-32"
>UTF-32 Big Endian Encoded Character</string
><string
key="type.char.UTF-8"
>UTF-8 Encoded Character</string
><string
key="type.enum"
>Common “Enum” Type</string
><string
key="type.enumeration"
>Enumeration Type</string
><string
key="type.fixBinob"
>Fixed Size Binary Object Type</string
><string
key="type.float"
>Common Float Type</string
><string
key="type.floating_point"
>Floating Point Types</string
><string
key="type.integer"
>Integer Types</string
><string
key="type.signed.integer"
>Signed Integer Measure</string
><string
key="type.sint"
>Common “Signed Int” type</string
><string
key="type.string"
>Unicode String</string
><string
key="type.uint"
>Common “Unsigned Int” type</string
><string
key="type.unsigned.integer"
>Unsigned Integer Measure</string
><string
key="type.varBinob"
>Variable Size Binary Object Type</string
><string
key="typingPrimitive"
>Typing Primitive</string
><string
key="URI"
>URI Property</string
><string
key="URL"
>URL Property</string
><string
key="URN"
>URN Property</string
><string
key="UUID"
>Universal Unique Identifier (UUID)</string
><string
key="UACN"
>User Assigned Component Name (UACN see ACN-epi19)</string
><string
key="unattainableAction"
>Unattainable Action</string
><string
key="unitScale"
>Unit Scale</string
><string
key="volatile"
>Volatile Access Class</string
><string
key="xenoPropertyReference"
>Reference to a “property” defined or accessed by another protocol</string
><string
key="softwareVersion"
>Software version</string
><string
key="hardwareVersion"
>Hardware version</string
><string
key="connectedState"
>State of a connection</string
><string
key="autoConnectedState"
>State of a connection – connection automatically determined</string
><string
key="explicitConnectedState"
>State of an explicitly specified connection</string
><string
key="writeConnectedState"
>State of a connection used for writing</string
><string
key="readConnectedState"
>State of a connection used for reading</string
><string
key="autoTrackedConnection"
>Connection tracked by an autoConnectedState property</string
><string
key="trackTargetRef"
>Reference to a track-target property</string
><string
key="binding"
>Property binding</string
><string
key="binder"
>Property binder</string
><string
key="binderRef"
>Reference to a binder property</string
><string
key="bindingState"
>Binding state</string
><string
key="bindingMechanism"
>Binding mechanism</string
><string
key="DMPbinding"
>DMP network binding</string
><string
key="DMPsetPropBinding"
>DMP set property binding</string
><string
key="DMPgetPropBinding"
>DMP get property binding</string
><string
key="DMPeventBinding"
>DMP event binding</string
><string
key="bindingAnchor"
>Binding anchor property</string
><string
key="pushBindingMechanism"
>Push binding mechanism</string
><string
key="pullBindingMechanism"
>Pull binding mechanism</string
><string
key="internalSlaveRef"
>Internal binding slave property reference</string
><string
key="internalMasterRef"
>Internal binding master property reference</string
><string
key="internalBidiRef"
>Internal bidirectional binding property reference</string
><string
key="boundProperty"
>Bound property</string
><string
key="windowProperty"
>Window property</string
><string
key="accessWindow"
>Access-window</string
><string
key="accessMatch"
>Match property enabling an access window to view a window-driven property</string
><string
key="accessEnable"
>Access-window enable</string
><string
key="accessInhibit"
>Access-window inhibit</string
><string
key="dynamicAccessEnable"
>Dynamic access-window matching property</string
><string
key="connectionMatch"
>Dynamic access-match by transport connection</string
><string
key="contextMatchWindow"
>Dynamic context sensitive access-window</string
><string
key="autoAssignContextWindow"
>Dynamic access-window with automatic assignment of window-driven properties</string
><string
key="loadOnAction"
>Value loaded on execution of an action</string
><string
key="actionSpecifier"
>Property specifying an action condition</string
><string
key="actionProperty"
>Property on which actions occur</string
><string
key="propertyActionSpecifier"
>Specification of an action applied to a property</string
><string
key="propertyLoadAction"
>Action condition triggered by property load</string
><string
key="propertyChangeAction"
>Action condition triggered by property value change</string
><string
key="actionState"
>State value required to meet an action condition</string
><string
key="actionStateBefore"
>Required state value before an action</string
><string
key="initializer"
>Initial value of property</string
><string
key="actionStateAfter"
>Required state value after an action</string
><string
key="refInArray"
>Reference within array</string
><string
key="rangeOver"
>Array reference index range specifier</string
><string
key="contextDependent"
>Context dependent value</string
><string
key="controllerContextDependent"
>Controller context dependent property</string
><string
key="connectionContextDependent"
>Connection context dependent</string
><string
key="limitByAccess"
>Limit which differ according to access method</string
><string
key="limitNetWrite"
>Limit on network settable value</string
><string
key="pollInterval"
>Network polling interval</string
><string
key="minPollInterval"
>Minimum polling interval</string
><string
key="maxPollInterval"
>Maximum polling interval</string
><string
key="netInterfaceItem"
>Network configuration item</string
><string
key="netInterface"
>Network interface configuration group</string
><string
key="accessNetInterface"
>Network interface used for device access</string
><string
key="netAddress"
>An address or host identifier used by a network protocol</string
><string
key="myNetAddress"
>An address of this appliance</string
><string
key="routerAddress"
>The address of a network router</string
><string
key="serviceAddress"
>The address of a network service</string
><string
key="netInterfaceRef"
>Reference to network configuration group</string
><string
key="netInterfaceDirection"
>Direction of an interface</string
><string
key="xenoPropRef"
>Reference to a property on a foreign network</string
><string
key="xenoBindingMechanism"
>Xeno or foreign protocol binding mechanism</string
><string
key="netInterfaceIEEE802.3"
>IEEE802.3 (Ethernet) network interface</string
><string
key="netInterfaceIEEE802.11"
>IEEE802.11 (wireless Ethernet or WiFi) network interface</string
><string
key="netAddressIEEE-EUI"
>IEEE extended unique identifier</string
><string
key="netAddressIPv4"
>IPv4 network address</string
><string
key="netAddressIPv6"
>IPv6 network address</string
><string
key="netMask"
>Network mask</string
><string
key="netMaskIPv4"
>Network mask for IPv4 networks</string
><string
key="defaultRouteAddress"
>Address of the default route</string
><string
key="netIfaceIPv4"
>Internet protocol version 4 (IPv4) configuration</string
><string
key="myAddressDHCP"
>My address as assigned by DHCP</string
><string
key="myAddressStatic"
>My address statically assigned</string
><string
key="DHCPserviceAddress"
>The address of a DHCP server</string
><string
key="baseAddressDMX512"
>DMX512 base address</string
><string
key="universeIdDMX512"
>DMX512 Universe identifier</string
><string
key="netInterfaceDMX512"
>DMX512 interface configuration group</string
><string
key="netInterfaceDMX512pair"
>DMX512 physical layer interface</string
><string
key="netDMX512-XLRpri"
>Primary pair of a DMX512 XLR connection</string
><string
key="netDMX512-XLRsec"
>Secondary pair of a DMX512 XLR connection</string
><string
key="netIfaceE1.31"
>E1.31 configuration</string
><string
key="universeIdE1.31"
>E1.31 protocol universe identifier</string
><string
key="slotAddressDMX512"
>DMX512 slot address</string
><string
key="STARTCode"
>DMX512 START code</string
><string
key="DMXpropRef"
>DMX512 property reference</string
><string
key="DMXpropRef-SC0"
>DMX512 property reference for NULL START code type data</string
><string
key="bindingDMXnull"
>Binding to a remote DMX512 property using NULL START code</string
><string
key="bindingDMXalt-refresh"
>Binding to a remote DMX512 property using Alternate START code</string
><string
key="deviceDatum"
>Device datum</string
><string
key="deviceDatumDescription"
>Device datum description</string
><string
key="dimension"
>Dimension (and units) of property</string
><string
key="dim-mass"
>Mass dimension</string
><string
key="dim-length"
>Length dimension</string
><string
key="dim-time"
>Time dimension</string
><string
key="dim-charge"
>Charge dimension</string
><string
key="dim-temp"
>Temperature dimension</string
><string
key="dim-angle"
>Angle dimension</string
><string
key="dim-freq"
>Frequency dimension</string
><string
key="dim-force"
>Force dimension</string
><string
key="dim-energy"
>Energy dimension</string
><string
key="dim-power"
>Power dimension</string
><string
key="length-m"
>Length in meters</string
><string
key="time-s"
>Time in seconds</string
><string
key="freq-Hz"
>Frequency in Hz</string
><string
key="temp-K"
>Temperature in Kelvins</string
><string
key="temp-celsius"
>Temperature in degrees Celsius (°c)</string
><string
key="angle-rad"
>Angle in radians</string
><string
key="angle-deg"
>Angle in degrees (°)</string
><string
key="dimensional-scale"
>Dimensional scale</string
><string
key="prefix-yocto"
>Prefix yocto</string
><string
key="prefix-zepto"
>Prefix zepto</string
><string
key="prefix-atto"
>Prefix atto</string
><string
key="prefix-femto"
>Prefix femto</string
><string
key="prefix-pico"
>Prefix pico</string
><string
key="prefix-nano"
>Prefix nano</string
><string
key="prefix-micro"
>Prefix micro</string
><string
key="prefix-milli"
>Prefix milli</string
><string
key="prefix-kilo"
>Prefix kilo</string
><string
key="prefix-mega"
>Prefix mega</string
><string
key="prefix-giga"
>Prefix giga</string
><string
key="prefix-tera"
>Prefix tera</string
><string
key="prefix-peta"
>Prefix peta</string
><string
key="prefix-exa"
>Prefix exa</string
><string
key="prefix-zetta"
>Prefix zetta</string
><string
key="prefix-yotta"
>Prefix yotta</string
><string
key="mass-g"
>Mass in grams (g)</string
><string
key="charge-C"
>Charge in coulombs (C)</string
><string
key="dim-area"
>Area dimension</string
><string
key="area-sq-m"
>Area in square meters (m^2)</string
><string
key="dim-volume"
>Volume dimension</string
><string
key="volume-cu-m"
>Volume in cubic meters (m^3)</string
><string
key="volume-L"
>Volume in liters (L)</string
><string
key="force-N"
>Force in newtons (N)</string
><string
key="energy-J"
>Energy in joules (J)</string
><string
key="power-W"
>Power in watts (W)</string
><string
key="dim-pressure"
>Pressure dimension</string
><string
key="pressure-Pa"
>Pressure in pascals (Pa)</string
><string
key="dim-current"
>Electric current dimension</string
><string
key="current-A"
>Current in amps (A)</string
><string
key="dim-voltage"
>Electromotive force dimension</string
><string
key="voltage-V"
>Voltage or emf in volts (V)</string
><string
key="dim-resistance"
>Electric resistance dimension</string
><string
key="resistance-ohm"
>Resistance in ohms (Ω)</string
><string
key="dim-torque"
>Torque dimension</string
><string
key="torque-Nm"
>Torque in newton meters (Nm)</string
><string
key="dim-luminous-intensity"
>Luminous intensity dimension</string
><string
key="luminous-intensity-cd"
>Luminous intensity in candela (cd)</string
><string
key="dim-luminous-flux"
>Luminous flux dimension</string
><string
key="luminous-flux-lm"
>Luminous flux in lumens (lm)</string
><string
key="dim-illuminance"
>Illuminance dimension</string
><string
key="illuminance-lx"
>Illuminance in lux (lx)</string
><string
key="dim-solid-angle"
>Solid angle dimension</string
><string
key="solid-angle-sr"
>Solic angle in steradians (sr)</string
><string
key="perceptual-dimension"
>Perceptually weighted dimension</string
><string
key="ratio"
>Ratio</string
><string
key="logratio"
>Logarithmic ratio</string
><string
key="logunit"
>Logarithmic units</string
><string
key="power-dBmW"
>Power in dBmW</string
><string
key="scalable-nonLinearity"
>Scalable nonlinear property value</string
><string
key="normalized-nonlinearity"
>Normalized nonlinear function between bounds</string
><string
key="nonlin-log"
>Logarithmic property</string
><string
key="nonlin-log10"
>Logarithm base 10 property</string
><string
key="nonlin-ln"
>Natural logarithm property</string
><string
key="nonlin-squareLaw"
>Generic “square-law” response curve</string
><string
key="normalized-square-law"
>Normalized “square-law” response curve</string
><string
key="nonlin-S-curve"
>Generic “s-curve” response curve</string
><string
key="nonlin-S-curve-precise"
>Precise “s-curve” response curve</string
><string
key="nonlin-monotonic"
>Monotonic nonlinearity</string
><string
key="normalized-monotonic"
>Monotonic normalized nonlinearity</string
><string
key="streamInput"
>Input to a stream processing group from elsewhere</string
><string
key="streamOuput"
>An output point from a stream processing group</string
><string
key="streamCoverter"
>Property creating a new stream from one or more others</string
><string
key="streamFilter"
>Property changing the character of a stream</string
><string
key="streamGovernor"
>Property governing the “size” of a stream</string
><string
key="beamSource"
>A stream source with physical direction</string
><string
key="beamDiverter"
>Beam diverter</string
><string
key="streamGroup"
>Stream processing group</string
><string
key="streamPoint"
>Stream point</string
><string
key="streamRatio"
>Stream ratio</string
><string
key="lightSource"
>Light source</string
><string
key="opticalLens"
>Lens</string
><string
key="DHCPLeaseTime"
>DHCP lease time</string
><string
key="DHCPLeaseRemaining"
>DHCP lease remaining</string
><string
key="netInterfaceState"
>Network interface initialization state</string
><string
key="myAddressLinkLocal"
>My link-local address as assigned by zeroconf</string
><string
key="DHCPclientState"
>State of DHCP client algorithm</string
><string
key="netNetworkAddress"
>Address of a network (subnet address)</string
><string
key="netHostAddress"
>Host part of a network address</string
><string
key="simplified-specialized"
>Simplified behavior for specialized application</string
></language
><language
altlang="en-US"
lang="en-GB"
><string
key="algorithm"
>Algorithm Behaviour</string
><string
key="behaviorRef"
>Behaviour Reference</string
><string
key="behaviorsetID"
>Behaviour Set Identifier</string
><string
key="colorFilter"
>Colour Filter</string
><string
key="colorSpec"
>Colour Specifier</string
><string
key="initialization.enum"
>Enum Initialisation State</string
><string
key="initializationBool"
>Boolean Initialisation State</string
><string
key="initializationState"
>Initialisation State</string
><string
key="measure"
>Measure Behaviour</string
><string
key="NULL"
>Null behaviour</string
><string
key="priority"
>Priority behaviour</string
><string
key="length-m"
>Length in metres</string
><string
key="area-sq-m"
>Area in square metres (m^2)</string
><string
key="volume-cu-m"
>Volume in cubic metres (m^3)</string
><string
key="volume-L"
>Volume in litres (L)</string
><string
key="torque-Nm"
>Torque in newton metres (Nm)</string
><string
key="netInterfaceState"
>Network interface initialisation state</string
><string
key="simplified-specialized"
>Simplified behaviour for specialised application</string
></language
></languageset
></DDL
>
