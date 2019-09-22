<?xml version="1.0" encoding="UTF-8"?>
<DDL
version="1.1"
xml:id="acnbase.bset.DDL"
><!--
$Id$
--><behaviorset
UUID="71576eac-e94a-11dc-b664-0017316c497d"
date="2010-09-03"
provider="http://www.esta.org/ddl/acn-core/"
xml:id="acnbase.bset"
><UUIDname
UUID="71576eac-e94a-11dc-b664-0017316c497d"
name="acnbase.bset"
></UUIDname
><UUIDname
UUID="edce0400-e940-11dc-b7aa-0017316c497d"
name="acnbase.lset"
></UUIDname
><label
key="acnbase.bset"
set="acnbase.lset"
></label
><!--Behaviors:

This behaviorset is an updated replacement for behaviorset 
a713a314-a14d-11d9-9f34-000d613667e2.
Many behaviors are copied directly, but a number of errors 
and omissions are corrected; additonal behaviors are added 
to model common situations; and some behaviors have been 
superseded by ones which model devices more clearly, 
efficiently or consistently.--><!--NULL behavior--><behaviordef
name="NULL"
><label
key="NULL"
set="acnbase.lset"
></label
><section
><hd
>NULL behavior</hd
><p
>This behavior carries no semantic meaning but may serve as the root for other behaviors. NULL behavior can be added to any property without affecting its meaning or other behaviors.</p
><p
>Similarly, any behaviordef may refine NULL behavior without affecting its meaning.</p
><section
><hd
>Note: DDL-2006 specification</hd
><p
>The DDL 2006 specification states in section 5.5:</p
><section
><p
>“Every behavior is a refinement of some more generic of abstract behavior – see refines. The one exception is the behavior NULL whose UUID is ‘00000000-0000-0000-0000-000000000000’.”</p
></section
><p
>This requirement is an error and cannot be followed because a behaviordef itself has no UUID (a behavior has a name and its behaviorset has a UUID). This behavior definition is provided so that there is a valid NULL behavior to which refines and behavior elements may refer.</p
></section
><section
><hd
>Note: Use in DDL v1.1</hd
><p
>Following the DDL revision to version 1.1, the requirement that all behaviordef must carry a refines element has been removed and there is no need for a NULL behavior definition for this purpose. However, this behavior has other uses, as well as for legacy and transitional compatibility.</p
></section
></section
></behaviordef
><!--Behaviors for fundamental decoding of property values - 
types and encodings. First a basic set of typing concepts, 
then common encodings for those types.--><behaviordef
name="typingPrimitive"
><label
key="typingPrimitive"
set="acnbase.lset"
></label
><section
><hd
>Typing Primitive</hd
><p
>An abstract behavior from which some basic typing concepts expressing the interpretation of values derive.</p
></section
></behaviordef
><behaviordef
name="group"
><label
key="group"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Group Property</hd
><p
>The property has no value at all. It exists to group sub-properties.</p
></section
></behaviordef
><behaviordef
name="ordered"
><label
key="ordered"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Ordered Types</hd
><p
>This behavior indicates that the property contains some sort of inherent ordering. For an ordered property the relationships “precedes” and “succeeds” and for discrete types (not floating point) “next” and “previous” have meaning.</p
><p
>Measures (see measure behavior) are implicitly ordered, however strings, bitmaps and enumerations are not necessarily so. Adding ordered behavior to an enumeration for example indicates that the states of the enumeration follow each other in a natural order – because progression from case A to case C will go via state B – e.g. (e.g. “uninitialized”, “initializing”, “initialized”). Note that the fact that states of an enumeration are represented by an binary integer does not in itself imply ordering.</p
></section
></behaviordef
><behaviordef
name="measure"
><label
key="measure"
set="acnbase.lset"
></label
><refines
name="ordered"
set="acnbase.bset"
></refines
><section
><hd
>Measure Behavior</hd
><p
>This is the base behavior from which any property whose value represents an analog of a measurable quantity or position should derive. This includes anything from temperature to phase angle.</p
><p
>Since properties may only have a single value, a measure is always one dimensional – multidimensional features are described by grouping properties together (see multidimensionalGroup behavior).</p
><p
>Single dimensional properties may be closed (e.g. see cyclic behavior) or open (e.g. see scalar behavior).</p
><p
>Measure properties are expressed as numbers. DMP encodings are normally binary integers or floating point values although some special cases may vary (e.g. dates expressed as numeric strings).</p
><section
><hd
>Default types of measure</hd
><p
>Behaviors refined from measure shall be assumed to be scalar (see scalar behavior) unless some other kind is explicitly specified (e.g. cyclic).</p
></section
></section
></behaviordef
><behaviordef
name="scalar"
><label
key="scalar"
set="acnbase.lset"
></label
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Scalar Measure</hd
><p
>The property is to be interpreted as a one dimensional scalar measure representing a position on a continuum. Magnitude comparisons are meaningful on scalars, logical operations are not. Scalar properties are commonly represented as integers (signed or unsigned) or floating point numbers but other encodings are possible.</p
><p
>Scalar quantities larger than one octet are usually subject to endian conversion.</p
></section
></behaviordef
><behaviordef
name="cyclic"
><label
key="cyclic"
set="acnbase.lset"
></label
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Cyclic Measure</hd
><p
>A cyclic measure is one representing a feature or function such as the position of a turntable or a compass bearing, whose value is not constrained at its limits but “wraps around” to an opposing limit. Cyclic values must often be treated in a different way from scalar properties because although they are single dimensional they have two different ways to change between two points.</p
><section
><hd
>Example</hd
><p
>A ship’s bearing is represented by a property whose units are degrees. If this property were declared as a normal linear scalar, then in order to turn from 315° (NW) to 45° (NE) the ship would need to turn anticlockwise via west then south. However, if bearing is declared as a cyclic property, a controller can be aware that it has two different ways to reach NE from NW. Namely clockwise or counter-clockwise.</p
></section
><section
><hd
>Use of Limits With Cyclic Properties</hd
><p
>Attached to cyclic properties, limits define the range that the property may take but do not represent “stops”. On reaching it's upper (inclusive) limit, a the property may advance to its lower (inclusive) limit in a single increment. It is preferred to declare one limit – usually the lower one – inclusive and the other – usually upper – exclusive. The exclusive and inclusive limit values then represent exactly the same position.</p
><section
><hd
>Limit examples</hd
><p
>In the bearing example above, the bearing property has a limitMinInc of 0° and a limitMaxExc of 360°. These two values both represent the same position (due north).</p
><p
>An example where the inclusive and exclusive limits might be reversed is the hour hand of a 12-hour clock whose limitMinExc would naturally be 0 and limitMaxInc 12 (0 and 12 represent the same hour but permissible values range 1—12 rather than 0—11). However, this situation only applies rarely and does not apply for example in the 24-hour case where hours range 0—23.</p
></section
></section
><section
><hd
>Control of Cyclic Properties - Cyclic Paths, Relative Moves</hd
><p
>If a cyclic property does not simply change instantly from one value to another but needs to progress through intermediate values – either because of its mechanical configuration or because of other constraints such as rate of change restrictions – then a controller may need to know or control the algorithm for doing this. cyclicPath behavior and its refinements allow these algorithms to be declared. see cyclicPath</p
><p
>Another means to control cyclic properties across multiple cycles is by relative movement. See relative.target</p
></section
><section
><hd
>Multi-cycle properties</hd
><p
>The limits on the value of a cyclic property may actually allow more than a single cycle of the property value. For example, the compass bearing may have limits of 0° and 3600° allowing ten full cycles to be represented by the value before “wrapping” back to the opposite extreme. This usage is discouraged because it brings no additional functionality. See “cyclesize” behavior for further discussion.</p
></section
></section
></behaviordef
><behaviordef
name="reference"
><label
key="reference"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Reference or Pointer</hd
><p
>A reference a property which points or refers to something else. This is an abstract behavior which is refined by many others such as property references, component references and so on. While reference is a refinement of typingPrimitive, many implementations will also refine an additional type such as integer (e.g. an address) or NCName (a lookup).</p
></section
></behaviordef
><behaviordef
name="bitmap"
><label
key="bitmap"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Bitmap</hd
><p
>A bitmap property is collection of bits each of which represents a different item or condition. Logical operations are meaningful on bitmaps, while magnitude comparisons and arithmetic operations are not. The bits in a bitmap have a weight or importance which is independent of their position in the bitmap and which is equal to all other bits in the bitmap unless explicitly stated otherwise in a refinement of this behavior.</p
></section
></behaviordef
><behaviordef
name="binObject"
><label
key="binObject"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Binary Object</hd
><p
>A binary object is an opaque data object it may be of fixed or variable size. The only operations defined on binary objects are copying and comparison. Comparison of two objects is a strict bit for bit comparison in which the size and every bit must match for the two objects to be equal.</p
><p
>Refinements may define more structure or specific operations.</p
></section
></behaviordef
><behaviordef
name="enumeration"
><label
key="enumeration"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Enumeration Base Type</hd
><p
>A set of alternative states or cases each represented by a symbol.</p
><section
><hd
>Ordering</hd
><p
>An enumeration may or may not be ordered. If not then all cases in the enumeration are considered to have equal weight and their sequence is unimportant. If an enumeration is ordered then the operations “next” and “previous” can be applied and the relationships “precedes” or “succeedes” are meaningful – see ordered behavior.</p
></section
></section
></behaviordef
><behaviordef
name="boolean"
><label
key="boolean"
set="acnbase.lset"
></label
><refines
name="enumeration"
set="acnbase.bset"
></refines
><section
><hd
>Boolean</hd
><p
>The property has two states or cases: true or false.</p
></section
></behaviordef
><behaviordef
name="character"
><label
key="character"
set="acnbase.lset"
></label
><refines
name="enumeration"
set="acnbase.bset"
></refines
><section
><hd
>Character</hd
><p
>A character property represents a single text character. It is a refinement of enumeration in which a set of possible characters is enumerated.</p
></section
></behaviordef
><behaviordef
name="textString"
><label
key="textString"
set="acnbase.lset"
></label
><refines
name="typingPrimitive"
set="acnbase.bset"
></refines
><section
><hd
>Text String</hd
><p
>The property represents a string of characters. A text string may be of fixed length but is frequently of variable length both in the number of characters contained and in the number of octets required to encode them. Refinements may define particular encodings or restrictions.</p
></section
></behaviordef
><!--Encoded types--><behaviordef
name="encoding"
><label
key="encoding"
set="acnbase.lset"
></label
><section
><hd
>Encoding</hd
><p
>This forms the root for a set of behaviors expressing the basic data representations for DMP properties. The combination of a typing primitive and an encoding description produces a basic encoded type.</p
><p
>While representations internal to the processor handling the data may be common across different data types (e.g. a bitmap and a measure are both commonly represented by an "integer"), for DMP descriptions they shall always be kept separate.</p
><p
>Primary refinements of encodings may also be abstract but specific data encodings are refined from them.</p
><p
>DMP transfer ensures that binary values of the correct size are transferred, but in the absence of further information an application can store but can not manipulate the data in any way – for example, an value representing an integer may need byte-swapping, while a value representing a floating point number would need other treatment. Refinements of this behavior are intended to provide a set of common data encodings.</p
><p
>Almost any property should carry some sort of encoding behavior.</p
></section
></behaviordef
><!--Encodings of Measures--><behaviordef
name="type.integer"
><label
key="type.integer"
set="acnbase.lset"
></label
><refines
name="measure"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Integer Types</hd
><p
>The property is a measure encoded as a binary integer.</p
><p
>Integer values shall be transferred using DMP in network byte order (most significant byte first). In most implementations it must be converted to native byte order before processing.</p
><p
>Any references to bit numbers in values of this type shall refer to the least significant bit of the resulting integer as bit-0 and number bits of increasing significance sequentially to the most significant.</p
><section
><hd
>Bit numbering example</hd
><p
>In an integer with size 4 (32 bits), bit 12 is the 4th from least significant bit of the 3rd octet "on the wire":</p
><p
xml:space="preserve"
>|  7   6   5   4   3   2   1   0   |
| b31  -   -   -   -   -   -  b24  |  First (most significant) octet
| b23  -   -   -   -   -   -  b16  |  second octet
| b15  -   -  b12  -   -   -   b8  |  third octet
|  b7  -   -   -   -   -   -   b0  |  fourth (least significant) octet</p
><p
>Bit 0 is the least significant bit and bit 31 is the most significant.</p
></section
><section
><hd
>Types of measure</hd
><p
>Integers shall be assumed to be scalar measures unless some other kind is explicitly specified (e.g. cyclic).</p
></section
></section
></behaviordef
><behaviordef
name="type.unsigned.integer"
><label
key="type.unsigned.integer"
set="acnbase.lset"
></label
><refines
name="type.integer"
set="acnbase.bset"
></refines
><section
><hd
>Unsigned Integer Measure</hd
><p
>The property is a meassure encoded as an unsigned binary integer.</p
><p
>Encoding is identical to type.integer. The value of the integer is interpreted using unsigned binary representation and may range from zero to 2^N−1 where N is the size of the property in bits. Note. that other more restrictive value limits may apply to particular property values.</p
></section
></behaviordef
><behaviordef
name="type.signed.integer"
><label
key="type.signed.integer"
set="acnbase.lset"
></label
><refines
name="type.integer"
set="acnbase.bset"
></refines
><section
><hd
>Signed Integer Measure</hd
><p
>The property is a meassure encoded as a signed binary integer.</p
><p
>Encoding is identical to type.integer. The value of the integer is interpreted using two's complement representation and may range from −(2^(N−1)) to 2^(N−1)−1 where N is the size of the property in bits. Note. that other more restrictive value limits may apply to particular property values.</p
></section
></behaviordef
><behaviordef
name="type.uint"
><label
key="type.uint"
set="acnbase.lset"
></label
><refines
name="type.unsigned.integer"
set="acnbase.bset"
></refines
><section
><hd
>Common “Unsigned Int” type</hd
><p
>Encoding and interpretation are identical to type.unsigned.integer. The size of the property shall be 1, 2, 4 or 8 octets.</p
></section
></behaviordef
><behaviordef
name="type.sint"
><label
key="type.sint"
set="acnbase.lset"
></label
><refines
name="type.signed.integer"
set="acnbase.bset"
></refines
><section
><hd
>Common “Signed Int” type</hd
><p
>Encoding and interpretation are identical to type.signed.integer. The size of the property shall be 1, 2, 4 or 8 octets.</p
></section
></behaviordef
><behaviordef
name="type.floating_point"
><label
key="type.floating_point"
set="acnbase.lset"
></label
><refines
name="measure"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Floating Point Types</hd
><p
>The property is a measure encoded using a floating point notation. It may require translation into a native floating point format before manipulation. This is an abstract type since it does not specify any particular format.</p
></section
></behaviordef
><behaviordef
name="type.float"
><label
key="type.float"
set="acnbase.lset"
></label
><refines
name="type.floating_point"
set="acnbase.bset"
></refines
><section
><hd
>Common Float Type</hd
><p
>The property is encoded using IEEE floating point format as defined in [IEEE 754-1985]. Size shall be 4 or 8 octets. Values shall be transmitted in network byte order.</p
></section
></behaviordef
><!--Encodings of Enumerations--><behaviordef
name="type.enumeration"
><label
key="type.enumeration"
set="acnbase.lset"
></label
><refines
name="enumeration"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Enumeration Type</hd
><p
>An enumeration property in which each enumerated case is represented by an integer “symbol”. The encoding shall be identical to type.unsigned.integer with each case enumerated being identified by a different value. Values are treated as symbols representing enumerated cases rather than being measures.</p
><section
><hd
>Ordering</hd
><p
>In an ordered enumeration the numeric ordering of the symbolic values assigned to each state shall correspond strictly to the ordering of the enumeration such that operations “next” or “previous” applied to those symbols as numbers shall generate the correct resulting symbols for the cases they represent and relationships “precedes” or “succeedes” applied to the numeric values of the symbols shall have the same result as the same relationships betaeen the cases represented.</p
></section
><section
><hd
>Permissible States</hd
><p
>Whether or not an enumeration is ordered, it may have minimum and maximum limits set (using limits behavior – see below) which define the range of binary states it is permitted to take. This use of arithmetic comparison to define the range of states does not imply any order or precedence within that range.</p
><p
>The set of permissible states may also be identified by a set of sub-properties or sibling properties – for an example see “selected” and “selector” behaviors. In these cases the number of sub-properties in the set defines the number of states of the enumeration. Enumerated properties used in this way may either take the form of an array or may be listed individually in sequence.</p
><section
><hd
>Assignment of States</hd
><p
>If the behavior does not explicitly define how states are associated with numeric values of the enumeration parent then they shall be assigned numeric values sequentially in the order they appear starting from zero.</p
></section
></section
></section
></behaviordef
><behaviordef
name="type.enum"
><label
key="type.enum"
set="acnbase.lset"
></label
><refines
name="type.enumeration"
set="acnbase.bset"
></refines
><section
><hd
>Common “Enum” Type</hd
><p
>Encoding and interpretation are identical to type.enumeration. The size of the property shall be 1, 2, 4 or 8 octets.</p
></section
></behaviordef
><behaviordef
name="type.boolean"
><label
key="type.boolean"
set="acnbase.lset"
></label
><refines
name="boolean"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Boolean Type</hd
><p
>A boolean property in which the states true and false are represented by a binary value. A value of zero represents false and any non-zero value represents true.</p
><p
>All non-zero values are to be interpreted as directly equivalent by receivers, but the preferred value for true to be sent by transmitters is 1. A boolean property may be declared any size, but one octet is sufficient.</p
></section
></behaviordef
><!--Encodings of Bitmaps--><behaviordef
name="type.bitmap"
><label
key="type.bitmap"
set="acnbase.lset"
></label
><refines
name="bitmap"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Bitmap Type</hd
><p
>The property is a bitmap as defined by bitmap behavior. It has a fixed number of bits.</p
><p
>The meaning and function of the bits in the bitmap is not defined here but may be defined by refinements of this behavior.</p
><p
>Because bits within a bitmap are all equally significant and arithmetic operations and magnitude comparisons are not appropriate, it is implementation choice whether to byte-swap before processing or not. Implementations shall ensure that bit order is maintained between reception and retransmission of bitmap values.</p
><p
>Descriptions of meaning and function of bitmap types shall always describe them in the order in which they are encoded “on the wire” in DMP messages.</p
><section
><hd
>Bit Identification</hd
><p
>To ensure identification and preservation of bit positions within bitmaps, behaviors and other descriptions governing bitmap values shall ensure that order in multi-octet values is clearly identified. The following terminology is provided for reference and unless a derived behavior clearly and unambiguously identifies other terms or conventions, these shall apply.</p
><p
>• Bits shall be identified by the octet in which they appear and by the bit position within that octet. Never by their position in some higher order word.</p
><p
>• All references use the order that the octets appear in DMP packets with the first octet in the property value being octet-0.</p
><p
>• Individual bits are numbered within an octet from zero to seven, with zero being the bit stored in the least significant position of holding registers. (Actual “on the wire” order of bits within an octet varies with different physical or link layer protocols.)</p
><p
>• Bits numbers greater than seven shall not be used.</p
></section
></section
></behaviordef
><behaviordef
name="type.fixBinob"
><label
key="type.fixBinob"
set="acnbase.lset"
></label
><refines
name="binObject"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Fixed Size Binary Object Type</hd
><p
>A fixed size binary object is transmitted as a sequence of octets whose number is given by the associated protocol element. The definition of the object shall always unambiguously specify the order in which octets are to be transmitted.</p
></section
></behaviordef
><behaviordef
name="type.varBinob"
><label
key="type.varBinob"
set="acnbase.lset"
></label
><refines
name="binObject"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Variable Size Binary Object Type</hd
><p
>A variable size binary object is transmitted as a sequence of octets using DMPs standard method for transferring variable length properties. The definition of the object shall always unambiguously specify the order in which octets are to be transmitted.</p
></section
></behaviordef
><!--Encodings of Characters and Strings--><behaviordef
name="type.character"
><label
key="type.char"
set="acnbase.lset"
></label
><refines
name="character"
set="acnbase.bset"
></refines
><refines
name="encoding"
set="acnbase.bset"
></refines
><section
><hd
>Character Type</hd
><p
>The set of characters used in DMP is the Unicode character set. Encodings recognised are UTF-8, UTF-16 and UTF-32.</p
><p
>This property represents a single character and shall have a fixed property length of 1, 2, 4 or 8 octets. The size of the property is not entirely dictated by the encoding used.</p
><p
>Because many unicode character encodings are of variable length the range of unicode characters expressible may be restricted by the size of the property. In all encodings four octets are required to represent all possible Unicode code points, while if combining marks are to be used to generate characters (e.g. “G acute – Ǵ ” U+0047, U+0301) even more octets may be required.</p
></section
></behaviordef
><behaviordef
name="type.char.UTF-8"
><label
key="type.char.UTF-8"
set="acnbase.lset"
></label
><refines
name="type.character"
set="acnbase.bset"
></refines
><section
><hd
>UTF-8 Encoded Character</hd
><p
>The property represents a single Unicode character encoded using UTF-8 encoding.</p
></section
></behaviordef
><behaviordef
name="type.char.UTF-16"
><label
key="type.char.UTF-16"
set="acnbase.lset"
></label
><refines
name="type.character"
set="acnbase.bset"
></refines
><section
><hd
>UTF-16 Big Endian Encoded Character</hd
><p
>The property represents a single Unicode character encoded using UTF-16BE encoding.</p
><p
>Since big endian encoding is the same as network byte order” and this is the standard ordering within DMP (and all of ACN) there is no equivalent UTF-16LE type.</p
></section
></behaviordef
><behaviordef
name="type.char.UTF-32"
><label
key="type.char.UTF-32"
set="acnbase.lset"
></label
><refines
name="type.character"
set="acnbase.bset"
></refines
><section
><hd
>UTF-32 Big Endian Encoded Character</hd
><p
>The property represents a single Unicode character encoded using UTF-32BE encoding.</p
><p
>Since big endian encoding is the same as network byte order” and this is the standard ordering within DMP (and all of ACN) there is no equivalent UTF-32LE type.</p
></section
></behaviordef
><behaviordef
name="type.string"
><label
key="type.string"
set="acnbase.lset"
></label
><refines
name="textString"
set="acnbase.bset"
></refines
><section
><hd
>Unicode String</hd
><p
>The property is a string of Unicode characters encoded using one of the standard Unicode encodings.</p
><section
><hd
>Encoding of Unicode Strings</hd
><p
>For immediate values (given in a &lt;value&gt; attribute) the encoding is determined by the normal rules of XML.</p
><p
>For DMP accessed values, the encoding may be UTF-8 or UTF-16. UTF-16 encoded strings shall always begin with a byte order mark (U+FEFF) which indicates both the byte order and the use of UTF-16 encoding. If no byte order mark is present, the encoding shall be UTF-8.</p
><p
>The length of the string shall be encoded by DMP using its standard length encoding method for variable length properties.</p
></section
><p
>For more information on Unicode strings and UTF-8 or UTF-16 encodings please see http://www.unicode.org</p
></section
></behaviordef
><behaviordef
name="type.NCName"
><label
key="type.NCName"
set="acnbase.lset"
></label
><refines
name="type.string"
set="acnbase.bset"
></refines
><section
><hd
>“No Colon Name” String</hd
><p
>The property is a string which matches the XML Schema NCName datatype. NCNames are used for naming XML elements and as identifiers within XML. DDL uses NCName type for naming a number of items which properties may reference including behavior definitions, string resource keys and property identifiers.</p
><p
>NCName properties are generally used as keys for matching XML elements and attributes and are intended for lexical matching. NCNames are unicode strings and match a target name when each and every Unicode code character matches in order. They are sensitive to case and other code-point variations in between different character representations but must be independent of the encoding used – a property value encoded in UTF-8 must be correctly matched to names in a description which happens to be encoded in UTF-16.</p
></section
></behaviordef
><behaviordef
name="stringRef"
><label
key="stringRef"
set="acnbase.lset"
></label
><refines
name="type.NCName"
set="acnbase.bset"
></refines
><refines
name="reference"
set="acnbase.bset"
></refines
><section
><hd
>String Resource Reference</hd
><p
>The use of languages and languagesets in DDL allows collections of string resources to be managed with multilingual substitution capabilities. A stringRef property allows these string resources to be referenced from property values.</p
><p
>The value of a stringRef property is a string which must match the key attribute of a string in one or more languages in a languageset. The referenced string subjected to language selection in the same way as for label elements in DDL.</p
><p
>The string key is a unicode character sequence conforming to the XML “NCName” definition. Unless the encoding of the string value of this property exactly matches that of the XML document containing the languageset, a straightforward byte-for-byte comparison of the property value with the key will not produce a correct match.</p
><p
>Any stringRef property shall include a languagesetID child property which identifies the languageset to be searched for the replacement value.</p
></section
></behaviordef
><!--Extended information on property access--><behaviordef
name="accessClass"
><label
key="accessClass"
set="acnbase.lset"
></label
><section
><hd
>Access Classes</hd
><p
>Abstract behavior refinements of which provide information on the access characteristics of a network accessible property.</p
></section
></behaviordef
><behaviordef
name="persistent"
><label
key="persistent"
set="acnbase.lset"
></label
><refines
name="accessClass"
set="acnbase.bset"
></refines
><section
><hd
>Persistent Access Class</hd
><p
>The property value is stored in persistent storage. Controllers should not expect to initialize persistent values each time they establish control over a device. Persistent properties may nevertheless be volatile – other processes may change the value, for example a configuration option which is persistent may be changeable via DMP or via some other method (html configuration page, front panel etc.).</p
></section
></behaviordef
><behaviordef
name="volatile"
><label
key="volatile"
set="acnbase.lset"
></label
><refines
name="accessClass"
set="acnbase.bset"
></refines
><section
><hd
>Volatile Access Class</hd
><p
>The property value is subject to change by processes or actions independent of DMP access. Examples of volatile properties include those changed by manual intervention (e.g. a switch or knob input, or front panel control), anything measuring environmental or external processes (e.g. temperature, liquid level) or those controlled via a separate and independent network or datalink (e.g. by a user via a browser interface).</p
><p
>Properties whose values change only as a result of DMP actions are not considered volatile even if those actions are indirect. Other behaviors such as “driven” may describe such indirect actions. Direct changing of properties by another DMP controller is also possible on any writable property and is not reason to declare it volatile.</p
><p
>Volatile properties may also be persistent – see “persistent” behavior.</p
></section
></behaviordef
><behaviordef
name="constant"
><label
key="constant"
set="acnbase.lset"
></label
><refines
name="accessClass"
set="acnbase.bset"
></refines
><section
><hd
>Constant Access Class</hd
><p
>A property with constant behavior is one whose value does not change throughout the lifetime of the component. Its value is “burned in”. By definition it cannot have write access. Some examples of constant properties are device serial number, manufacturer name or physical limits.</p
></section
></behaviordef
><!--Properties which are loaded as a group--><behaviordef
name="accessOrder"
><label
key="accessOrder"
set="acnbase.lset"
></label
><section
><hd
>Access-Order Sensitivity</hd
><p
>DMP and other ACN protocols go to some trouble to ensure ordered processing of messages. However by default, properties in DMP are assumed to be independent of each other and may be read or written in any order – the only difference arising from the small difference timing due to processing sequence differences. Implementations are possible which take advantage of this and may re-arrange the order of access to properties, for example to optimise range addressing at the transmitting end or to optimise the sequence of processing at the receiving end.</p
><p
>In most cases it is preferable to define behavior so that access order sensitivity does not occur but in some cases different results will be produced if the order of access is changed for two or more interdependent properties. in extreme cases it may be impossible to access some of them at all.</p
><p
>accessOrder behavior flags this characteristic and specific refinements define how property access ordering works.</p
><p
>Sensitivity to access order varies. In some cases, a property cannot be accessed at all unless some prior action has been taken, in other cases it may be accessed but may operate differently, or may only work if another property is accessed afterwards. In general if the sequence set_property A, set_property B generates different results from the sequence set_property B, set_property A, then there is an order sensitivity.</p
><section
><hd
>Example</hd
><p
>Consider a "fade timer" property of a lighting dimmer, which is set in conjunction with a target value (see targetTimer behavior). The timer value defines the time taken for the dimmer to fade to the target value and its current value is “latched” every time the target level is written. This could be used as follows:</p
><section
><p
>1. set timer to 2 seconds</p
><p
>2. set target to 45 (dimmer now takes 2 seconds to fade to level 45)</p
><p
>3. set target to 86 (dimmer takes another 2 seconds to reach level 86)</p
><p
>4. set timer to 12 seconds</p
><p
>5. set target to 0 (dimmer fades to 0 over 12 seconds)</p
></section
><p
>If timer and target are set in the same packet, then re-ordering to optimise accesses could have drastic effects. for example if steps 4 and 5 were reversed, the final fade would take 2 seconds instead of 12.</p
><p
>In this instance, either timer or target can be accessed separately, however, if both are set together, timer must be set before target to have the desired effect. Also changes to timer have no effect on the driven property (the dimmer level) unless target is set.</p
><p
>In this example given, application which fully understands the behavior of the timer “knows” that it is order sensitive and could in principle set a flag to the DMP and lower layers to preserve its ordering. However, an application which does not fully understand the timer behavior could benefit from the knowledge that its interaction with the target level is order sensitive. Whether or not the behavior is fully understood, the controller can also use this information to signal to the lower layers that they must preserve order without having to have special case code for each order-dependent behavior.</p
></section
><section
><hd
>Note: Time of processing</hd
><p
>The difference in time of processing alone is not sufficient to create an order sensitivity unless the processes associated with the properties are extremely time sensitive. This is characterised by considering whether the change in timing caused by re-ordering two accesses which are both contained within the same packet could be significant.</p
><section
><hd
>Example</hd
><p
>For example, two properties, Target and Speed (rate1stLimit) used to drive a parent property in a controlled ramp. When used to model a mechanical motion whose response time is measured in tens or hundreds of milliseconds these properties are not order sensitive and setting (Target, Speed) is no different from setting (Speed, Target) within a single packet. However, when Speed and Target are controlling a software generated ramp which has effectively infinite acceleration and a maximum value for Speed which allows a full scale change of the driven property in microseconds, then if Target is set before Speed, the driven property could have completed the change before the Speed message is processed.</p
></section
></section
></section
></behaviordef
><behaviordef
name="atomicLoad"
><label
key="atomicLoad"
set="acnbase.lset"
></label
><refines
name="accessOrder"
set="acnbase.bset"
></refines
><section
><hd
>atomicLoad</hd
><p
>This abstract behavior indicates that multiple properties form a group whose values can be set atomically by the setting of a master property. The values of all the other properties within the group may be changed in arbitrary ways, but their values only take effect when the master property is set and do so in an atomic way.</p
><p
>This behavior has many uses when a number of separate items are required to take effect together – examples range from setting an action timer and target at the same time, to setting a system wide property reference where both a component ID and a property address need to be changed at once.</p
><p
>Atomic operations only make sense when the master and members of the group have variable values. Usually this means that they are network writable, but can also apply to properties with volatile or implied values which are not necessarily directly writable.</p
><section
><hd
>Addresses of Properties in Atomic Groups</hd
><p
>Particular care must be taken by designers in assigning addresses to properties which form atomic groups. Since they are access order sensitive, access to them cannot be optimised so easily by controllers and badly assigned addresses can cause inefficiency in the protocol. In most cases where all properties within the group are likely to be loaded together, it is preferable to assign them contiguous addresses with the master property at the end. This allows a simple range address to be used and all values to be sent together.</p
></section
></section
></behaviordef
><behaviordef
name="atomicMaster"
><label
key="atomicMaster"
set="acnbase.lset"
></label
><refines
name="atomicGroupMember"
set="acnbase.bset"
></refines
><section
><hd
>Master Property of Atomic Load Group</hd
><p
>This property is the master property of an atomic load group. See atomicLoad for concepts. When this property is loaded, the values which have been previously set in the other member properties of the group take effect.</p
><p
>The group is defined by those properties which declare atomicLoadGroup behavior and which refer to this property as the group master. References may be structural (e.g. the master is the parent and the members are children) or by explicit property references.</p
></section
></behaviordef
><behaviordef
name="atomicTrigger"
><label
key="atomicTrigger"
set="acnbase.lset"
></label
><refines
name="atomicMaster"
set="acnbase.bset"
></refines
><section
><hd
>Atomic Action Trigger</hd
><p
>In some atomic action groups the trigger to load all the values cannot be the load of one of a network value but must be some other trigger action such as a time synchronization event, or a change in a driven, implied or volatile property. Refinements and derivatives of atomicTrigger identify the action which triggers the atomicLoad.</p
></section
></behaviordef
><behaviordef
name="atomicGroupMember"
><label
key="atomicGroupMember"
set="acnbase.lset"
></label
><refines
name="atomicLoad"
set="acnbase.bset"
></refines
><section
><hd
>Atomic Group Member</hd
><p
>This property is a member of an atomic load group. See atomicLoad for concepts. An atomicLoadGroup shall have exactly one property which is an atomicMaster. Changes to all other group member properties have no immediate effect but are simply stored, When the atomicMaster property is loaded or triggered, the most recent value stored for this property takes effect in an atomic way together with the values stored for all other members of the group.</p
><p
>The group membership is defined by a single property with atomicMaster behavior and includes as members all those properties which declare atomicLoadGroup behavior and which all refer to the same atomicMaster property. References may be structural (e.g. the master is the parent and the members are children) or by explicit property references.</p
></section
></behaviordef
><behaviordef
name="atomicParent"
><label
key="atomicParent"
set="acnbase.lset"
></label
><refines
name="atomicMaster"
set="acnbase.bset"
></refines
><section
><hd
>Atomic Master Parent Property</hd
><p
>This property is the master property of an atomic group. Any children or descendant properties which have atomicWithAncestor behavior only take effect at the time this master is written. This is the simplest form of atomicLoad property group where the parent property is the master and all other members are descendants.</p
><p
>Properties with atomicWithAncestor behavior shall only be atomic with their nearest atomicParent ancestor, therefore to ascertain which descendants are atomic with an atomicParent property, the DDL tree must not be searched below descendants which themselves have atomicParent behavior.</p
></section
></behaviordef
><behaviordef
name="atomicWithAncestor"
><label
key="atomicWithAncestor"
set="acnbase.lset"
></label
><refines
name="atomicGroupMember"
set="acnbase.bset"
></refines
><section
><hd
>Member of Atomic Property Group with Parent Master</hd
><p
>This property is a member of an atomic group whose master property is the nearest ancestor with atomicParent behavior. Any children or descendant properties which have atomicWithAncestor behavior only take effect at the time this master is written. This is the simplest form of atomicLoad property group where the parent property is the master and all other members are descendants.</p
><p
>Properties with atomicWithAncestor behavior shall only be atomic with their nearest atomicParent ancestor, therefore to ascertain which descendants are atomic with an atomicParent property, the DDL tree must not be searched below descendants which themselves have atomicParent behavior.</p
></section
></behaviordef
><behaviordef
name="atomicMasterRef"
><label
key="atomicMasterRef"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to Atomic Master</hd
><p
>This property provides a mechanism for associating an atomicGroupMember with its associated master property. The logical parent of this property shall be an atomicGroupMember and this property's value is a property reference which identifies the master property of the group.</p
><p
>The definition of a group by refrerence in this way allows an arbitrary set of properties to form an atomic group independently of other structural requirements.</p
></section
></behaviordef
><behaviordef
name="syncGroupMember"
><label
key="syncGroupMember"
set="acnbase.lset"
></label
><refines
name="atomicGroupMember"
set="acnbase.bset"
></refines
><section
><hd
>Synchronization Group Member</hd
><p
>There are cases where the results of a set-property action do not take place immediately but are timed to occur at a future time. The set of all actions which are synchronized to a particular time trigger is called a synchronization group. The trigger could be when a specified time arrives or on the occurrence of some other synchronizing action. In these cases the synchronizing action must be an atomicMaster (often an atomicTrigger).</p
></section
></behaviordef
><!--Behaviors providing dynamic definition or extension of devices and descriptions--><behaviordef
name="algorithm"
><label
key="algorithm"
set="acnbase.lset"
></label
><section
><hd
>Algorithm Behavior</hd
><p
>An abstract behavior. Refinements of this describe specific algorithms by which certain properties operate – for an example see cyclicPath algorithms associated with cyclic behaviors.</p
></section
></behaviordef
><behaviordef
name="behaviorRef"
><label
key="behaviorRef"
set="acnbase.lset"
></label
><refines
name="type.NCName"
set="acnbase.bset"
></refines
><section
><hd
>Behavior Reference</hd
><p
>The value of this property is a behavior name name identifying a behavior which applies to the parent property.</p
><p
>Use of behavior reference allows berhaviors to be dynamically assigned to a property.</p
><p
>A primary use of Behavior References is to allow declaration or selection of alternative behaviors within a single device. For example, an audio volume control may have different response curves available, each described by a separate behavior.</p
><p
>See “cyclicPath” behavior for an example of this usage.</p
><p
>Any behaviorRef property shall include a behaviorsetID child property which identifies the behaviorset to which the behavior belongs.</p
></section
></behaviordef
><behaviordef
name="paramSzArray"
><label
key="paramSzArray"
set="acnbase.lset"
></label
><section
><hd
>Parametrically Sized Array</hd
><section
><hd
>Introduction</hd
><p
>The rules of DDL do not allow multiple instances of the same device to differ in their declaration of an array size or increment within the DDL itself since these values are part of the XML content which is invariable from instance to instance. This means for example that a device description for a selector wheel with ten items cannot easily be reused for a similar selector wheel with 12 items.</p
></section
><section
><hd
>Parametrically Sized Array Mechanism</hd
><p
>The paramSzArray property declared is the first element of an array. All other elements of the array are of the same type. The paramSzArray property must have a child property with arraySize behavior which gives the number of elements in the array. The paramSzArray property's protocol access specification must normally specify an increment attribute to indicate the addresses of subsequent elements.</p
></section
><section
><hd
>Example</hd
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;This is really an array&lt;/label&gt;
  &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
  &lt;behavior name="paramSzArray" set="acnbase.bset" /&gt;
  &lt;protocol protocol="ESTA.DMP"&gt;
    &lt;propref_DMP read="true" write="true" size="2" inc="1" loc="100"/&gt;
  &lt;/protocol&gt;

  &lt;property valuetype="network"&gt;
    &lt;label&gt;Here is the array size&lt;/label&gt;
    &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
    &lt;behavior name="arraySize" set="acnbase.bset" /&gt;
    &lt;protocol protocol="ESTA.DMP"&gt;
      &lt;propref_DMP read="true" write="false" size="2" loc="99"/&gt;
    &lt;/protocol&gt;
  &lt;/property&gt;

&lt;/property&gt;</p
><p
>This declares an array of unsigned shorts at DMP addresses 100, 101... The size of the array is given by the read-only property at address 99.</p
><p
>In the same way as with a statically declared array, all children of a parametrically sized array property are themselves repeated as part of the array. The only exception is the arraySize property itself which is exempt.</p
></section
></section
></behaviordef
><behaviordef
name="arraySize"
><label
key="arraySize"
set="acnbase.lset"
></label
><refines
name="type.unsigned.integer"
set="acnbase.bset"
></refines
><section
><hd
>Array Size</hd
><p
>This property gives the size of the array of its parent property. This is the only child of a parametrically sized array property which is not considered to repeat with the array.</p
><p
>Array Size is almost invariably a read-only property.</p
></section
></behaviordef
><behaviordef
name="propertySetSelector"
><label
key="propertySetSelector"
set="acnbase.lset"
></label
><refines
name="type.enum"
set="acnbase.bset"
></refines
><section
><hd
>Property Set Selector</hd
><p
>Many devices two or more alternative sets of properties which represent, different configurations, or representations or alternative means of control and where those alternatives are mutually exclusive.</p
><p
>Each alternative, can be represented by a different set of properties and the Property Set Selector is used to choose or switch between them.</p
><p
>A Property Set Selector shall contain one or more Property Sets.</p
><p
>The child(ren) of the selected property set shall then be treated as though they appeared in the property tree at the position of the Property Set Selector.</p
><p
>Where children are plain property sets, they are assigned enumeration symbols sequentially in the order in which they are declared, starting from zero. Refinements may define other ways to assign symbols.</p
><p
>Note that while property sets are mutually exclusive, individual properties may appear in more than one property set using the shared property mechanism.</p
><p
>It is allowable to have an empty propertySet as a child. This choice then represents a way of disabling the other option(s).</p
><p
>The range of a propertySetSelector shall be from zero to N−1 where N is the number of property set children.</p
><p
>In DMP properties in unselected property sets are still accessible. Values read from such properties may be invalid or undefined. Values written to properties in unselected sets shall have no effect on the functioning or behavior of the device (unless those properties also appear in the selected property set) but they should be stored and take effect when the property set becomes slected.</p
><section
><hd
>Mutually Exclusive Property Sets Example</hd
><p
>A cyclic property represents a rotating feature of the device. This may rotate continuously and be controlled in speed alone (property set 0: speed is set at DMP location 14) or may be controlled in position (property set 1: position is set at DMP location 15). The selection between the two is made by setting DMP location 13 (the Property Set Selector) to 0 for speed or 1 for position.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;rotator&lt;/label&gt;
  &lt;behavior name="cyclic" set="acnbase.bset"/&gt;
  &lt;behavior name="type.uint" set="acnbase.bset"/&gt;
  &lt;behavior name="driven" set="acnbase.bset"/&gt;
  &lt;protocol protocol="ESTA.DMP"&gt;
    &lt;propref_DMP abs="true" read="true" size="2" loc="12"/&gt;
  &lt;/protocol&gt;

  &lt;property valuetype="network"&gt;
    &lt;label&gt;choose speed or position control&lt;/label&gt;
    &lt;behavior name="propertySetSelector" set="acnbase.bset"/&gt;
    &lt;protocol protocol="ESTA.DMP"&gt;
      &lt;propref_DMP abs="true" read="true" write="true" size="1" loc="13"/&gt;
    &lt;/protocol&gt;

    &lt;property valuetype="NULL"&gt;
      &lt;label&gt;set 0: properties for speed control&lt;/label&gt;
      &lt;behavior name="propertySet" set="acnbase.bset"/&gt;

      &lt;property valuetype="network"&gt;
        &lt;label&gt;speed input&lt;/label&gt;
        &lt;behavior name="rate1st" set="acnbase.bset"/&gt;
        &lt;behavior name="type.sint" set="acnbase.bset"/&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP abs="true" read="true" write="true" size="2" loc="14"/&gt;
        &lt;/protocol&gt;
      &lt;/property&gt;

    &lt;/property&gt;

    &lt;property valuetype="NULL"&gt;
      &lt;label&gt;set 1: properties for position control&lt;/label&gt;
      &lt;behavior name="propertySet" set="acnbase.bset"/&gt;

      &lt;property valuetype="network"&gt;
        &lt;label&gt;position input&lt;/label&gt;
        &lt;behavior name="target" set="acnbase.bset"/&gt;
        &lt;behavior name="type.uint" set="acnbase.bset"/&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP abs="true" read="true" write="true" size="2" loc="15"/&gt;
        &lt;/protocol&gt;
      &lt;/property&gt;

    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;
</p
></section
></section
></behaviordef
><behaviordef
name="propertySet"
><label
key="propertySet"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Property Set</hd
><p
>This is a simple container which holds a set of properties. It has no meaning in terms of device structure other than to group a set of properties together in an identifiable block for example to allow then to be selected by a propertySetSelector. This means that the contents of the propertySet should be treated within the device structure as though they occurred in place of the propertySet.</p
></section
></behaviordef
><behaviordef
name="label"
><label
key="label"
set="acnbase.lset"
></label
><section
><hd
>Label Property</hd
><p
>This property associates a label with its parent property. A label can be attached as a child to any property but is particularly useful for labelling preferred values, choices and similar items where it can be incorporated into a user interface for example by providing entries for a selection menu.</p
><p
>A property with label behavior is in many ways similar to the label attribute of DDL, but since it can be network accessed it can vary from instance to instance of a device, can be dynamically variable, or if persistent can allow the user to add their own labels to particular properties.</p
><p
>Refinements of label define how particular properties encode the label text.</p
></section
></behaviordef
><behaviordef
name="labelString"
><label
key="labelString"
set="acnbase.lset"
></label
><refines
name="label"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><section
><hd
>Label String</hd
><p
>A label whose text value is encoded directly as a string.</p
></section
></behaviordef
><behaviordef
name="labelRef"
><label
key="labelRef"
set="acnbase.lset"
></label
><refines
name="label"
set="acnbase.bset"
></refines
><refines
name="stringRef"
set="acnbase.bset"
></refines
><section
><hd
>Label Reference</hd
><p
>This property is a label whose text is derived using the multilingual string replacement mechanism of languagesets in DDL. See stringRef behavior.</p
><p
>See “choice” behavior for an example of multilingual labels.</p
></section
></behaviordef
><!--Grouping Behaviors
These behaviors express some common grouped constructs where a property
has no value itself but is a container for child properties.--><behaviordef
name="multidimensionalGroup"
><label
key="multidimensionalGroup"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Multidimensional Property Group</hd
><p
>A group of scalar properties representing orthogonal aspects of a single entity. Examples are x,y,z in a positional system, r,g,b in color.</p
></section
></behaviordef
><behaviordef
name="deviceInfoGroup"
><label
key="deviceInfoGroup"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Device Information Group</hd
><p
>This group property contains miscellaneous information about a device which is not normally directly relevant to control but may be of interest to users.</p
><p
>Items suitable for inclusion include, device manufacturer, model and serial numbers, information on equipment weight or dimensions. A device information group should be present in most compnents exposing devices.</p
></section
></behaviordef
><behaviordef
name="deviceSupervisory"
><label
key="deviceSupervisory"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Device Supervisory Group</hd
><p
>This group property contains miscellaneous information about a device which is relevant to overall supervisory control but does not fit into specific control channels.</p
><p
>Items suitable for inclusion include overall equipment temperature, input voltage status, operating pressure.</p
></section
></behaviordef
><behaviordef
name="sharedProps"
><label
key="sharedProps"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Shared Property Container</hd
><p
>The property sharing mechanism of DDL requires that a property which appears at multiple places in the tree, be fully declared just once (using “sharename”) and be declared by referenced in each other place that it occurs.</p
><p
>In some cases the model of a device makes it hard to pick just one occurrence which should be fully described (for example because multiple occurrences are declared together within an array or because all occurrences occur in subdevices whose device class is variable). In these cases a fully described occurence may be placed in a sharedProps container anywhere which is convenient for addressing purposes within the description. The sharedProps group itself and its immediate child properties gather no structural meaning from their position in the overall property tree, but must be fully described and have a sharename. Other occurrences of each shared property are then declared by reference at the places where they belong within the device model structure.</p
></section
></behaviordef
><!--Properties representing commonly encountered quantities or values with special encodings.--><behaviordef
name="UUID"
><label
key="UUID"
set="acnbase.lset"
></label
><refines
name="type.fixBinob"
set="acnbase.bset"
></refines
><section
><hd
>Universal Unique Identifier (UUID)</hd
><p
>Property's value is a UUID. See ESTA E1.17 (ACN) specification for further reference. Methods for generating UUIDs and binary and text formats are given in [RFC-4122: A Universally Unique IDentifier (UUID) URN Namespace. Internet Engineering Task Force (IETF) 2005].</p
><section
><hd
>Network Representation</hd
><p
>UUID properties shall be transmitted in DMP as a sequence of 16 octets in the order given as default in [RFC-4122]. This is network byte order and corresponds exactly to the left-to-right order of hexadecimal digit pairs when the UUID is written as specified.</p
></section
><section
><hd
>Immediate Values</hd
><p
>Immediate values of UUIDs shall be either specified by value or by name. The type attribute of the property value shall be “object” when specified by value and “string” when specified by name.</p
><section
><hd
>Specification by Value</hd
><p
>When specified by value, UUIDs shall be written as 32 hexadecimal digits separated by dashes after the 8th, 12th, 16th and 20th digits. Hex digits shall use lower case letters while parsers should accept either lower case or upper case. This notation is compatible with the generic format defined in DDL for values of type “object” and also with the conventional notation in [RFC-4122]</p
><p
>This form is identical to that used in UUID attributes in DDL.</p
><p
>This form may be converted to a URI by prepending “urn:uuid:”. e.g. “urn:uuid:b981bec9-953a-4482-8728-82ae8c913ee8”</p
></section
><section
><hd
>Specification by Name</hd
><p
>When specified by name, the value given shall be a string which exactly matches the name attribute of a UUIDname element defined within the same device. The value of the UUID is then given by that element's UUID attribute. See DDL specification for UUIDname element definition.</p
><p
>If a UUID does not have a name defined by a UUIDname element then specification by value is the only method available.</p
></section
></section
></section
></behaviordef
><behaviordef
name="CID"
><label
key="CID"
set="acnbase.lset"
></label
><refines
name="UUID"
set="acnbase.bset"
></refines
><section
><hd
>Component Identifier</hd
><p
>The property's value is a component identifier (CID) in accordance with the ACN definition and is formatted as specified in the generic UUID behavior.</p
></section
></behaviordef
><behaviordef
name="languagesetID"
><label
key="languagesetID"
set="acnbase.lset"
></label
><refines
name="UUID"
set="acnbase.bset"
></refines
><section
><hd
>Language Set Identifier</hd
><p
>The value of this property identifies a language set. This is the UUID of a languageset element.</p
></section
></behaviordef
><behaviordef
name="behaviorsetID"
><label
key="behaviorsetID"
set="acnbase.lset"
></label
><refines
name="UUID"
set="acnbase.bset"
></refines
><section
><hd
>Behavior Set Identifier</hd
><p
>The value of this property identifies a set of behaviors. This is the UUID of a behaviorset element.</p
></section
></behaviordef
><behaviordef
name="DCID"
><label
key="DCID"
set="acnbase.lset"
></label
><refines
name="UUID"
set="acnbase.bset"
></refines
><section
><hd
>Device Class Identifier (DCID)</hd
><p
>The value of this property is a UUID identifying a device class. This is the UUID of the device element describing the class of devices.</p
></section
></behaviordef
><behaviordef
name="time"
><label
key="time"
set="acnbase.lset"
></label
><section
><hd
>Time</hd
><p
>Abstract behavior which underlies a wide variety of time and or date behaviors.</p
></section
></behaviordef
><behaviordef
name="timePoint"
><label
key="timePoint"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><refines
name="time"
set="acnbase.bset"
></refines
><section
><hd
>Time Point</hd
><p
>A scalar property defining a point in time.</p
><p
>A time point is always relative to some agreed reference point. Examples of time points range from dates which are commonly measured in days since “0 AD”, to points in time code which are often measured in units such as frames and related to some arbitrary “start of program” point.</p
></section
></behaviordef
><behaviordef
name="countdownTime"
><label
key="countdownTime"
set="acnbase.lset"
></label
><refines
name="timePoint"
set="acnbase.bset"
></refines
><section
><hd
>Countdown Time</hd
><p
>A property representing the time until an event. The property value decrements towards zero and the associated event occurs at the time it reaches zero. The associated event and mechanism for resetting the timer are defined by refinements of this behavior.</p
></section
></behaviordef
><behaviordef
name="timePeriod"
><label
key="timePeriod"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><refines
name="time"
set="acnbase.bset"
></refines
><section
><hd
>Time Period</hd
><p
>A property defining a time period or interval.</p
></section
></behaviordef
><behaviordef
name="date"
><label
key="date"
set="acnbase.lset"
></label
><refines
name="timePoint"
set="acnbase.bset"
></refines
><section
><hd
>Date</hd
><p
>Abstract behavior expressing a date. A date may also include time of day and/or time-zone information, daylight savings time etc. Specific refinements may define formats.</p
></section
></behaviordef
><behaviordef
name="ISOdate"
><label
key="ISOdate"
set="acnbase.lset"
></label
><refines
name="date"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><section
><hd
>ISO 8601 Format Date String</hd
><p
>The property contains a date string in ISO 8601 Format. e.g. “2005-02-24T20:08:59+0000”. This is the preferred format to use for dates in DMP.</p
></section
></behaviordef
><!--References to device and protocol structures--><behaviordef
name="componentReference"
><label
key="componentReference"
set="acnbase.lset"
></label
><refines
name="reference"
set="acnbase.bset"
></refines
><section
><hd
>Component Reference</hd
><p
>This property references or points to an ACN component. Reference mechanisms include reference by CID or potentially by other protocol specific mechanisms (e.g. SDT member ID).</p
></section
></behaviordef
><behaviordef
name="deviceRef"
><label
key="deviceRef"
set="acnbase.lset"
></label
><refines
name="DCID"
set="acnbase.bset"
></refines
><refines
name="reference"
set="acnbase.bset"
></refines
><section
><hd
>Device Reference</hd
><p
>This property forms a point at which another device is joined to the current one as a sub-device.</p
><p
>The value of this property (immediate or network accessed) provides the DCID of the sub-device. The entire sub-device is to be regarded as attached at this property. Children of this property may be provided to qualify the value of the sub-device's DCID but are not a part of the sub-device.</p
><section
><hd
>Circular references:</hd
><p
>Where this property has an immediate value, the child device is invariant and is therefore implicitly a part of the definition of the parent with respect to rules on variability of device classes.</p
><p
>It is an error for any device description to reference its own DCID as an invariant child or, recursively as any invariant descendant thereof.</p
></section
><section
><hd
>Dynamic Devices</hd
><p
>As with any other property, a deviceRef may have a network value which can change. This in turn implies that the sub-device attached at this point may change. This is invaluable to enable descriptions of modular devices which contain interchangeable components or re-configurable devices. However, when a device can change “on the fly” during control, extreme care must be taken to ensure that controllers do not access properties within a newly configured device thinking them to be properties which existed in a previous configuration.</p
><p
>Specific precautions depend on the context of individual devices but can include: issuing an event when a sub-device changes; ensuring that the property maps of permissible sub-devices do not overlap or are harmonized to prevent faults; preventing sub-devices from changing while controllers are connected; defining interlock properties so that a controller must explicitly request a change of device.</p
></section
></section
><section
><hd
>NULL Device</hd
><p
>If the value of a deviceRef property is the NULL UUID 00000000-0000-0000-0000-000000000000 this shall be taken to indicate that no subdevice is attached at this point. This NULL device is useful for dynamic attachment where no subdevice is currently attached, and in template devices where the value of an immediate subdevice is parameterized and may not always be present.</p
></section
></behaviordef
><behaviordef
name="CIDreference"
><label
key="CIDreference"
set="acnbase.lset"
></label
><refines
name="CID"
set="acnbase.bset"
></refines
><refines
name="componentReference"
set="acnbase.bset"
></refines
><section
><hd
>Component Reference by CID</hd
><p
>This property references or points to an ACN component. Its value is the CID of the component referenced.</p
></section
></behaviordef
><behaviordef
name="propertyRef"
><label
key="propertyRef"
set="acnbase.lset"
></label
><refines
name="reference"
set="acnbase.bset"
></refines
><section
><hd
>Generic Property Reference</hd
><p
>An abstract behavior. Properties with this behavior or its refinements are references to other properties within the system. There are various ways to implement property references inclusing named properties and local or systemwide property numbers.</p
><p
>There is not a one-to-one correspondence between DDL properties and DMP properties since DDL can include properties with immediate values or no value at all. In general for fixed references to parts of the device model, a DDL property reference is appropriate, while for dynamic references to networked properties (e.g. for bindings), a DMP property address is more useful.</p
><section
><hd
>Note: Relationship to Standard DMP Behaviorset</hd
><p
>This behavior has more abstract behavior than the behavior of the same name in DMP behaviorset a713a314-a14d-11d9-9f34-000d613667e2. That behavior is superceded by localDDLPropertyRef in this behaviorset.</p
></section
></section
></behaviordef
><behaviordef
name="DDLpropertyRef"
><label
key="DDLpropertyRef"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to DDL Property</hd
><p
>This is a reference to a property in a DDL device model. The reference is to a property as defined by the DDL model within an instance of a device.</p
></section
></behaviordef
><behaviordef
name="namedPropertyRef"
><label
key="namedPropertyRef"
set="acnbase.lset"
></label
><refines
name="DDLpropertyRef"
set="acnbase.bset"
></refines
><refines
name="type.NCName"
set="acnbase.bset"
></refines
><section
><hd
>Named Property Reference</hd
><p
>The value of this property is a NCName which identifies a property within a device or component. The name shall match the xml:id attribute on a property within the description of the device.</p
><p
>The scope of the reference depends on the refinement of this behavior.</p
></section
></behaviordef
><behaviordef
name="localDDLpropertyRef"
><label
key="localDDLPropertyRef"
set="acnbase.lset"
></label
><refines
name="namedPropertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Local Named Property Reference</hd
><p
>A reference by name to a DDL property within the same device or component. The reference shall obey the default scoping rules of the DDL specification for element identifiers and references. That is, highest priority is a match in the same device within which this property occurs. Secondly, a single match within the same component.</p
><section
><hd
>Note: Relationship to Standard DMP Behaviorset</hd
><p
>This behavior replaces “propertyRef” behavior in DMP behaviorset a713a314-a14d-11d9-9f34-000d613667e2.</p
></section
></section
></behaviordef
><behaviordef
name="globalDDLpropertyRef"
><label
key="globalDDLPropertyRef"
set="acnbase.lset"
></label
><refines
name="namedPropertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Global Named Property Reference</hd
><p
>A reference by name to a DDL property within a component. This property shall contain a componentReference property which identifies the component within which the named property is found. The name shall match a single property within the component in accordance with the “global” scope as defined in the DDL spec.</p
><p
>Note: If the componentReference property is a CIDreference, then this property reference is truly global since CIDs have global scope.</p
></section
></behaviordef
><behaviordef
name="DMPpropertyRef"
><label
key="DMPpropertyRef"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to DMP Property</hd
><p
>This is a reference to a DMP property which is by definition a network property. The most obvious form of DMP property reference is a DMP property address (see DMPaddress behavior), but other forms of reference may be defined, and a DMP address may be qualified by component references, relative addressing schemes and so on. These should all refine this behavior.</p
></section
></behaviordef
><behaviordef
name="DMPpropertyAddress"
><label
key="DMPpropertyAddress"
set="acnbase.lset"
></label
><refines
name="type.uint"
set="acnbase.bset"
></refines
><refines
name="DMPpropertyRef"
set="acnbase.bset"
></refines
><section
><hd
>DMP Property Address</hd
><p
>The value of this property is an absolute property address. That it contains the 32-bit DMP property address. The component to which the address relates must be identified separately, either implicitly or explicitly.</p
><p
>The size shall be 4 octets (32 bits).</p
></section
></behaviordef
><behaviordef
name="localPropertyAddress"
><label
key="localPropertyAddress"
set="acnbase.lset"
></label
><refines
name="DMPpropertyAddress"
set="acnbase.bset"
></refines
><section
><hd
>Local Property Address</hd
><p
>The value of this property is the absolute address of a property in the same component. The component to which the property refers is implicitly the same as the component containing it.</p
></section
></behaviordef
><behaviordef
name="systemPropertyAddress"
><label
key="systemPropertyAddress"
set="acnbase.lset"
></label
><refines
name="DMPpropertyAddress"
set="acnbase.bset"
></refines
><refines
name="atomicParent"
set="acnbase.bset"
></refines
><section
><hd
>System-wide Property Address</hd
><p
>The value of this property is the address of a property in an arbitrary component in the system.</p
><p
>The value of this property is the absolute DMP address of the property within its component. The component shall be identified by a componentReference which forms an atomic group with this property as its master (using any of the available techniques for defining atomic groups).</p
></section
></behaviordef
><behaviordef
name="xenoPropertyReference"
><label
key="xenoPropertyReference"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Xeno or Foreign Property Reference</hd
><p
>This property identifies a value which is accessible using some protocol or network mechanism other than those used in the current device description. The nature of such a reference is necessarily dependent on the other protocol or access method.</p
><p
>This behavior provides the basis of a mechanism to implement and describe proxies and gateways. Refinements of this property for individual protocols define standard ways to describe the properties or variables accessed with protocols in DDL terms.</p
><p
>For example, a binding may be described with the master property being a standard DMP property and the slave being identified by a xenoPropertyReference. This description indicates that writes to the DMP property will generate the necessary protocol controls in the “foreign” protocol to change the foreign value. If the reference is itself a writable value, it can be used to “patch” the master to different slave values within the foreign protocol.</p
><p
>In some cases a DMP component can implement a completely transparent proxy in which it represents the remote device as a native DMP device on the network. In these cases there is no need for foreign references or bindings as the work is done entirely by the proxy component. However, with some less capable or more generic translation components, some of the details of the remote protocol must be exposed.</p
></section
></behaviordef
><behaviordef
name="transportConnection"
><label
key="transportConnection"
set="acnbase.lset"
></label
><refines
name="reference"
set="acnbase.bset"
></refines
><section
><hd
>Network Transport Connection Identifier</hd
><p
>The value of this property identifies a particular connection in a network or datalink transport. For example, in SDT a connection is an SDT session and is defined by the combination of a leader CID, a session number and a protocol. When DMP is transported on SDT and the protocol is DMP this is a DMP connection.</p
><p
>Another use of transportConnection properties is to identify other protocols in proxies or gateways.</p
><p
>Refinements of connection define the format to be used for different transports individually.</p
></section
></behaviordef
><behaviordef
name="connection.ESTA.DMP"
><label
key="connection.ESTA.DMP"
set="acnbase.lset"
></label
><refines
name="transportConnection"
set="acnbase.bset"
></refines
><section
><hd
>DMP Connection Identifier</hd
><p
>This property identifies a particular connection in the transport underlying DMP.</p
><p
>Refinements of connection define the format to be used for different transports individually.</p
></section
></behaviordef
><behaviordef
name="connection.ESTA.SDT"
><label
key="connection.ESTA.SDT"
set="acnbase.lset"
></label
><refines
name="type.uint"
set="acnbase.bset"
></refines
><refines
name="transportConnection"
set="acnbase.bset"
></refines
><section
><hd
>SDT Connection Identifier</hd
><p
>The value of this property is an SDT session number. To completely identify an arbitrary connection, the session leader CID and protocol must also be identified. If this property contains a component reference then that identifies the session leader otherwise the leader is implicitly the component containing this property.</p
><p
>The value is transmitted as a two octet unsigned integer.</p
></section
></behaviordef
><behaviordef
name="connection.ESTA.SDT.ESTA.DMP"
><label
key="connection.ESTA.SDT.ESTA.DMP"
set="acnbase.lset"
></label
><refines
name="connection.ESTA.SDT"
set="acnbase.bset"
></refines
><refines
name="connection.ESTA.DMP"
set="acnbase.bset"
></refines
><section
><hd
>DMP-SDT Connection Identifier</hd
><p
>This property identifies an SDT connection with the implicit protocol DMP. See connection-ESTA.SDT for details of format and component specification.</p
></section
></behaviordef
><!--URIs and URLs--><behaviordef
name="URI"
><label
key="URI"
set="acnbase.lset"
></label
><refines
name="type.string"
set="acnbase.bset"
></refines
><section
><hd
>URI Property</hd
><p
>The property is a URI as defined in RFC2396 or its successors and encoded in a type.string.</p
><p
>Note that while RFC2396 specifies a restricted character set, it does not specify the encoding of those characters. However this behavior defines the encoding by refinement of type.string.</p
></section
></behaviordef
><behaviordef
name="URL"
><label
key="URL"
set="acnbase.lset"
></label
><refines
name="URI"
set="acnbase.bset"
></refines
><section
><hd
>URL Property</hd
><p
>This property is a URL as defined in RFC2396.</p
><p
>“The term "Uniform Resource Locator" (URL) refers to the subset of URI that identify resources via a representation of their primary access mechanism (e.g., their network "location"), rather than identifying the resource by name or by some other attribute(s) of that resource.”</p
></section
></behaviordef
><behaviordef
name="URN"
><label
key="URN"
set="acnbase.lset"
></label
><refines
name="URI"
set="acnbase.bset"
></refines
><section
><hd
>URN Property</hd
><p
>This property is a URN as defined in RFC2396.</p
><p
>“The term "Uniform Resource Name" (URN) refers to the subset of URI that are required to remain globally unique and persistent even when the resource ceases to exist or becomes unavailable.”</p
></section
></behaviordef
><!--Device information
These items are normally included in a device information group.
See devInfoItem for full explanation --><behaviordef
name="devInfoItem"
><label
key="devInfoItem"
set="acnbase.lset"
></label
><section
><hd
>Device Information Item</hd
><p
>An single item of device information – see “devInfoGroup”. Many miscellaneous items of device information may be carried in properties with this behavior or its refinements.</p
><p
>Note that although commonly encountered items such as manufacturer or serial number are refinements of devInfoItem, the only benefit of refining new behaviors from it comes if those behaviors arewidely reused across different devices. For more obscure or device specific items, it is enough to declare a property to have deviceInfoItem behavior and to supply a suitable label attribute to indicate to the user the property's meaning. Such a property with its label becomes a simple name, value pair. e.g.</p
><p
xml:space="preserve"
>&lt;property valuetype="immediate"&gt;
  &lt;label&gt;Assistant Stylist&lt;/label&gt;
  &lt;behavior name="deviceInfoItem" set="acnbase.bset"/&gt;
  &lt;behavior name="type.string" set="acnbase.bset"/&gt;
  &lt;value type="string"&gt;Joe Bloggs&lt;/value&gt;
&lt;/property&gt;</p
></section
></behaviordef
><behaviordef
name="manufacturer"
><label
key="manufacturer"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Manufacturer</hd
><p
>The property is a string giving the manufacturer or producer of the device.</p
><p
>This string is intended primarily for human use rather than as an identifier for automated decision making.</p
><p
>There is no organized system for ensuring that manufacturer names are unique across boundaries (both international and commercial) but manufacturers should, in their own interests, take reasonable steps to ensure that the string clearly and uniquely identifies them. For example by using “Acme Controls and Networks Inc. NY, USA" rather than just “ACN”. They should also be consistent in their use across devices.</p
></section
></behaviordef
><behaviordef
name="maunfacturerURL"
><label
key="maunfacturerURL"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="URL"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Manufacturer URL</hd
><p
>This property provides a URL for the manufacturer/producer of the device.</p
><p
>The purpose of a manufacturer URL is to provide a portal to information about the manufacturer and relevant to the device, such as commercial contacts, manuals, service arrangements, catalogues, softway upgrades etc.</p
><p
>Wherever possible a manufacturer URL should point to a location where such links may be found. which typically will be a page specific to the device with wider links available, or a home-page for the division or company making the device with links down to the device relevant information.</p
></section
></behaviordef
><behaviordef
name="ESTA_OrgID"
><label
key="ESTA_OrgID"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.uint"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>ESTA Manufacturer ID</hd
><p
>The property is an ESTA numeric manufacturer ID (2 octets) for the manufacturer/producer of the device, assigned as specified in ANSI E1.11-2004. Please refer to:</p
><p
xml:space="preserve"
>http://www.esta.org/tsp/</p
></section
></behaviordef
><behaviordef
name="IEEE_OUI"
><label
key="IEEE_OUI"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.unsigned.integer"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>IEEE Organization Unique Identifier</hd
><p
>The property is the IEEE assigned “Organizationally Unique Identifier or 'company_id'” for the manufacturer/producer of the device. For details refer to:</p
><p
xml:space="preserve"
>http://standards.ieee.org/regauth/</p
><p
>The poperty shall be three octets in length and shall be transmitted in Network Byte Order.</p
></section
></behaviordef
><behaviordef
name="devModelName"
><label
key="devModelName"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Device Model Name</hd
><p
>This property is a string containing the manufacturer or producer's model name for the device.</p
><p
>This string is intended primarily for human use rather than as an identifier for automated decision making (ACN provides CIDs and DCIDs for automated control).</p
><p
>Manufacturers should take reasonable steps to ensure that all devices of the same type have the same string while device of different types have different strings across their product range.</p
><p
>This string may typically be the same as the “Fixed Component Type Name (FCTN)” available through discovery (see ACN EPI 19), but the FCTN is a component level identifier while this property is a device level item and may be assigned to sub-devices.</p
></section
></behaviordef
><behaviordef
name="devSerialNo"
><label
key="devSerialNo"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Device Serial Number</hd
><p
>This property is a string containing the manufacturer or producer's serial number for the device.</p
><p
>This string is intended primarily for human use, e.g when reporting service information, rather than as an identifier for automated decision making (ACN provides CIDs and DCIDs for automated control).</p
><p
>Manufacturers should take reasonable steps to ensure that no two devices they produce have the same value for this string.</p
></section
></behaviordef
><behaviordef
name="date.manufacture"
><label
key="date.manufacture"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="ISOdate"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Manufacture Date</hd
><p
>The property provides the date of manufacture or production of this device.</p
></section
></behaviordef
><behaviordef
name="date.firmwareRev"
><label
key="date.firmwareRev"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="ISOdate"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Firmware Revision Date</hd
><p
>This property reports the date of the latest firmware revision for the device. This is the creation date of the firmware, and not necessarily the date that the firmware was loaded into the device.</p
></section
></behaviordef
><behaviordef
name="softwareVersion"
><label
key="softwareVersion"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Software Version</hd
><p
>The property reports a string giving the version of the software (or firmware) in the device.</p
><section
><hd
>Note: String Matching</hd
><p
>Version strings will often be subject to text matching and comparison. Treatment of case and whitespace in such comparisons cannot be guaranteed. Maunfacturers must therefore ensure that version strings are consistently presented such that comparisons between strings obtained from different sources or devices will give neither false positives nor false negatives.</p
><p
>In practical terms, this means that string values of different versions must always be different irrespective of case or whitespace changes, whilst values for the same version, whatever their source, must match exactly including both case and whitespace.</p
></section
></section
></behaviordef
><behaviordef
name="hardwareVersion"
><label
key="hardwareVersion"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Hardware Version</hd
><p
>The property reports a string giving the version of the hardware of the device.</p
><p
>There is currently no standard for more detailed hardware version information (versions of electronics, mechanics etc. so producers must structure this string to give the necessary information.</p
><section
><hd
>Note: No Dynamic Reconfiguration</hd
><p
>Since this string refines “constant”, it cannot be used to express dynamically re-configuration (e.g. hot-swapping). DDLs generic mechanisms for dynamic sub-device attachment can do this.</p
></section
><section
><hd
>Note: String Matching</hd
><p
>Version strings will often be subject to text matching and comparison. Treatment of case and whitespace in such comparisons cannot be guaranteed. Maunfacturers must therefore ensure that version strings are consistently presented such that comparisons between strings obtained from different sources or devices will give neither false positives nor false negatives.</p
><p
>In practical terms, this means that string values of different versions must always be different irrespective of case or whitespace changes, whilst values for the same version, whatever their source, must match exactly including both case and whitespace.</p
></section
></section
></behaviordef
><behaviordef
name="FCTN"
><label
key="FCTN"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="constant"
set="acnbase.bset"
></refines
><section
><hd
>Fixed Component Type Name (FCTN)</hd
><p
>This property contains the Component Type Name of the ACN Component containing the device. The value shall be the same FCTN that is declared when the device advertises its services for discovery – see ACN-epi19.</p
><p
>The property value is contant and cannot be written.</p
></section
></behaviordef
><behaviordef
name="UACN"
><label
key="UACN"
set="acnbase.lset"
></label
><refines
name="devInfoItem"
set="acnbase.bset"
></refines
><refines
name="type.string"
set="acnbase.bset"
></refines
><refines
name="persistent"
set="acnbase.bset"
></refines
><section
><hd
>User Assigned Component Name (UACN)</hd
><p
>This property accesses the Component Name of the ACN Component containing the device. The value shall be the same User Assigned Component Name that is declared when the device advertises its services for discovery – see ACN-epi19.</p
><p
>If the property is writable, then this provides a means for user setting of this string via the network. The property is required to be persistent by ACN-epi19.</p
></section
></behaviordef
><!--Scaling and offsets--><behaviordef
name="scale"
><label
key="scale"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><section
><hd
>Scale</hd
><p
>Abstract behaviour which relates the value of a measure property to the associated units property. Scale shall be a logical child of the property whose value it describes. More specific behaviours are refined from scale.</p
><p
>Scale shall be a scalar value. Fractional scales must use floating point values.</p
><p
>Scale is only meaningful in terms of units so if a scale property is present, units must also be defined, either explicitly or by inheritance.</p
><p
>Scale and units are both inherited by any descendents of the parent which implicitly refer to the same measure. For example, they are inherited by limit properties and target values.</p
><section
><hd
>Default Scale</hd
><p
>If units are provided without any explicit scale property, the scale shall default to a unitScale of 1.0 so that the value expresses the units directly. The default scale of 1.0 is commonly used with immediate values.</p
></section
></section
></behaviordef
><behaviordef
name="unitScale"
><label
key="unitScale"
set="acnbase.lset"
></label
><refines
name="scale"
set="acnbase.bset"
></refines
><section
><hd
>Unit Scale</hd
><p
>Scale property which gives the size of a unit step in the associated units. Relationship is:</p
><p
xml:space="preserve"
>output = parentValue * unitScale [units]</p
><p
>If parent property is an integer, this is the size of one least significant bit increment. If parent property is a floating point value, this is the size of a step of 1.0</p
></section
></behaviordef
><behaviordef
name="fullScale"
><label
key="fullScale"
set="acnbase.lset"
></label
><refines
name="scale"
set="acnbase.bset"
></refines
><section
><hd
>Full Scale</hd
><p
>Scale property which gives the size of a full scale change of the parent property in the associated units. Relationship is:</p
><p
xml:space="preserve"
>output = V * PP / PPmax [units]</p
><p
>Where:</p
><section
><p
>V is the value of the this (fullScale) property</p
><p
>PP is the value of the parent property</p
><p
>PPmax is the maximum value which the parent property's data type can express (e.g. 0..255 for a single octet unsigned integer, 32767 for a two octet signed integer).</p
><p
>PPmax shall be determined by the data type of the parent and not by any limits behaviors attached to it. This means that two properties with the same datatype, fullScale value and units will scale identically irrespective of upper and lower limits. Also scaling does not change if limits are dynamically varied.</p
></section
><p
>Like unitScale, fullScale relates the parent property's value to the associated units property. Rather than expressing the size of each step, it expresses the size of the maximum value expressible by the property's particular datatype.</p
><p
>fullScale is often useful for properties which have no limits and which are scaled to the full binary range that their size allows (e.g. 0 - 255 for a single octet property).</p
><p
>If a fullScale property is present a units property shall also be present.</p
><p
>fullScale shall be a scalar value.</p
><p
>Fractional scales must use floating point values.</p
></section
></behaviordef
><behaviordef
name="measureOffset"
><label
key="measureOffset"
set="acnbase.lset"
></label
><refines
name="localDatum"
set="acnbase.bset"
></refines
><section
><hd
>Measure Offset</hd
><p
>This is a localDatum applied to a measure quantity. It defines the origin used by the value of its parent (and any children of the parent which inherit the parent's reference) relative to the prevailing datum - see localDatum for definition of the prevailing datum.</p
><section
><hd
>Example</hd
><p
>An automated lighting projector has a range of tilt which can vary from -45° to +90° relative to pointing straight down with a resolution of 0.1°. The projector uses an unsigned integer to represent tilt, ranging from 0 to 1350 with a unit of “°” and a unitScale of “0.1”. The projector uses the preferred polar coordinate behavior angleX to express tilt and in the coordinate system of the projector, the Z axis along which the projector is directed, points vertically upward, so a tilt value of 0 represents 135° (180° - 45°) and this is the value of the measureOffset.</p
></section
><section
><hd
>Scaling</hd
><p
>There is an ambiguity in the measurement units of a measureOffset behavior. In the example above, the offset is 135°, but it is not clear whether this should be expressed unscaled in the prevailing units as “135” or scaled according to the scaling factor applying to the parent giving “1350”.</p
><p
>Unless the measureOffset property contains its own explicit scaling property then it shall always be expressed with the units and scaling of its parent. Therefore in the example above the value of measureOffset must be “1350”.</p
><p
>The algoritm for converting from a value Vc in conventional units to the specific control values of the parent Vp (and assuming scaling is given by a unitScale property) is therefore:</p
><p
xml:space="preserve"
>Vp = (Vc/unitScale) - measureOffset</p
><p
>conversely:</p
><p
xml:space="preserve"
>Vc = (Vp + measureOffset) × unitScale</p
></section
></section
></behaviordef
><!--Dimension and units of properties--><behaviordef
name="dimension"
><label
key="dimension"
set="acnbase.lset"
></label
><section
><hd
>Dimension (and units) of property</hd
><p
>Refinements of this behavior specify the dimension (as in dimensional analysis) of the property to which it is attached. Further refinements specify units of those dimensions.</p
><p
>It is not benaficial to have a proliferation of different units for the same thing, so except in special cases, units of dimension should be the preferred SI unit.</p
><p
>This is normally applied to a measure property except in certain cases of special formats (e.g. time expressed as date and time).</p
><section
><hd
>Note: Dimensional Analysis</hd
><p
>Dimensional analysis is a conceptual tool often applied in physics, chemistry, and engineering to understand physical situations involving a mix of different kinds of physical quantities. The dimensions of a physical quantity are associated with combinations of mass, length, time, electric charge, and temperature, represented by symbols M, L, T, Q, and Θ (respectively).</p
></section
></section
></behaviordef
><!--Dimensional prefixes--><behaviordef
name="dimensional-scale"
><label
key="dimensional-scale"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Dimensional scale</hd
><p
>Behaviors refined from this define scale factors which are applied to dimensional behaviors on the same property. Dimensional scales are only applicable in combination with a dimensional behavior which defines an established unit of measure.</p
><p
>The predominant use of dimensional-scale is as a base behavior for the common prefixes micro, milli, kilo etc.</p
><p
>In refinements of this behavior scale factors use the symbol “^” to mean “raised to the power of”. Thus 10^3 is ten to the power of three which is 1000.</p
></section
></behaviordef
><behaviordef
name="prefix-yocto"
><label
key="prefix-yocto"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix yocto</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-24.</p
></section
></behaviordef
><behaviordef
name="prefix-zepto"
><label
key="prefix-zepto"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix zepto</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-21.</p
></section
></behaviordef
><behaviordef
name="prefix-atto"
><label
key="prefix-atto"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix atto</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-18.</p
></section
></behaviordef
><behaviordef
name="prefix-femto"
><label
key="prefix-femto"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix femto</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-15.</p
></section
></behaviordef
><behaviordef
name="prefix-pico"
><label
key="prefix-pico"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix pico</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-12.</p
></section
></behaviordef
><behaviordef
name="prefix-nano"
><label
key="prefix-nano"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix nano</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-9.</p
></section
></behaviordef
><behaviordef
name="prefix-micro"
><label
key="prefix-micro"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix micro</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-6.</p
></section
></behaviordef
><behaviordef
name="prefix-milli"
><label
key="prefix-milli"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix milli</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^-3.</p
></section
></behaviordef
><behaviordef
name="prefix-kilo"
><label
key="prefix-kilo"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix kilo</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^3.</p
></section
></behaviordef
><behaviordef
name="prefix-mega"
><label
key="prefix-mega"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix mega</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^6.</p
></section
></behaviordef
><behaviordef
name="prefix-giga"
><label
key="prefix-giga"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix giga</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^9.</p
></section
></behaviordef
><behaviordef
name="prefix-tera"
><label
key="prefix-tera"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix tera</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^12.</p
></section
></behaviordef
><behaviordef
name="prefix-peta"
><label
key="prefix-peta"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix peta</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^15.</p
></section
></behaviordef
><behaviordef
name="prefix-exa"
><label
key="prefix-exa"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix exa</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^18.</p
></section
></behaviordef
><behaviordef
name="prefix-zetta"
><label
key="prefix-zetta"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix zetta</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^21.</p
></section
></behaviordef
><behaviordef
name="prefix-yotta"
><label
key="prefix-yotta"
set="acnbase.lset"
></label
><refines
name="dimensional-scale"
set="acnbase.bset"
></refines
><section
><hd
>Prefix yotta</hd
><p
>The dimensional unit attached to the same property must be multiplied by 10^24.</p
></section
></behaviordef
><!--Dimensions and derived units--><behaviordef
name="dim-mass"
><label
key="dim-mass"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Mass dimension</hd
><p
>The property represents a mass.</p
><p
>Dimension: M</p
></section
></behaviordef
><behaviordef
name="mass-g"
><label
key="mass-g"
set="acnbase.lset"
></label
><refines
name="dim-mass"
set="acnbase.bset"
></refines
><section
><hd
>Mass in grams (g)</hd
><p
>The property - when scaled by any scale factor present - represents mass expressed in grams (g).</p
><section
><hd
>Note on SI Units</hd
><p
>The preferred SI unit for mass is the kilogram. 1g = 1kg/1000. However, since these behaviors allow the prefix to be set independently of the base unit, the unit here is g to avoid confusion when used in combination with prefixes.</p
></section
></section
></behaviordef
><behaviordef
name="dim-length"
><label
key="dim-length"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Length dimension</hd
><p
>The property represents a length.</p
><p
>Dimension: L</p
></section
></behaviordef
><behaviordef
name="length-m"
><label
key="length-m"
set="acnbase.lset"
></label
><refines
name="dim-length"
set="acnbase.bset"
></refines
><section
><hd
>Length in metres (m)</hd
><p
>The property - when scaled by any scale factor present - represents length expressed in metres (m).</p
></section
></behaviordef
><behaviordef
name="dim-time"
><label
key="dim-time"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Time dimension</hd
><p
>The property represents a time.</p
><p
>Dimension: T</p
></section
></behaviordef
><behaviordef
name="time-s"
><label
key="time-s"
set="acnbase.lset"
></label
><refines
name="dim-time"
set="acnbase.bset"
></refines
><section
><hd
>Time in seconds (s)</hd
><p
>The property - when scaled by any scale factor present - represents time expressed in seconds (s).</p
></section
></behaviordef
><behaviordef
name="dim-charge"
><label
key="dim-charge"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Charge dimension</hd
><p
>The property represents an electrical charge.</p
><p
>Dimension: Q</p
></section
></behaviordef
><behaviordef
name="charge-C"
><label
key="charge-C"
set="acnbase.lset"
></label
><refines
name="dim-charge"
set="acnbase.bset"
></refines
><section
><hd
>Charge in coulombs (C)</hd
><p
>The property - when scaled by any scale factor present - represents electric charge expressed in coulombs (C).</p
></section
></behaviordef
><behaviordef
name="dim-temp"
><label
key="dim-temp"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Temperature dimension</hd
><p
>The property represents a temperature.</p
><p
>Dimension: Θ</p
></section
></behaviordef
><behaviordef
name="temp-K"
><label
key="temp-K"
set="acnbase.lset"
></label
><refines
name="dim-temp"
set="acnbase.bset"
></refines
><section
><hd
>Temperature in kelvins (K)</hd
><p
>The property - when scaled by any scale factor present - represents temperature in kelvins (K).</p
></section
></behaviordef
><behaviordef
name="temp-celsius"
><label
key="temp-celsius"
set="acnbase.lset"
></label
><refines
name="dim-temp"
set="acnbase.bset"
></refines
><section
><hd
>Temperature in Degrees Celsius (°C)</hd
><p
>The property - when scaled by any scale factor present - represents temperature in degrees Celsius (°C).</p
><p
>Note: °C is NOT an SI unit but is included as it is more commonly understood and used in many situations. For SI compatibility temp-celsius may be converted to the SI unit temp-K by adding 273.15.</p
></section
></behaviordef
><behaviordef
name="dim-angle"
><label
key="dim-angle"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Angle dimension</hd
><p
>The property represents an angle.</p
><p
>In conventional dimensional analysis, angle has no dimension. It is nevertheless convenient to treat it as one.</p
></section
></behaviordef
><behaviordef
name="angle-rad"
><label
key="angle-rad"
set="acnbase.lset"
></label
><refines
name="dim-angle"
set="acnbase.bset"
></refines
><section
><hd
>Angle in radians (rad)</hd
><p
>The property - when scaled by any scale factor present - represents angle expressed in radians.</p
><p
>Note: Radians are an SI unit and are invaluable in many situations, but an angle-deg behavior is also provided for properties better expressed in degrees.</p
></section
></behaviordef
><behaviordef
name="angle-deg"
><label
key="angle-deg"
set="acnbase.lset"
></label
><refines
name="dim-angle"
set="acnbase.bset"
></refines
><section
><hd
>Angle in degrees (°)</hd
><p
>The property - when scaled by any scale factor present - represents angle expressed in degrees (°).</p
><p
>Note: Degrees is NOT an SI unit but is included as it is more commonly understood and used in many situations. angle-deg may be converted to the SI unit angle-rad by multiplying by π/180 (pi/180).</p
></section
></behaviordef
><behaviordef
name="dim-solid-angle"
><label
key="dim-solid-angle"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Solid angle dimension</hd
><p
>The property represents a solid angle.</p
><p
>In conventional dimensional analysis, solid angle has no dimension. It is nevertheless convenient to treat it as one.</p
></section
></behaviordef
><behaviordef
name="solid-angle-sr"
><label
key="solid-angle-sr"
set="acnbase.lset"
></label
><refines
name="dim-solid-angle"
set="acnbase.bset"
></refines
><section
><hd
>Solid angle in steradians (sr)</hd
><p
>The property - when scaled by any scale factor present - represents solid angle expressed in steradians.</p
></section
></behaviordef
><behaviordef
name="dim-freq"
><label
key="dim-freq"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Frequency dimension</hd
><p
>The property represents a frequency.</p
><p
>Dimension: 1/T</p
></section
></behaviordef
><behaviordef
name="freq-Hz"
><label
key="freq-Hz"
set="acnbase.lset"
></label
><refines
name="dim-freq"
set="acnbase.bset"
></refines
><section
><hd
>Frequency in hertz (Hz)</hd
><p
>The property - when scaled by any scale factor present - represents frequency expressed in hertz (Hz).</p
></section
></behaviordef
><behaviordef
name="dim-area"
><label
key="dim-area"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Area dimension</hd
><p
>The property represents an area.</p
><p
>Dimension: L^2</p
></section
></behaviordef
><behaviordef
name="area-sq-m"
><label
key="area-sq-m"
set="acnbase.lset"
></label
><refines
name="dim-area"
set="acnbase.bset"
></refines
><section
><hd
>Area in square metres (m^2)</hd
><p
>The property - when scaled by any scale factor present - represents area expressed in square metres (m^2).</p
></section
></behaviordef
><behaviordef
name="dim-volume"
><label
key="dim-volume"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Volume dimension</hd
><p
>The property represents a volume.</p
><p
>Dimension: L^3</p
></section
></behaviordef
><behaviordef
name="volume-cu-m"
><label
key="volume-cu-m"
set="acnbase.lset"
></label
><refines
name="dim-volume"
set="acnbase.bset"
></refines
><section
><hd
>Volume in cubic metres (m^3)</hd
><p
>The property - when scaled by any scale factor present - represents volume expressed in cubic metres (m^3).</p
></section
></behaviordef
><behaviordef
name="volume-L"
><label
key="volume-L"
set="acnbase.lset"
></label
><refines
name="dim-volume"
set="acnbase.bset"
></refines
><section
><hd
>Volume in liters (L)</hd
><p
>The property - when scaled by any scale factor present - represents volume expressed in liters (L).</p
><section
><hd
>Note: non-SI unit</hd
><p
>The liter is not an SI unit. It can be converted into the SI unit of M^3 by multiplying by 0.001. (1 cubic meter = 1000 liters). It is therefore the same as a cubic meter with a prefix on "milli".</p
></section
></section
></behaviordef
><behaviordef
name="dim-force"
><label
key="dim-force"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Force dimension</hd
><p
>The property represents a force.</p
><p
>Dimension: ML/T^2</p
></section
></behaviordef
><behaviordef
name="force-N"
><label
key="force-N"
set="acnbase.lset"
></label
><refines
name="dim-force"
set="acnbase.bset"
></refines
><section
><hd
>Force in newtons (N)</hd
><p
>The property - when scaled by any scale factor present - represents force expressed in newtons (N).</p
></section
></behaviordef
><behaviordef
name="dim-energy"
><label
key="dim-energy"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Energy dimension</hd
><p
>The property represents an energy.</p
><p
>Dimension: ML^2/T^2</p
></section
></behaviordef
><behaviordef
name="energy-J"
><label
key="energy-J"
set="acnbase.lset"
></label
><refines
name="dim-energy"
set="acnbase.bset"
></refines
><section
><hd
>Energy in joules (J)</hd
><p
>The property - when scaled by any scale factor present - represents energy expressed in joules (J).</p
></section
></behaviordef
><behaviordef
name="dim-power"
><label
key="dim-power"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Power dimension</hd
><p
>The property represents power.</p
><p
>Dimension: ML^2/T^3</p
></section
></behaviordef
><behaviordef
name="power-W"
><label
key="power-W"
set="acnbase.lset"
></label
><refines
name="dim-power"
set="acnbase.bset"
></refines
><section
><hd
>Power in watts (W)</hd
><p
>The property - when scaled by any scale factor present - represents power expressed in watts (W).</p
></section
></behaviordef
><behaviordef
name="dim-pressure"
><label
key="dim-pressure"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Pressure dimension</hd
><p
>The property represents a pressure.</p
><p
>Dimension: M/(LT^2)</p
></section
></behaviordef
><behaviordef
name="pressure-Pa"
><label
key="pressure-Pa"
set="acnbase.lset"
></label
><refines
name="dim-pressure"
set="acnbase.bset"
></refines
><section
><hd
>Pressure in pascals (Pa)</hd
><p
>The property - when scaled by any scale factor present - represents pressure expressed in pascals (Pa).</p
></section
></behaviordef
><behaviordef
name="dim-current"
><label
key="dim-current"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Electric current dimension</hd
><p
>The property represents a current.</p
><p
>Dimension: Q/T</p
></section
></behaviordef
><behaviordef
name="current-A"
><label
key="current-A"
set="acnbase.lset"
></label
><refines
name="dim-current"
set="acnbase.bset"
></refines
><section
><hd
>Current in amps (A)</hd
><p
>The property - when scaled by any scale factor present - represents electric current expressed in amps (A).</p
></section
></behaviordef
><behaviordef
name="dim-voltage"
><label
key="dim-voltage"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Electromotive force dimension</hd
><p
>The property represents electromotive force (also called electrical potential difference or voltage).</p
><p
>Dimension: ML^2/(QT^2)</p
></section
></behaviordef
><behaviordef
name="voltage-V"
><label
key="voltage-V"
set="acnbase.lset"
></label
><refines
name="dim-voltage"
set="acnbase.bset"
></refines
><section
><hd
>Voltage or EMF in volts (V)</hd
><p
>The property - when scaled by any scale factor present - represents electro motive force (voltage) expressed in volts (V).</p
></section
></behaviordef
><behaviordef
name="dim-resistance"
><label
key="dim-resistance"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Electric resistance dimension</hd
><p
>The property represents a resistance.</p
><p
>Dimension: ML^2/(Q^2T)</p
></section
></behaviordef
><behaviordef
name="resistance-ohm"
><label
key="resistance-ohm"
set="acnbase.lset"
></label
><refines
name="dim-voltage"
set="acnbase.bset"
></refines
><section
><hd
>Resistance in ohms (Ω)</hd
><p
>The property - when scaled by any scale factor present - represents electrical resistance expressed in ohms (Ω).</p
></section
></behaviordef
><behaviordef
name="dim-torque"
><label
key="dim-torque"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Torque dimension</hd
><p
>The property represents a torque (also called moment or couple).</p
><p
>Dimension: ML^2/T^2</p
></section
></behaviordef
><behaviordef
name="torque-Nm"
><label
key="torque-Nm"
set="acnbase.lset"
></label
><refines
name="dim-torque"
set="acnbase.bset"
></refines
><section
><hd
>Torque in newton metres (Nm)</hd
><p
>The property - when scaled by any scale factor present - represents torque expressed in newton metres (Nm).</p
></section
></behaviordef
><behaviordef
name="perceptual-dimension"
><label
key="perceptual-dimension"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Perceptually Weighted Dimension</hd
><p
>A number of dimensional measures are based on the characteristics of the human senses. For example, the most common photometric measures are based not on the power of light, but are wavelength-weighted to match a model of the perceptual characteristics of the human eye. This makes a distinction between for example luminous flux which is perceptually weighted and radiant flux which expresses un-adjusted power across the entire spectrum.</p
><p
>Other common examples are encountered in acoustics.</p
><p
>Dimensions and units which use a perceptually weighted measure should refine this behavior. Other refinements of this behavior may be used to specify the weighting model, where different ones may be in use.</p
></section
></behaviordef
><behaviordef
name="dim-luminous-intensity"
><label
key="dim-luminous-intensity"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><refines
name="perceptual-dimension"
set="acnbase.bset"
></refines
><section
><hd
>Luminous Intensity dimension</hd
><p
>The property represents a luminous intensity.</p
><p
>Dimension: ML^2/T^3 (power per solid angle – adjusted for human eye wavelength sensitivity)</p
></section
></behaviordef
><behaviordef
name="luminous-intensity-cd"
><label
key="luminous-intensity-cd"
set="acnbase.lset"
></label
><refines
name="dim-luminous-intensity"
set="acnbase.bset"
></refines
><section
><hd
>Luminous intensity in candela (cd)</hd
><p
>The property - when scaled by any scale factor present - represents luminous intensity expressed in candela (cd).</p
></section
></behaviordef
><behaviordef
name="dim-luminous-flux"
><label
key="dim-luminous-flux"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><refines
name="perceptual-dimension"
set="acnbase.bset"
></refines
><section
><hd
>Luminous Flux dimension</hd
><p
>The property represents a luminous flux.</p
><p
>Dimension: ML^2/T^3 (power – adjusted for human eye wavelength sensitivity)</p
></section
></behaviordef
><behaviordef
name="luminous-flux-lm"
><label
key="luminous-flux-lm"
set="acnbase.lset"
></label
><refines
name="dim-luminous-flux"
set="acnbase.bset"
></refines
><section
><hd
>Luminous flux in lumens (lm)</hd
><p
>The property - when scaled by any scale factor present - represents luminous flux expressed in lumens (lm).</p
></section
></behaviordef
><behaviordef
name="dim-illuminance"
><label
key="dim-illuminance"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><refines
name="perceptual-dimension"
set="acnbase.bset"
></refines
><section
><hd
>Illuminance dimension</hd
><p
>The property represents a luminous flux.</p
><p
>Dimension: ML^2/T^3 (power – adjusted for human eye wavelength sensitivity)</p
></section
></behaviordef
><behaviordef
name="illuminance-lx"
><label
key="illuminance-lx"
set="acnbase.lset"
></label
><refines
name="dim-illuminance"
set="acnbase.bset"
></refines
><section
><hd
>Illuminance in lux (lx)</hd
><p
>The property - when scaled by any scale factor present - represents illuminance expressed in lux (lx).</p
></section
></behaviordef
><behaviordef
name="ratio"
><label
key="ratio"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><refines
name="dimension"
set="acnbase.bset"
></refines
><section
><hd
>Ratio</hd
><p
>Two ratiometric units are included in DMPunit [dmpBase.bset]: the decibel (dB) and the neper (Np). However their use is unclear and the simplest ratiometric measure, the linear numerical ratio, is not included.</p
><p
>A ratio has no units, but there are many cases where a property represents a ratio of one quantity to another. Examples include amplifiers and attenuators, dimmers, transformers, modulators, PWM controllers etc.</p
><p
>In order to correctly interpret a ratio property, three pieces of information are required. These are the two quantities being related and the dimension that the relationship is measured in.</p
><section
><hd
>Quantities Related</hd
><p
>The quantities related by a ratio may be property values, they may be implicit quantities (for example the ratio of an attenuator relates the “downstream” value to the “upstream” value) or one or other may be arbitrary external references. (for example the common measure dBm relates to an arbitrary reference of 1mW).</p
><p
>The quantities which are related by a ratio are defined by refinements of ratio behavior and may be additionally specified by properties conatained within it.</p
></section
><section
><hd
>Dimension of Ratio</hd
><p
>Any ratio property must also carry a dimension behavior to indicate the dimension of the quantities related, unless that dimension is expressed clearly in another way (e.g. by refinement of this behavior).</p
><p
>If the dimension related by the ratio is not specified erroneous results may be inferred. A very relevant example occurs in electrical signals where the most easily measured quantity is often voltage, while ratios in dimmers, attenuators or amplifiers are usually expressed in terms of power which increases with voltage squared (if load is constant).</p
><p
>Other examples commonly occur with loose notions such as “size” of an object which may refer to the linear size (length), or the volume or mass which in many cases increases as the cube of the length where proportion and density are maintained.</p
><section
><hd
>Negative Ratios</hd
><p
>For some quantities a negative ratio makes sense (e.g. a DC voltage), whilst for many it does not (power of a signal). Signed ratios are permitted, but if a signed number representation is used for a ratio which cannot reasonably be negative then a limit must be imposed, e.g. using a limit subproperty.</p
></section
><section
><hd
>Note: Dimensional Analysis</hd
><p
>While a ratio has no units (a length ratio of 0.5 is the same whether length is measured in meters or yards), if the ratio is expressed as A/B dimensional analysis requires that the dimensions of A and of B are the same and this is refered to as the dimension of the ratio. See dimension behavior for more discussion of dimensional analysis principles.</p
></section
></section
><section
><hd
>Scaling of Ratios</hd
><p
>In the absence of specific non-linearities, a ratio property shall be linear. That is a scaling factor expressed as a rational number with 1.0 meaning the two related quantities are equal. A scale property within a ratio defines the scale relating to 1.0 with no units.</p
><section
><hd
>Scaling examples</hd
><p
>A single octet ratio property with a fullScale subproperty of 1.0, implies that a value of 0 represents 0.0 (off, no result), 1 represents 1/255, 2 is 2/255 up to 255 which represents 1.0 (1:1 ratio - direct equality), with linear scaling in between. When relating power, this is the typical scaling of a traditional lighting dimmer.</p
><p
>A two octet property with type.sint behavior and a unitscale subproperty of 0.001 expresses a ratio in thousandths ranging from -32.768 through 0.0 to +32.767.</p
></section
></section
><section
><hd
>Non-linear ratios</hd
><p
>Non-linear measures of ratio are common. The dB is widely used and precisely defined as is the neper. In some cases non-linear ratios are more complex. For example in a phase control dimmer, linear change in phase delay does not produce a linear change in power, voltage or any common measure. The theoretical transfer function being an integral of a sinusoid.</p
><p
>Non-linear behaviors may be applied to ratios to define such relationships. Such non-linearities always relate back to the linear scale.</p
></section
><section
><hd
>Percentages</hd
><p
>Whilst commonly used as measure of ratio, a percentage is nothing more than a ratio with a unitscale of 0.01. There is no explicit percentage behavior or unit in DDL, but applications may trivially convert linear ratios to percentages for presentation to users if required.</p
></section
><section
><hd
>decibel and neper units deprecated.</hd
><p
>As noted above, the use of dB and Np in DMPunit behavior is ill defined and is deprecated. New behaviors derived from ratio and non-linearity should be used instead.</p
></section
><section
><hd
>Ratios Relating to Standard Values</hd
><p
>There are a number of common measurements which consist of a ratio relating to a standard value. These are frequently quantities where a logarithmic measure is more relevant than a linear one. Examples include sound pressure levels and pH values. See logunit for more details.</p
></section
></section
></behaviordef
><behaviordef
name="logratio"
><label
key="logratio"
set="acnbase.lset"
></label
><refines
name="ratio"
set="acnbase.bset"
></refines
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Logarithmic Ratio</hd
><p
>It is very common in many fields of electrical engineering and elsewhere to use a logarithmic scale to express a ratio. The most common measure is the decibel (dB) which uses logarithm base 10, but natural logarithms are also used in nepers (Np) and there are a variety of other measures.</p
><p
>Refinements of this behavior specify the particular scale used.</p
><section
><hd
>Scaling of logratios</hd
><p
>If this behavior is used unrefined, the scale shall be taken to relate to logarithms to the base ten. Thus a unitscale value of 1 implies a full decade per step.</p
><p
>A property directly expressing the commonly used dB scale would therefore have a unitscale of 0.1. Because dB are so common, they have their own derivative.</p
></section
><section
><hd
>Logarithmic Units vs Logarithmic Ratios</hd
><p
>Logarithmic scales are also used in a number of cases to generate absolute units, by relating the ratio of a quantity to some standard measure. Examples include pH and sound pressure levels.</p
><p
>It is impoortant to distinguish between pure ratios (e.g. dB) and absolute logarithmic units which often have similar names or symbols and may be used very loosely (e.g. dB(A), dBm). A complimentary behavior to this one logunit, defines such units.</p
></section
></section
></behaviordef
><behaviordef
name="logunit"
><label
key="logunit"
set="acnbase.lset"
></label
><refines
name="dimension"
set="acnbase.bset"
></refines
><refines
name="logratio"
set="acnbase.bset"
></refines
><section
><hd
>Logarithmic Units</hd
><p
>There is a widely used class of units which relate the quantity measured to a standard reference unit using a logarithmic scale (frequently dB). Examples include sound pressure levels dB(SPL) and pH values. There is more discussion of this under logratio behavior.</p
><p
>To use such a unit, both the logarithmic scale (Log10, Np, dB etc.), and the reference value must be specified.</p
><p
>Refinements of this behavior describe such absolute units. For purely relative measures such as dB or Np, see logratio.</p
></section
></behaviordef
><behaviordef
name="power-dBmW"
><label
key="power-dBmW"
set="acnbase.lset"
></label
><refines
name="logunit"
set="acnbase.bset"
></refines
><section
><hd
>Power in dBmW</hd
><p
>This property expresses a power measured in dBmW. That is the ratio expressed in dB of the power represented by the property to a reference of 1mW.</p
></section
></behaviordef
><!--Non-linear properties--><behaviordef
name="nonLinearity"
><label
key="nonLinearity"
set="acnbase.lset"
></label
><section
><hd
>Non Linearity</hd
><p
>This property indicate that there is a non-linear relationship between the value of the parent property and the device function its represents.</p
><p
>Refinements of this behavior may indicate specific non-linear relationships or classes of relationship.</p
></section
></behaviordef
><behaviordef
name="scalable-nonLinearity"
><label
key="scalable-nonLinearity"
set="acnbase.lset"
></label
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Scalable nonlinear property value</hd
><p
>nonLinearity behavior specifies that there is a non-linear relationship between the property value carrying this behavior and the units of the quantity it represents. This behavior refines from that one but provides rules on how its refinements are scaled.</p
><section
><hd
>Note: Definition of term</hd
><p
>For the purposes of this and derived behaviors, non-linearity means that the relationship between the property value Vp and the value of the quantity it represents Vq cannot be expressed just by using a constant offset and constant scale actor – when drawn on a graph with uniform scales onthe axes, it does not form a straight line.</p
><p
>While these behaviors define functions and transformations, this definition of non-linearity is not the same as used in mathematics.</p
></section
><section
><hd
>Transformations</hd
><p
>In general the quantity represented by the property varies by some function of the property value:</p
><p
xml:space="preserve"
>Vq = f(Vp)

Vq is the value of the represented quantity and Vp is the value of the property</p
><p
>Where a scalable-nonLinearity property also has a scaling sub-property, the scale shall be applied to the value of the property before the non-linear transformation, and not to the “output value” or the quantity represented. Exceptions to this rule must clearly and explicitly state how scaling applies.</p
><p
>Thus for a scale value c, and a non-linearity f() we have:</p
><p
xml:space="preserve"
>Vq = f(Vp⋅c)   Note: NOT c⋅f(Vp)</p
><p
>Any scalable non-linearity must specify its transformation clearly enough for this expression to be deterministic. If necessary it must specify the units used.</p
><section
><hd
>Warning: Transformations and their Inverses</hd
><p
>A non-linear behavior must be absolutely clear about which way the transformation is expressed. There are common non-linearities which are named after the inverse transformation from that in the equations above. Notably logarithmic transforms where the property value is the log of the quantity rather than the other way around.</p
></section
></section
><section
><hd
>Example</hd
><p
>For example a nonlin-log10 property with unitScale of 0.1 represents a quantity as follows:</p
><p
xml:space="preserve"
>Vq = 10^(0.1⋅Vp)</p
><p
>Which is the familiar decbel scale. See warning above.</p
></section
></section
></behaviordef
><behaviordef
name="normalized-nonlinearity"
><label
key="normalized-nonlinearity"
set="acnbase.lset"
></label
><refines
name="scalable-nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Normalized nonlinear function between bounds</hd
><section
><hd
>Introduction</hd
><p
>A function like y = 10^x (nonlin-log10 behavior) or y = x^2 (nonlin-squareLaw behavior) is unbounded, in that there is no particular restriction on the value of the input x. However, there are a very widely used class of nonlinear functions whch have a restricted range of input, either by some natural condition or by application of arbitrary limits and an output which is also restricted in range. This means that the endpoints of the function are fixed, and the function only applies for values between those endpoints.</p
><p
>An example is the common S-curve of phase controlled dimmers (nonlin-S-curve) where the input value is restricted between 0 and 180° firing angle. The output varies from zero to full power over this range.</p
></section
><section
><hd
>Bounds</hd
><p
>For a normalized-nonliearity, the bounds on the property value are expressed in the normal way - either as limits sub-properties, or as the full range of the underlying type (e.g. 0..255 for a 1 octet unsigned integer). These define the bounds of the nonlinear function. For purposes of calculating the actual quantity, the property value takes uniform steps between these limits.</p
></section
><section
><hd
>Normalization</hd
><p
>A function which is bounded as above can be normalized by application of suitable scaling (and in some cases offset) to a generic form such that as the input ranges from min to max the output ranges from 0 to 1.0. Given such a normalization, it is possible to define scaling in a generic way or to substitute different functions such that the framework does not change and for many control purposes the details of the function need not be known.</p
><p
>A good example of this is the choice of transfer curve for a dimmer – whatever the chosen curve, as the input vaires from min to max, the output ranges from zero to full-power.</p
><p
>Refinements of this behavior indicate a function which has been normalized in this way. Refinements must state clearly what the normalization is.</p
></section
><section
><hd
>Scaling of Normalized nonlinearities</hd
><p
>Contrary to the general principle of other scalable-nonLinearity properties, on a normalized property, the scale represents output value. This is because the input range is defined by the bounds and it is usually necessary to know the actual output at these limits rather than the normalized output.</p
><section
><hd
>Use of unitScale scaling</hd
><p
>Where the scaling is provided by a unitScale behavior, the output may be calculated, by taking the number of steps of the property (number of unit steps for a float) from minimum to maximum and multiplying this by the unitScale value.</p
><p
xml:space="preserve"
>at P = Plolim,  Q = 0
at p = Philim,  Q = (Philim - Plowlim) * unitScale</p
><p
>Where Q is the actual value of the quantity represented, P is the property value, and Plolim and Pilim are the low and high limits imposed on the property.</p
></section
><section
><hd
>Use of fullScale scaling</hd
><p
>fullScale behavior defines the scale between the limits of the underlying property type, irrespective of applied limits. in keeping with this a fullScale behavior on a normalized-nonlinearity property indicates that the input range of the function is the full range of the property type, and the fullScale property indicates the output values at the full extents of this range. Any limits properties present restrict the bounds of the input value without changing the scaling or normalization of the function. Furthermore for signed types, the lower limit for scaling is defined to be the property value of 0 and not the negative minimum. Refinements of this behavior may define normalized functions which allow negative inputs.</p
><p
xml:space="preserve"
>at P = 0,   Q = 0
at P = typemax,  Q = fullScale</p
><p
>where Q is the actual value of the quantity represented, P is the property value, and typemax is the maximum value representable by the underlying data type (e.g. 32767 for a 2-byte signed integer).</p
></section
><section
><hd
>Examples</hd
><section
><p
>Given a property with normalized-nonlinearity behavior, length-m behavior, type.uint behavior and a size of 2 octets. A fullscale behavior of 0.2 would indicate that the position or length represented was 0 for a property value of 0 and 20cm for a property value of 65535, but changed in some nonlinear fashion between these points.</p
></section
><section
><p
>A property has a size of 1 octet. It has behaviors type.uint, nonlin-S-curve &amp; voltage-V. It has two child properties with immediate values: A. with a limitMaxInc behavior and value 100. B. with unitScale behavior and a value of 0.4.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;behavior name="type.uint" set="acnbase.bset"/&gt;
  &lt;behavior name="nonlin-S-curve" set="acnbase.bset"/&gt;
  &lt;behavior name="voltage-V" set="acnbase.bset"/&gt;
  &lt;protocol name="ESTA.DMP"&gt;
    &lt;proppref_DMP ... size="1"/&gt;
  &lt;/protocol&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="limitMaxInc" set="acnbase.bset"/&gt;
    &lt;value type="uint"&gt;100&lt;/value&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="unitScale" set="acnbase.bset"/&gt;
    &lt;value type="float"&gt;0.4&lt;/value&gt;
  &lt;/property&gt;
&lt;property&gt;</p
><p
>This represents a voltage which varies from 0 to 40V following an S-curve (or S-law) profile for input values ranging from 0 to 100</p
></section
></section
></section
></section
></behaviordef
><behaviordef
name="nonlin-log"
><label
key="nonlin-log"
set="acnbase.lset"
></label
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Logarithmic Property</hd
><p
>This property changes as the logarithm of the quantity it represents. This means that a specific change in the property value represents a fixed multiplication of the represented quantity.</p
><p
xml:space="preserve"
>Vp = logb(Vq) where logb means log to the base b
  or
Vq = b^(Vp) where b is the base of the logarithm</p
><p
>nonlin-log alone represents a general relationship, but in order to be able to calculate actual values, the base of the logarithm must be known. Refinements of this behavior may specify specific logarithmic relationships.</p
><p
>Note, for most logarithmic properties, the provisions of logratio and logunit and their refinements are likely to be more suitable. However, nonlin-log and its refinements are particularly applicable in cases where different non-linear relationships may be selected.</p
></section
></behaviordef
><behaviordef
name="nonlin-log10"
><label
key="nonlin-log10"
set="acnbase.lset"
></label
><refines
name="nonlin-log"
set="acnbase.bset"
></refines
><refines
name="scalable-nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Logarithm base 10 property</hd
><p
>This property changes as log base 10 of the quantity it represents.</p
><p
xml:space="preserve"
>Vp = log10(Vq)
  or
Vq = 10^Vp</p
><p
>See scalable-nonLinearity behavior for further explanation.</p
><section
><hd
>Example</hd
><p
>See scalable-nonLinearity for an example.</p
></section
></section
></behaviordef
><behaviordef
name="nonlin-ln"
><label
key="nonlin-ln"
set="acnbase.lset"
></label
><refines
name="nonlin-log"
set="acnbase.bset"
></refines
><refines
name="scalable-nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Natural Logarithm property</hd
><p
>This property changes as the natural logarithm (log base e) of the quantity it represents. Scaling applied to the same property relates the value to this scale, so for example a unitscale of 1.0 generates the neper scale (Np).</p
><p
xml:space="preserve"
>Vp = ln(Vq)
  or
Vq = e^Vp</p
></section
></behaviordef
><behaviordef
name="nonlin-squareLaw"
><label
key="nonlin-squareLaw"
set="acnbase.lset"
></label
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><refines
name="scalable-nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Generic “Square-law” response curve</hd
><p
>This property changes with the quantity it represents according to a “square law”. This means that the quantity is the property value squared (or proportional to it if a scale is introduced).</p
><p
xml:space="preserve"
>Vq = Vp^2</p
></section
></behaviordef
><behaviordef
name="normalized-square-law"
><label
key="normalized-square-law"
set="acnbase.lset"
></label
><refines
name="nonlin-squareLaw"
set="acnbase.bset"
></refines
><refines
name="normalized-nonlinearity"
set="acnbase.bset"
></refines
><section
><hd
>Normalized “Square-law" response curve</hd
><p
>This differs from the generic nonlin-squareLaw behavior in that it is normalized to give a full scale output of 1.0 over the range of the property from 0 to max.</p
><p
>The property range and scaling method is defined in normalized-nonlinearity behavior from which this derives.</p
><p
xml:space="preserve"
>Vq = Vp^2/Vpmax^2</p
><p
>where Vpmax is the upper limit of the range (maximum of type if fullScale provided, explicit limit if unitScale provided).</p
></section
></behaviordef
><behaviordef
name="nonlin-S-curve"
><label
key="nonlin-S-curve"
set="acnbase.lset"
></label
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><refines
name="normalized-nonlinearity"
set="acnbase.bset"
></refines
><section
><hd
>Generic “S-curve” response curve</hd
><p
>This property changes with the quantity it represents according to an “S-curve”. This means that on an S-curve graph, the quantity is the y-axis value while the property value is the x-axis value.</p
><p
>The original s-curve derives from the response of a phase control dimmer where the phase angle varies linearly. For idealized phase control, however, many variants on this curve are commonly referred to as an S-curve or S-law and this behavior alone does not distinguish between them.</p
><p
>See normalized-nonlinearity behavior from which this derives for how scaling is applied.</p
></section
></behaviordef
><behaviordef
name="nonlin-S-curve-precise"
><label
key="nonlin-S-curve-precise"
set="acnbase.lset"
></label
><refines
name="nonlin-S-curve"
set="acnbase.bset"
></refines
><section
><hd
>Precise “S-curve” response curve</hd
><p
>As noted in nonlin-S-curve which this refines, there are many variants on the S-law or S-curve and the term is used very loosely.</p
><p
>This behavior specifies much more precisely that the transfer function approximates to that of an idealized phase control power controller applied to a sinusoidal input. This curve is given by the equation:</p
><p
xml:space="preserve"
>P = (θ - sin(2⋅θ)/2)/π   [in radians]
  or
P = (θ - sin(2⋅θ)/2)/180   [in degrees]</p
><p
>where P is the power output normalized to 1.0 at full power and θ (theta) is the phase angle ranging from 0 to π radians or 0 to 180°. This curve is expressed in power and should be applied as such.</p
><p
>For purposes of scaling the rules of normalized-nonlinearity behavior shall apply, with the value θ ranging over one half cycle (180° or π radians) for full-scale output.</p
><p
xml:space="preserve"
>Vq = (Vp - sin(2⋅Vp)/2)    ;Vp in half cycles</p
></section
></behaviordef
><behaviordef
name="nonlin-monotonic"
><label
key="nonlin-monotonic"
set="acnbase.lset"
></label
><refines
name="nonLinearity"
set="acnbase.bset"
></refines
><section
><hd
>Monotonic nonlinearity</hd
><p
>This behavior can be attached to any property where the designer is too lazy to work out what they are really doing and just know “it goes from min to max”.</p
><p
>Being monotonic tells the poor looser who has to deal with this shoddy equipment which way to turn the knob. See Wikipedia for definition of monotonic if you are struggling already.</p
><p
>Occasionally but rarely there are genuine reasons for using nonlin-monotonic.</p
></section
></behaviordef
><behaviordef
name="normalized-monotonic"
><label
key="normalized-monotonic"
set="acnbase.lset"
></label
><refines
name="nonlin-monotonic"
set="acnbase.bset"
></refines
><refines
name="normalized-nonlinearity"
set="acnbase.bset"
></refines
><section
><hd
>Monotonic normalized nonlinearity</hd
><p
>As the property value varies from min to max, the quantity it represents varies from 0 to 1.0 (subject to scaling) by some unspecified but monotionic function.</p
></section
></behaviordef
><!--Priorities--><behaviordef
name="priority"
><label
key="priority"
set="acnbase.lset"
></label
><refines
name="type.uint"
set="acnbase.bset"
></refines
><section
><hd
>Priority</hd
><p
>There are various schemes in common practice which require values to be assigned a priority. Examples range from prioritized access to resources to prioritized algorithms for combining inputs in a driver/driven scenario. This behavior assigns a priority in such schemes. In the absence of specification to the contrary in a refinement, the priority is assigned to the parent of this property. The effect of the priority must be identified either by the behavior description for the prioritized property or by a refinement of this behavior.</p
><p
>The value of a priority property is an unsigned integer. A priority of zero means “unused” or no priority at all and should normally mean no action. For non-zero values, higher values have higher priority. The range of priorities permitted can be declared by limits in the normal way or shall be the full range of the underlying integer (1..255 for 1 octet etc.).</p
></section
></behaviordef
><!--Driven properties and drivers--><behaviordef
name="driven"
><label
key="driven"
set="acnbase.lset"
></label
><section
><hd
>Driven Property</hd
><p
>The property value is derived in some direct way from the value(s) of one or more of its children which shall have "driver" behavior (or refinements thereof). Refinements of this behavior define the particular relationship between the contibutary properties and the derived value.</p
><p
>A driven property shall have at least one child property with “driver” behavior.</p
><p
>If a driven property is writable (write="true"), then it may be written directly as well as implicitly by changing one of its driver properties and the latest action will apply until overriden by any subsequent action (latest takes precedence). Note that in some many cases a driver property may be constantly changing (e.g. when governed by a timer sub-property) and in this case writing to the driven property directly may be immediately overridden or cause unexpected effects. It is preferable in these circumstances to make the primary property read-only and use a selector to choose between either a direct input or a more complex driven input.</p
><p
>The definition of particular driven relationships shall be expressed by refinement of driven behavior and not of driver behavior except where different drivers perform different roles in generating the driven property value.</p
><p
>It is legal to have a driven property which has neither write nor read access - it's value is entirely determined by the particular driven relationship and the values or the driver children and in this cases, the driven property need not have a network (DMP) address. It is however normally preferable for a driven property to have read only access.</p
><p
>Driver properties in many cases must be the same type as the driven property. However, this is not always true so refinements must specify any constraints on the types of driven and driver properties.</p
><p
>In cases where an unspecified number of driver properties contribute in an equal way to the driven value (see example below), the driver property may be declared as an array.</p
><section
><hd
>example</hd
><p
>A driven property's value is the sum of the values of a number of driver sub-properties. When any of the driver properties changes the driven property is implicitly updated.</p
></section
></section
></behaviordef
><behaviordef
name="driver"
><label
key="driver"
set="acnbase.lset"
></label
><section
><hd
>Driver Property</hd
><p
>A driver property provides the input to a driven property (see “driven” behavior). A driver property may only occur as the child of a driven property.</p
><p
>When the driver property is changed its parent property is implicitly recalculated and updated according to the relationship defined by the particular refinement of driver behavior.</p
></section
></behaviordef
><behaviordef
name="target"
><label
key="target"
set="acnbase.lset"
></label
><refines
name="driver"
set="acnbase.bset"
></refines
><section
><hd
>Target Property</hd
><p
>One of the most fundamental types in control, a target directly represents the desired value of its parent and is the simplest driver property.</p
><p
>A target property should be used whenever a direct input of desired value is required but there are other constraining properties which may restrict or modify the value of the parent such that the target value cannot be achieved. The case where the actual value and the target differ may be static (e.g. where the target is outside the imits of the parent value) or dynamic (e.g. where the parent property cannot or should not change instantaneously).</p
><p
>As stated in section “Semantics of Property Values - Driven Properties” of DDL the primary property representing a physical feature in a device gives the actual current value of that feature. In most cases this current value must be a driven value and the writable driver property representing the desired state is a driver.</p
><p
>If any of the units, scaling, limits and datatype of a target property are not explicitly declared, they are defined to be identical to those of the property which it drives. The driven property must explicitly declare these. Declaring values for units, scaling, limits and datatype which are different from those of the driven property is strongly discouraged unless there is additional processing which is explicitly declared (as with non-linear or combinatorial drivers).</p
></section
></behaviordef
><behaviordef
name="unattainableAction"
><label
key="unattainableAction"
set="acnbase.lset"
></label
><refines
name="enumeration"
set="acnbase.bset"
></refines
><section
><hd
>Unattainable Action</hd
><p
>Where an target value for a driven property is set to a value which is unattainable because of constraints such as limits on range, time, speed or resolution, the device may reject the request, or may take other action such as making a best efforts attempt to achieve the target.</p
><p
>Refinements of this behavior allow the device to reveal what it's action will be or allow the controller to set the desired behavior.</p
></section
></behaviordef
><behaviordef
name="currentTarget"
><label
key="currentTarget"
set="acnbase.lset"
></label
><refines
name="target"
set="acnbase.bset"
></refines
><section
><hd
>Current Target</hd
><p
>This property shall be readable and not writable. it shall be the child of a driven property and shall express the current value which the driven property is at or is progressing towards. If the drivers for that property generate a desired value which is not achievable (e.g. because it is outside limits of size or speed) then this property contains the desired value which is actually in use.</p
><p
>In cases where out of limits target values are simply rejected by the device then the Current Target property shall contain the last target value which was accepted as being within limits.</p
><p
>In cases where out-of limits values are accepted and the device attempts to reach the closest value to the target which is within limits or follows some other rule then this property returns the value that the parent property+ is at or proceeding towards.</p
><p
>Unless explicitly stated, its type, size and scaling shall be the same as for it's parent property.</p
></section
></behaviordef
><behaviordef
name="trippable"
><label
key="trippable"
set="acnbase.lset"
></label
><refines
name="volatile"
set="acnbase.bset"
></refines
><section
><hd
>Trippable property</hd
><p
>This behavior takes its name from the common term “trip switch”. A property with this behavior is writable in the normal way but may revert to a different value owing to some action within the device (the action “trips” the switch).</p
><section
><hd
>Example – Initialization</hd
><p
>A initializationBool property represents the initialization state of a device or section of a device – true for initialized, false for uninitialized. In many devices this typically represents the process of internal calibration and initialization.</p
><p
>The actual state value will typically be driven by a target which expresses the desired state – turn the target off and the device duly goes into its uninitialized state, turn the target on again and it will execute its initialization routine. However, should the device detect a calibration error during operation, the user may prefer that it does not immediately attempt to re-calibrate itself as would be indicated by a target of “initialized”. Alternatively, the device may be set to give up after a number of unsuccessful calibration attempts.</p
><p
>In all these cases, if the target value is trippable, then the calibration error, or failure can trip the target property into the false state.</p
></section
><p
>The state to which the trippable property changes and the action causing that change may be expressed in sub-properties or refined behaviors.</p
></section
></behaviordef
><!--Numeric bounds and limits on property values--><behaviordef
name="limit"
><label
key="limit"
set="acnbase.lset"
></label
><section
><hd
>Limit</hd
><p
>Applies a bound to the values a parent property may take.</p
><p
>Limits refined from this behavior apply directly to the the numeric value of the property rather than to the quantity represented by that value. For properties whose values are scalars this makes no difference, but limits may also be applied to enumerations, cyclic properties and other encodings and types. Individual behaviors discuss how to interpret limits which are applied, but in general the property must be represented by a type which can be interpreted numerically for the purposes of expressing limits. This use of numeric comparison for defining limits does not imply that the property can be interpreted numerically for other purposes.</p
><p
>The requirement for numeric comparison to describe limits means that for values of more than one octet, the encoding of the parent (limited) property must be one for which significance or byte ordering are clearly defined.</p
></section
></behaviordef
><behaviordef
name="limitMinExc"
><label
key="limitMinExc"
set="acnbase.lset"
></label
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Minimum Exclusive Limit</hd
><p
>Parent property’s value must be greater than this property’s value.</p
><p
xml:space="preserve"
>propertyValue &gt; limitMinExc</p
></section
></behaviordef
><behaviordef
name="limitMinInc"
><label
key="limitMinInc"
set="acnbase.lset"
></label
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Minimum Inclusive Limit</hd
><p
>Parent property’s value must be greater than or equal to this property’s value.</p
><p
xml:space="preserve"
>propertyValue ≥ limitMinInc</p
></section
></behaviordef
><behaviordef
name="limitMaxExc"
><label
key="limitMaxExc"
set="acnbase.lset"
></label
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Maximum Exclusive Limit</hd
><p
>Parent property’s value must be less than this property’s value</p
><p
xml:space="preserve"
>propertyValue &lt; limitMaxExc</p
></section
></behaviordef
><behaviordef
name="limitMaxInc"
><label
key="limitMaxInc"
set="acnbase.lset"
></label
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Maximum Inclusive Limit</hd
><p
>Parent property’s value must be less than or equal to this property’s value</p
><p
xml:space="preserve"
>propertyValue ≤ limitMaxInc</p
></section
></behaviordef
><behaviordef
name="limitByAccess"
><label
key="limitByAccess"
set="acnbase.lset"
></label
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Limit Which Differ According to Access Method</hd
><p
>By default, limit properties assume that the limit applies, however the limited property gets its value. However, for properties which may obtain their values by multiple means – for example by local front panel setting, by DMP writes, by other protocols and so on – there can be cases where different limits are imposed for different accesss methods. For example, a property may allow a wider range of values to be set using local override than can be set using normal network operation.</p
><p
>This behavior shall be an additional behavior on another limit behavior on the same property. It indicates that this limit applies to a particular access method only, and that other limits may apply for other access methods. Therefore, values which are apparently out-of-limits may be read back from the property if they are set with one method and interrogated with another.</p
><p
>Possible distinctions between limits covered by this behavior include not only limits which differ from one access method to another but also those which differ according to access context.</p
><p
>The access method referred to shall be defined by refinement of this behavior and optionally by additional sub-properties defined by those refinements.</p
><p
>Limits qualified by this behavior apply to operations which change a property value only (e.g. network writes) and not to operations which interrogate a property value (e.g. network reads) which should read the actual value within its absolute limits.</p
><p
>When one or more limitByAccess properties are present, any limits expressed on the same property without the limitByAccess qualification describe the absolute limit which the property value may take, irrespective of access method. They also therefore applies to any access methods for which no explicit limitByAccess is given.</p
><p
>Since DDL is directed at describing control via the access protocol (or possible via multiple access protocols) and provides little information on other ways to control a device, there is no way to know what other access methods may be present (but see volatile behavior) so if no such unqualified limits is given, then the property must be presumed to be limited only by its underlying datatype and size.</p
></section
></behaviordef
><behaviordef
name="limitNetWrite"
><label
key="limitNetWrite"
set="acnbase.lset"
></label
><refines
name="limitByAccess"
set="acnbase.bset"
></refines
><section
><hd
>Limit on Network Settable Value</hd
><p
>The presence of this behavior on the same property as another limit behavior (see limitByAccess), implies that the given limit applies to network writes using the access protocol of the description only and that values which are set by other means (front panel, other protocols etc.) may extend further.</p
></section
></behaviordef
><!--Time or rate limited changes

There are many instances where a device feature either cannot change instantaneously or
where control over the rate of change is required.

These are expressed as driven properties where the drivers are a desired ?target? value
and various modifiers over time or rate of change apply.

As with many properties, time and rate modifiers can be adjustable control parameters
or may simply express the physical limitations of the system. --><behaviordef
name="relativeTarget"
><label
key="relativeTarget"
set="acnbase.lset"
></label
><refines
name="target"
set="acnbase.bset"
></refines
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Relative Target</hd
><p
>This is similar to a target property but expresses a required change rather than an absolute target. Refinements elaborate the specifics.</p
><p
>For a scalar property, a relative move is constrained to be no larger than the range of that property. For a cyclic property, a relative target may encompass multiple cycles of the property and the device is required to keep track of these to ensure the correct relative move. This means that the relative target property may require a larger type than its parent.</p
></section
></behaviordef
><behaviordef
name="moveTarget"
><label
key="moveTarget"
set="acnbase.lset"
></label
><refines
name="relativeTarget"
set="acnbase.bset"
></refines
><section
><hd
>Move Target</hd
><p
>This property implements a move or change relative to the last TARGET position. That is it moves the target by the amount given.</p
><p
>In the case of a cyclic property, which is currently changing through several cycles to reach the target (see Relative Target), any full cycle changes not completed shall be lost as a result of a change to the move target property.</p
></section
></behaviordef
><behaviordef
name="moveRelative"
><label
key="moveRelative"
set="acnbase.lset"
></label
><refines
name="relativeTarget"
set="acnbase.bset"
></refines
><section
><hd
>Move Relative</hd
><p
>This property implements a move or change relative to the current actual position. It sets a new target relative to the actual position without reference to current targets.</p
><p
>The position is taken at the point the property change is acted upon. In most cases this is when the set_property command is received and processed, but this may be modified for example by an associated property with atTime behavior, in which case the change shall be relative to the actual position when the "atTime" triggers the change.</p
></section
></behaviordef
><!--Control over timing of changes and rate of movement--><behaviordef
name="actionTimer"
><label
key="actionTimer"
set="acnbase.lset"
></label
><refines
name="timePeriod"
set="acnbase.bset"
></refines
><section
><hd
>Action Timer</hd
><p
>This behavior underlies a range of behaviors which apply times to actions. The value of an action timer is the time to be applied and retains the value set. The time period begins at some trigger action which might be the time the timer is set or might be another event such as the setting of another property or expiry of another timer.</p
><p
>An action action timer may have a child progressTimer property which provides instantaneous readback of the progress of the timer – see progressTimer. The progressTimer sub-property inherits the units and scale of its actionTimer parent unless other scaling is explicitly specified.</p
></section
></behaviordef
><behaviordef
name="targetTimer"
><label
key="targetTimer"
set="acnbase.lset"
></label
><refines
name="actionTimer"
set="acnbase.bset"
></refines
><refines
name="driver"
set="acnbase.bset"
></refines
><refines
name="atomicLoad"
set="acnbase.bset"
></refines
><section
><hd
>Target Timer</hd
><p
>A property which applies a time to any change in the parent property. Works in conjunction with an atomic group which must have a master property. Except in special cases the atomicMaster property should be the target driver property.</p
><p
>Whenever the target property is set, the feature it controls changes from its instantaneous value at the time the set command applies, to the set target value over the time given by this target timer property.</p
><p
>The targetTimer value does not revert to zero after use but shall be retained (subject to other methods to change it such as local actions, other protocols, power cycling etc.) and applied to all subsequent changes to the target until a new value is set.</p
><p
>In accordance with the semantics of atomic groups, the targetTimer value will be “latched” when the master (usually the target) value is set which means that subsequent changes to the targetTimer do not affect the move in progress. However, a move can be retimed by first setting the new timer and then re-setting the target to the same value.</p
><p
>If the targetTimer property is set to zero then control of the target behaves as though the targetTimer was not there at all.</p
><p
>If a delayTime or atTime property is present in the same atomic group as the targetTimer, then the targetTimer shall not commence until the atTime is reached or the delayTime expires, whichever happens later. Note though that the setting of the atomicMaster value is the action which latches the targetTimer value, not the triggering by atTime or delayTime.</p
><p
>e.g. in lighting systems this can apply a timed fade with a single command.</p
></section
></behaviordef
><behaviordef
name="delayTime"
><label
key="delayTime"
set="acnbase.lset"
></label
><refines
name="actionTimer"
set="acnbase.bset"
></refines
><refines
name="driver"
set="acnbase.bset"
></refines
><refines
name="atomicLoad"
set="acnbase.bset"
></refines
><section
><hd
>Delay Time</hd
><p
>This property defines a delay before a change to a target value shall be applied is applied to the associated driven property. It works in conjunction with an atomic group which must have a master property. Except in special cases the atomicMaster property should be the target driver property.</p
><p
>Whenever the atomicMaster property is set, the delay value must be “latched” and only at the expiry of the delay does the normal action take effect.</p
><p
>The delaytime value does not revert to zero after use but shall be retained (subject to other methods to change it such as local actions, other protocols, power cycling etc.) and applied to all subsequent changes to the target until a new value is set.</p
><p
>If the atomicMaster value is changed before expiry of the delay, then a new delay is started and the earlier action will never take effect.</p
><p
>In accordance with the semantics of atomic groups, the delay may be changed any number of times after the atomicMaster has been set with no effect until the atomicMaster is set again.</p
><p
>If an atTime property is present in the same atomic group as the delayTime, then the delayTime shall not commence until the atTime is reached. Note though that within a single atomic group, the setting of the atomicMaster value is the action which latches the delayTime value, and not the triggering of the delay by atTime.</p
><p
>If the delayTime property is set to zero then control of the target behaves as though the delayTime was not there at all.</p
></section
></behaviordef
><behaviordef
name="atTime"
><label
key="atTime"
set="acnbase.lset"
></label
><refines
name="timePoint"
set="acnbase.bset"
></refines
><refines
name="atomicTrigger"
set="acnbase.bset"
></refines
><refines
name="syncGroupMember"
set="acnbase.bset"
></refines
><section
><hd
>At Time</hd
><p
>This property defines a time at which a change to the to the associated driven property shall apply. The arrival of the time point is the trigger for a synchronization group consisting of all properties which identify this property as the synchronization trigger.</p
><p
>If the timePoint is in the future changes made to group member properties will take no effect until the timePoint is reached. At that time they will be triggered as though they were set at that time.</p
><p
>Changes made to member properties before the timePoint are not queued but simply overwrite each other. When timePoint arrives, whatever value was last set for the member property shall take effect.</p
><p
>If the atTime is set to a value in the past, the group is unsynchronized and further changes to member values shall take immediate effect.</p
><p
>An atTime property may be used to synchronise actions across multiple properties and if a system wide time reference is established, across multiple devices. The time reference used by atTime must be specified elsewhere for example see ACN-epi25.</p
></section
></behaviordef
><behaviordef
name="rate"
><label
key="rate"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><refines
name="driver"
set="acnbase.bset"
></refines
><section
><hd
>Rate</hd
><p
>An abstract behavior inidcating that the property imposes or expresses a restriction on the rate of change of its parent property with respect to time (speed, acceleration, etc.) Rate is an abstract behavior and refinements declare more specific versions..</p
><p
>Refined behaviors may represent restrictions on first or higher derivatives of the parent or may represent more complex change profile restrictions.</p
><p
>A rate which is constant is simply a declaration of the capability of the device.</p
><section
><hd
>Note on Order Sensitivity</hd
><p
>Rate properties may have order sensitivity with their associated target properties depending on whether the timing changes caused by re-ordering the accesses to the two are significant. (See accessOrder behavior, Note: Time of processing).</p
><p
>If rate a rate property is order sensitive with another property, then the creation of an atomic load group should be considered.</p
></section
><section
><hd
>Units of Rate Properties</hd
><p
>A rate property may be specified with no units value. in this case, the units shall be defined to be the appropriate derivative of the units of the driven property with respect to time measured in seconds and is independent of the scaling of the parent. For example if the driven property is declared with units of metres and unitscale of 0.001 (1mm), the units of a rate2nd driver shall be m.s^-2 (metres per second per second) unless other units are explicitly provided. A scale property may then be specified as normal representing the scaling of the rate property expressed in these default units.</p
><p
>As another example, if a driven property represents angle (e.g. tilt, pan, rotation etc.) in degrees, a rate1stLimit driver defaults, to degrees per second, while if the driven property has declared units of revolutions (not reccommended), the rate1stLimit driver would default to revolutions per second.</p
></section
></section
></behaviordef
><behaviordef
name="rate1st"
><label
key="rate1st"
set="acnbase.lset"
></label
><refines
name="rate"
set="acnbase.bset"
></refines
><section
><hd
>First Derivative Rate</hd
><p
>This property imposes a rate of change on its parent property with respect to time. It is preferable to use rate1stLimit over rate1st in devices which need to be tolerant and adapt to less precise control or where control is primarily over position and speed is a secondary concern.</p
><p
>rate1st is preferred over rate1stLimit in devices where control of speed is of primary importance over control of position or where very tight control over speed profiles is required.</p
></section
></behaviordef
><behaviordef
name="rate1stLimit"
><label
key="rate1stLimit"
set="acnbase.lset"
></label
><refines
name="rate"
set="acnbase.bset"
></refines
><section
><hd
>First Derivative Rate Limit</hd
><p
>This property imposes a limit on the first order rate of change of the property it drives (usually its parent) with respect to time (a speed limit).</p
><p
>It should be used to declare or control the maximum speed of a mechanical system or to control the speed at which any property changes.</p
><p
>rate1stLimit does not constrain the rate of change to be equal to this property, merely less than or equal to. The actual rate of change may be below this limit because of mechanical constraints, or application of other behaviors such as a timer (see _timer) or by acceleration and deceleration being too low to reach the full rate during a short move.</p
><p
>It is generally preferable to use rate1stLimit over rate1st in devices which need to be tolerant and adapt to less precise control or where control is primarily over position and speed is a secondary concern.</p
><p
>rate1st is preferred over rate1stLimit in devices where control of speed is of primary importance over control of position, or where very tight control over speed profiles is required.</p
></section
></behaviordef
><behaviordef
name="rate2nd"
><label
key="rate2nd"
set="acnbase.lset"
></label
><refines
name="rate"
set="acnbase.bset"
></refines
><section
><hd
>Second Derivative Rate</hd
><p
>This property imposes a value on the second order rate of change of its parent with respect to time (an acceleration).</p
><p
>It should be used to declare or control the acceleration of a mechanical system or the second order rate of change of any property.</p
><p
>It is generally preferable to use rate2ndLimit over rate2nd where combined control over position, speed and acceleration are required. rate2nd is preferred over rate2ndLimit in devices where control of acceleration is of primary importance or where very tight control over speed and acceleration profiles is required.</p
><p
>Note that it is not possible to impose a fixed non-zero value of a rate2nd property at the same time as a fixed rate1st property for the same parent. (a fixed speed precludes non-zero acceleration) - use a rate2ndLimit instead.</p
></section
></behaviordef
><behaviordef
name="rate2ndLimit"
><label
key="rate2ndLimit"
set="acnbase.lset"
></label
><refines
name="rate"
set="acnbase.bset"
></refines
><section
><hd
>Second Derivative Rate Limit</hd
><p
>This property imposes a limit on the second order rate of change of its parent with respect to time (a maximum acceleration).</p
><p
>It should be used to declare or control the acceleration of a mechanical system or to control the speed at which any property changes.</p
><p
>A rate2ndLimit with a negative value implies a deceleration limit.</p
><p
>rate2ndLimit does not constrain the acceleration to be equal to this property, merely less than or equal to. The actual acceleration may be below this limit because of mechanical constraints, or application of other behaviors such as a rate1stLimit.</p
><p
>It is generally preferable to use rate2ndLimit over rate2nd where combined control over position, speed and acceleration are required. rate2nd is preferred over rate2ndLimit in devices where control of acceleration is of primary importance or where very tight control over speed and acceleration profiles is required.</p
></section
></behaviordef
><behaviordef
name="suspend"
><label
key="suspend"
set="acnbase.lset"
></label
><refines
name="boolean"
set="acnbase.bset"
></refines
><section
><hd
>Suspend</hd
><p
>For properties which cannot change instantaneously – e.g. those which have time or rate sub-properties operating or which have physical limitations on their rate of change – a suspend sub-property allows pausing of the current move.</p
><p
>When this property is set to true, change of the function represented by the parent property is halted (subject to time taken to come to rest). Suspension also pauses any associated timer actions, progressIndicators etc.</p
><p
>When this property is reset to false, the move will continue.</p
><p
>If the target is updated while suspend is true, the change will not take effect until the suspend property is set to false.</p
></section
></behaviordef
><behaviordef
name="progressIndicator"
><label
key="progressIndicator"
set="acnbase.lset"
></label
><refines
name="volatile"
set="acnbase.bset"
></refines
><section
><hd
>Progress Indicator</hd
><p
>This property is an indication of the progress of a property towards it's target. In many cases, there is an unspecified delay between a target being set and the driven property reaching that target. Often the device itself cannot predict the delay accurately but can nevertheless indicate that progress is under way.</p
><p
>The value of a progressIndicator property should change linearly with time (unless an explicit non-linearity is indicated) towards completion. However, it is recognised that wide variation may occur in cases where the time expected is unpredictable. For this reason a progress indicator does not necessarily have timer behavior.</p
><p
>Refinements define specific meanings for values.</p
><section
><hd
>Example – device initialization</hd
><p
>Many devices must perform an initialization routine before they can function – either automatically or on command. A boolInitState property reflects this as two simple states “initiailized” or “uninitiailized”. AprogressIndicator property which is a child of the initialization state can provide useful feedback on the progress of the initialization process.</p
></section
></section
></behaviordef
><behaviordef
name="progressCounter"
><label
key="progressCounter"
set="acnbase.lset"
></label
><refines
name="progressIndicator"
set="acnbase.bset"
></refines
><refines
name="type.uint"
set="acnbase.bset"
></refines
><section
><hd
>Progress Counter</hd
><p
>This property is a progressIndicator represented as an unsigned integer which counts upward from zero towards a maximum as action progresses, reaching the maximum at completion. It is unscaled and although in keeping with progressIndicator behavior it should aim for a linear change with time, this is not certain.</p
><p
>The maximum value of a progressCounter may be explicitly provided by a sub-property with limit behavior or may default to the maximum expressible by the size of the property.</p
><p
>The initial value when the action monitored is triggered may be greater than zero where the device is able to tell from initial conditions that progress will complete more quickly than the maximum.</p
></section
></behaviordef
><behaviordef
name="progressTimer"
><label
key="progressTimer"
set="acnbase.lset"
></label
><refines
name="progressIndicator"
set="acnbase.bset"
></refines
><refines
name="countdownTime"
set="acnbase.bset"
></refines
><section
><hd
>Progress Timer</hd
><p
>A progressTimer is a progressIndicator property which accurately reflects a known time to completion. As its refinement of countdownTime indicates, it counts down from an initial value to zero, at which time the operation in progress shall be completed.</p
><p
>A progressTimer shall not be a writable property, but its value may obviously be driven by other properties or network actions such as changing the target value (see driven and target behaviors), suspending an action (see suspend behavior) etc.</p
><p
>The initial value of a progressTimer property (the value it counts down from) will generally depend on initial conditions.</p
><p
>A progressTimer must specify its scale and units in order to be useful. This may either be explicitly via scale and units properties or implicitly by inheritance from some sort of timer parent (e.g. see actionTimer).</p
></section
></behaviordef
><!--Drivers combining multiple inputs--><behaviordef
name="maxDriven"
><label
key="maxDriven"
set="acnbase.lset"
></label
><refines
name="driven"
set="acnbase.bset"
></refines
><section
><hd
>Property Driven as Maximum</hd
><p
>The value of this property is the maximum of all it's driver properties. The maxDriven and all contained driver properties shall have non-cyclic ordered types.</p
><p
>Note: in lighting control this behavior is frequently called “highest takes precedence” or “HTP”.</p
></section
></behaviordef
><behaviordef
name="minDriven"
><label
key="minDriven"
set="acnbase.lset"
></label
><refines
name="driven"
set="acnbase.bset"
></refines
><section
><hd
>Property Driven as Minimum</hd
><p
>The value of this property is the minimum of all it's driver properties. The minDriven and all contained driver properties shall have non-cyclic ordered types.</p
></section
></behaviordef
><behaviordef
name="drivenOr"
><label
key="drivenOr"
set="acnbase.lset"
></label
><refines
name="driven"
set="acnbase.bset"
></refines
><refines
name="boolean"
set="acnbase.bset"
></refines
><section
><hd
>Property Driven as Logical Or of its Drivers</hd
><p
>The value of this property is the logical OR of all it's driver properties. The drivenOr property is of boolean type. Driver properties may be of any ordered type with a zero value interpreted as false and a non-zero value as true.</p
></section
></behaviordef
><behaviordef
name="drivenAnd"
><label
key="drivenAnd"
set="acnbase.lset"
></label
><refines
name="driven"
set="acnbase.bset"
></refines
><refines
name="boolean"
set="acnbase.bset"
></refines
><section
><hd
>Property Driven as Logical And of its Drivers</hd
><p
>The value of this property is the logical AND of all it's driver properties. The drivenAnd property is of boolean type. Driver properties may be of any ordered type with a zero value interpreted as false and a non-zero value as true.</p
></section
></behaviordef
><behaviordef
name="maxDrivenPrioritized"
><label
key="maxDrivenPrioritized"
set="acnbase.lset"
></label
><refines
name="maxDriven"
set="acnbase.bset"
></refines
><section
><hd
>Prioritized Maximum Value</hd
><p
>This is a variant of the maxDriven combination scheme in which the driver properties are prioritized.</p
><p
>Each driver property contributing to this driven value shall be assigned a priority (by default the priority property is a child of the driver property – see priority behavior). Of all the drivers, only those with equal highest priority are considered. The driven value is then the highest of those driver values. A drive with a priority of zero is never considered (its value is taken as zero) even if there are no drivers with non-zero priority.</p
><section
><hd
>Examples</hd
><p
>Consider 4 driver properties A..D with associated priorities Pa..Pd</p
><p
xml:space="preserve"
>  A,pA      B,Pb      C,Pc      D,Pd  ⇒  Driven value
 99,8      50,6    1000,6       3,0   ⇒  99    ;A has highest priority
 99,6      50,6    1000,6       3,0   ⇒  1000  ;A, B, C have equal highest
                                               ;priority, C has highest value
 99,0      50,0    1000,0       3,0   ⇒  0     ;all drivers have priority 0</p
></section
></section
></behaviordef
><!--Geometrical properties expressing length, space, position,
direction, physical movement, size etc in 2 and 3 dimensions.--><!--Groups defining basic spatial concepts--><behaviordef
name="spatialCoordinate"
><label
key="spatialCoordinate"
set="acnbase.lset"
></label
><refines
name="multidimensionalGroup"
set="acnbase.bset"
></refines
><section
><hd
>Spatial Coordinate</hd
><p
>A spatial coordinate - number of dimensions and coordinate system are specified in refined behaviours</p
><p
>If a spatialCoordinate group contains a datum or coordinate reference property then this defines the default reference for all properties within the group unless their behaviors specify otherwise.</p
></section
></behaviordef
><behaviordef
name="ordinate"
><label
key="ordinate"
set="acnbase.lset"
></label
><section
><hd
>Ordinate</hd
><p
>Any multidimensional measurement or quantity is expressed in terms of a set of ordinates. Each property of a multidimensional group which contributes directly to specifying the multidimensional quantity must be an ordinate (or refinement of it).</p
><p
>Any property representing an ordinate potentially also represents a shift of datum to which other properties may be referenced. For example an ordinate representing a displacement in the x-direction might provide a reference from which another property's x values are measured. This shift in reference is implicit to any ordinate property and can be used as a datum. For example, see datumProperty behavior.</p
><p
>Most conventional multidimensional systems use just sufficient ordinates to define the required measurement. For example in three dimensional space there are many coordinate systems (rectangular, polar etc. but they nearly all have exactly three ordinates. However, some systems are in use which have more ordinates than strictly necessary – they have redundancy.</p
><p
>A common example of redundant ordinates occurs in color control. While color is often expressed in three axes (e.g. hue, saturation and brightness, or red, green and blue intensities), nevertheless luminaires for projecting colored light commonly include intensity in addition to three separate colour controls (e.g. intensity, yellow, magenta, cyan) and printers use cyan, magenta, yellow and black. These occur partly for physical reasons (due to the way the luminaire works or the characterisitics available in printing inks) and partly because expression of color in three axes is an inexact science based the perceptual response of the eye which varies widely between individuals.</p
></section
></behaviordef
><behaviordef
name="datum"
><label
key="datum"
set="acnbase.lset"
></label
><section
><hd
>Datum</hd
><p
>A datum is a point or reference (an origin) from which other measurements are made. Refinements include coordinate origins, reference surfaces etc. A datum may apply to a simple measure quantity or may specify the origin point and orientation for multidimensional axes.</p
></section
><section
><hd
>Implicit Datum</hd
><p
>Many properties may carry an inplicit datum, this may be a “natural” zero inherent because the quantity being measured is an absolute quantity, it may be a well established convention or it may be a part of the definition of a behavior. In these cases, no explicit datum property need be provided.</p
><section
><hd
>Absolute or “Natural” Zero</hd
><p
>A measure of an absolute quantity has a “natural” zero point. For example, a measure representing the volume of a liquid has an implicit datum at zero volume. Likewise a measure of the electrical current in a wire is an absolute quantity. These must be clear from the units and other behaviors present.</p
><p
>Often units alone are not enough to establish a “natural” zero and additional behaviors are required to unambiguously define whether these are absolute measures or not. For example, the thickness of a sheet of steel is an absolute quantity and can be measured in millimetres, but the position of a probe along a slider is also measured in millimetres yet is relative to an arbitrary origin. Similarly pressure measurements may be absolute but are frequently made relative to the prevailing atmospheric pressure (gauge pressure).</p
></section
><section
><hd
>Conventional Origin</hd
><p
>Many measures carry an implicit convention for a zero point. Examples include compass bearings (with zero at due north by convention) and temperatures measured in °C where zero is an arbitrary point but is part of the definition of the unit.</p
><p
>Mathematics also defines conventions for many spatial measurements and these must be observed. For example, in two dimensional space, where an xy-plane is established, rotation is always positive in the anticlockwise direction (from x to y). This convention harmonises with the extension to three dimensions where the direction follows a right hand rule of rotation about the z axis.</p
></section
><section
><hd
>Origin Defined by Behavior</hd
><p
>Certain behaviors imply an origin or zero point for purposes of formalising the definition of common practices. For example, while in polar coordinates θ is conventionally measured away from the z-axis, in other prevalent systems such as altitude, azimuth it is measured away from the xy-plane. i.e.</p
><p
xml:space="preserve"
>azimuth = 90° - θ</p
><p
>A polar ordinate behavior explicitly defined to take the xy-plane as its origin can then be used for azimuth and any other polar system using the same convention.</p
></section
></section
><section
><hd
>Multiple Datum Properties</hd
><p
>It is occasionally convenient to specify a datum, not as a single property but using a chain of properties each of which references another. Where multiple datum properties occur within the same context they accumulate strictly in the order in which they are declared.</p
></section
><section
><hd
>Moving Datum</hd
><p
>If a property representing a part of a device references a datum which is variable (because it depends on variable network properties) then it follows that this part of the device changes with the datum relative to other frames of reference. For example, if a property representing a physical orientation has a variable network value indicating that its physical orientation changes or can be changed, then any properties which reference that property as a datum will move with it.</p
></section
></behaviordef
><behaviordef
name="localDatum"
><label
key="localDatum"
set="acnbase.lset"
></label
><refines
name="datum"
set="acnbase.bset"
></refines
><section
><hd
>Local Datum</hd
><p
>A localDatum property defines the reference position and orientation for a subsection of a device as related to any explicit of implicit datum which would otherwise apply – it may apply to one or more properties. It directly defines the reference position applying to its parent property and by inheritance, to any children of the parent which inherit their reference from the parent.</p
><p
>It is common for control quantities to be offset or transformed from the common units which would describe the controlled quantity. This is commonly done in order to express the range available in simpler terms.</p
><p
>For example, a pressure sensor built to detect atmospheric pressure at sea level need only cover a range from around 80kPa to 110kPa and could reasonably express its value relative to an 80kPa localDatum (i.e. 0 ⇒ 80kPa, 30 ⇒ 110kPa).</p
><p
>Similarly, a multidimensional positioner operating in one physical part of a device may use coordinates transformed relative to a global coordinate system operating within the device.</p
><p
>The value of a localDatum property defines the position and orientation of the parent's value origin in terms of whatever reference origin and units would apply were it not present.</p
><section
><hd
>Prevailing Datum for Measure of Local Datum</hd
><p
>If a prevailing reference origin already exists, which is established by an explicit datum then this shall be used as the origin to express the localDatum. Otherwise if the measure carries an implicit datum that shall apply (see datum behavior for discussion of implicit datum). If there is no prevailing datum either implicit or explicit then a localDatum is meaningless.</p
></section
><section
><hd
>Multiple Dimensions</hd
><p
>A localDatum must make sense in the dimensional structure of its parent. If its parent is a single dimensional measure, then the localDatum is also a measure and its value gives the offset of the parents measure relative to the prevailing zero point.</p
><p
>If the parent is a multidimensional group then the localDatum must also be a multidimensional group and express the transformation required to get from the prevailing frame of reference to the parent group's local frame of reference.</p
><p
>Note that many multidimensional transformations (e.g. translation operations in cartesian coordinates) may be expressed by applying a scalar localDatum individually to each axis. Datum shifts should always be applied at the lowest possible level that is, nearest the leaf nodes of the device model tree and therefore separate scalar localDatum properties should be used in preference to a single combined multiaxis datum shift.</p
></section
><section
><hd
>See Also</hd
><p
>measureOffset</p
></section
></section
></behaviordef
><behaviordef
name="datumProperty"
><label
key="datumProperty"
set="acnbase.lset"
></label
><refines
name="localDatum"
set="acnbase.bset"
></refines
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Datum Property</hd
><p
>It is common, particularly in mechanical systems involving motion for one part of a mechanism to take its datum from another. This property identifies another property – usually another part of the mechanism – which provides the datum for its parent.</p
><section
><hd
>Example</hd
><p
>A common example is a pan/tilt style mounting (also known as azimuth/altitude, bearing/elevation etc.). The first axis in such a mechanism is fixed with respect to the device and can be described as a simple rotation about a fixed axis with respect to whatever datum prevails in the device (generally pan is rotation about the z-axis). The second axis in such a mechanism rotates with the first and so takes its datum from it. Tilt would normally be rotation about the x-axis as defined in the frame of reference which rotates with pan. In this case the pan property can carry a xml:id identifier and tilt would then have a datumProperty child which identifies the pan property as its datum.</p
><p
>Contrast this with a trackball or joystick where the axes of both angles of rotation are fixed with respect to the same frame of reference.</p
></section
></section
></behaviordef
><behaviordef
name="coordinateReference"
><label
key="coordinateReference"
set="acnbase.lset"
></label
><refines
name="datum"
set="acnbase.bset"
></refines
><section
><hd
>Coordinate Reference</hd
><p
>A property identifying the origin and orientation for a system of coordinates, directions etc.</p
><p
>Examples of refined behaviors could include a text property relating the coordinate system to the physical features of the equipment (e.g. refer to mounting holes etc) or to a more universal standard such as latitude and longitude.</p
></section
></behaviordef
><behaviordef
name="deviceDatum"
><label
key="deviceDatum"
set="acnbase.lset"
></label
><refines
name="coordinateReference"
set="acnbase.bset"
></refines
><section
><hd
>Device Datum</hd
><p
>This property provides a base coordinate reference for an entire piece of equipment.</p
><p
>A variety of mechanisms may operate for expressing such a datum. If an external coordinate system is present, a datum may refer to that. An alternative is to simply describe the physical orientation relative to the device itself to which internal coordinates are referenced.</p
></section
></behaviordef
><behaviordef
name="deviceDatumDescription"
><label
key="deviceDatumDescription"
set="acnbase.lset"
></label
><refines
name="deviceDatum"
set="acnbase.bset"
></refines
><refines
name="textString"
set="acnbase.bset"
></refines
><section
><hd
>Device Datum Description</hd
><p
>Much equipment has no knowledge of any external coordinate system but nevertheless requires some arbitrary reference point and orientation on which its own coordinate system is based. A deviceDatumDescription is a human readable text which identifies this reference point to a user.</p
><p
>A datumDescription does not enable automatic calculation of relative position, but does provide a description accessible to the user which can aid in generation of a system-wide desctription. It facilitates manual setup of such systems by informing the operator where any physical measurements within the device are measured from. This enables relative positions of devices to be measured unambiguously.</p
><p
>The description must provide the user with sufficient information to accurately establish the internal coordinates used by the device relative to some external coordinate system. It should unambiguously identify a real feature which is accessible externally on the device from which measurements can be made – for example a mounting point. The description must provide a fixed reference for as many dimensions as are related to it.</p
></section
></behaviordef
><!--Individual dimensional measures --><behaviordef
name="length"
><label
key="length"
set="acnbase.lset"
></label
><refines
name="scalar"
set="acnbase.bset"
></refines
><section
><hd
>Length</hd
><p
>A scalar representing a physical length. May be the length of a displacement or offset</p
></section
></behaviordef
><behaviordef
name="angle"
><label
key="angle"
set="acnbase.lset"
></label
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Angle</hd
><p
>Property represents an angle. May be an angular displacement, offset or subtended. Depending on its range and physical implementation, an angle may be a scalar or may be cyclic.</p
></section
></behaviordef
><behaviordef
name="orthogonalLength"
><label
key="orthogonalLength"
set="acnbase.lset"
></label
><refines
name="length"
set="acnbase.bset"
></refines
><refines
name="ordinate"
set="acnbase.bset"
></refines
><section
><hd
>Orthogonal Length</hd
><p
>A length representing a displacement in a direction orthogonal to all other orthogonalLengths in the same group.</p
></section
></behaviordef
><behaviordef
name="ordX"
><label
key="ordX"
set="acnbase.lset"
></label
><refines
name="orthogonalLength"
set="acnbase.bset"
></refines
><section
><hd
>X Ordinate</hd
><p
>A length specifying an offset in the X direction relative to whatever coordinate reference is in force.</p
></section
></behaviordef
><behaviordef
name="ordY"
><label
key="ordY"
set="acnbase.lset"
></label
><refines
name="orthogonalLength"
set="acnbase.bset"
></refines
><section
><hd
>Y Ordinate</hd
><p
>A length specifying an offset in the Y direction relative to whatever coordinate reference is in force.</p
></section
></behaviordef
><behaviordef
name="ordZ"
><label
key="ordZ"
set="acnbase.lset"
></label
><refines
name="orthogonalLength"
set="acnbase.bset"
></refines
><section
><hd
>Z Ordinate</hd
><p
>A length specifying an offset in the Z direction relative to whatever coordinate reference is in force.</p
></section
></behaviordef
><!--Polar coordinate measures--><behaviordef
name="polarOrdinate"
><label
key="polarOrdinate"
set="acnbase.lset"
></label
><refines
name="ordinate"
set="acnbase.bset"
></refines
><section
><hd
>Polar Ordinate</hd
><p
>In two dimensions, polar coordinates are fairly simple and are generally expressed as a radial length and a single angle. A single angle may also be used to express a direction.</p
><p
>In three dimensions, there are two common systems of polar coordinates used to define a point or direction (cylindrical and spherical) and there are a wide variety of variants particularly on the spherical system.</p
><p
>Polar ordinates are also used extensively both mathematically and in physically systems to define the orientation of an object in 3D space – the “six axis” positioning model is common – or to define the relationship between one frame of reference and another.</p
><p
>Polar ordinates encountered in DDL will often describe physical systems and must reflect the actual mechanics of those systems. An automated spotlight usually has two physical axes of rotation and its properties usually control those directly.</p
><p
>There are a plethora of different conventions in in common use. For simple polar direction these range from altitude/azimuth, declination/right ascension (both used in astronomy) to latitude/longitude in geography and navigation, distance/bearing/elevation in gunnery and a wide range of other fields, and pan/tilt used for cameras and automated lighting. For measurement of orientation, aeronautical engineers use pitch/yaw/roll while mathematicians use Euler angles for which there are many related but differing conventions.</p
><p
>Behaviors for three dimensional rotations are defined for rotation or angular displacement about the three major cartesian coordinate axes x, y, z (see angleX, angleY, angleZ). In all cases positive angle follows a right-hand rule: positive rotation about an axis is clockwise when viewed in the direction of increasing value for that axis.</p
><p
>Rather than define more behaviors for different rotation conventions and orders of application, the existing behaviors should be used with appropriate shifts of datum explicitly specified. Thus for azimuth/altitude (assuming that z is vertically upwards) . Azimuth should be expressed as angleZ while altitude is then angleX with a datum which rotating with azimuth.</p
><p
xml:space="preserve"
>&lt;!--
   datum at start has been set with origin at intersection
   of azimuth and altitude axes
--&gt;
&lt;property valuetype="NULL"&gt;
  &lt;behavior name="direction3D"/&gt;
  &lt;property xml:id="azimuth" valuetype="network"&gt;
    &lt;label&gt;Azimuth Angle&lt;/label&gt;
    &lt;behavior name="angleZ" set="acnbase.bset"/&gt;
    &lt;protocol name="ESTA.DMP"&gt; ... &lt;/protocol&gt;
  &lt;/property&gt;
  &lt;property valuetype="network"&gt;
    &lt;label&gt;Altitude Angle&lt;/label&gt;
    &lt;behavior name="angleX" set="acnbase.bset"/&gt;
    &lt;protocol name="ESTA.DMP"&gt; ... &lt;/protocol&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;behavior name="datumProperty" set="acnbase.bset"/&gt;
      &lt;value type="string"&gt;azimuth&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
><p
>Following this convention, polar coordinates (r, θ, ϕ) (r, theta, phi) which follow the common convention of θ (theta) being in the xy-plane and ϕ (phi) being rotation away from the z-axis would have θ expressed as angleZ and ϕ as angleY with its datum rotating with θ.</p
><p
xml:space="preserve"
>&lt;property valuetype="NULL"&gt;
  &lt;behavior name="point3D"/&gt;
  &lt;property xml:id="theta" valuetype="network"&gt;
    &lt;label&gt;Theta coordinate&lt;/label&gt;
    &lt;behavior name="angleZ" set="acnbase.bset"/&gt;
    &lt;protocol name="ESTA.DMP"&gt; ... &lt;/protocol&gt;
  &lt;/property&gt;
  &lt;property xml:id="phi" valuetype="network"&gt;
    &lt;label&gt;Phi coordinate&lt;/label&gt;
    &lt;behavior name="angleY" set="acnbase.bset"/&gt;
    &lt;protocol name="ESTA.DMP"&gt; ... &lt;/protocol&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;behavior name="datumProperty" set="acnbase.bset"/&gt;
      &lt;value type="string"&gt;theta&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
  &lt;property valuetype="network"&gt;
    &lt;label&gt;Radial distance&lt;/label&gt;
    &lt;behavior name="ordZ" set="acnbase.bset"/&gt;
    &lt;protocol name="ESTA.DMP"&gt; ... &lt;/protocol&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;behavior name="datumProperty" set="acnbase.bset"/&gt;
      &lt;value type="string"&gt;phi&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
><p
>See also ordinate, datum, localDatum and datumProperty behaviors.</p
></section
></behaviordef
><behaviordef
name="radialLength"
><label
key="radialLength"
set="acnbase.lset"
></label
><refines
name="orthogonalLength"
set="acnbase.bset"
></refines
><section
><hd
>Radial Length</hd
><p
>A distance from an origin point. radialLength is orthogonal to angular offsets or height in polar coordinate systems.</p
></section
></behaviordef
><behaviordef
name="angleX"
><label
key="angleX"
set="acnbase.lset"
></label
><refines
name="polarOrdinate"
set="acnbase.bset"
></refines
><section
><hd
>Angle about x-axis</hd
><p
>This is a generic property representing an angle or rotation about the x-axis in whatever local coordinate reference applies. When used as an absolute angle, it is referenced to the y-axis (x = z = 0) with positive rotation from the y-axis towards the z-axis. This is the angle out of the xy-plane.</p
></section
></behaviordef
><behaviordef
name="angleY"
><label
key="angleY"
set="acnbase.lset"
></label
><refines
name="polarOrdinate"
set="acnbase.bset"
></refines
><section
><hd
>Angle about y-axis</hd
><p
>This is a generic property representing an angle or rotation about the y-axis in whatever local coordinate reference applies. When used as an absolute angle, it is referenced to the z-axis (y = x = 0) with positive rotation from the z-axis towards the x-axis. This is the angle out of the yz-plane.</p
></section
></behaviordef
><behaviordef
name="angleZ"
><label
key="angleZ"
set="acnbase.lset"
></label
><refines
name="polarOrdinate"
set="acnbase.bset"
></refines
><section
><hd
>Angle about z-axis</hd
><p
>This is a generic property representing an angle or rotation about the z-axis in whatever local coordinate reference applies. When used as an absolute angle, it is referenced to the x-axis (z = y = 0). This is the angle out of the zx-plane.</p
><p
>angleZ is also the normal angle to use in two dimensional polar coordinate systems where positions are all within the xy plane.</p
></section
></behaviordef
><!--Two dimensional measures--><behaviordef
name="point2D"
><label
key="point2D"
set="acnbase.lset"
></label
><refines
name="spatialCoordinate"
set="acnbase.bset"
></refines
><section
><hd
>Point in 2 Dimensions</hd
><p
>Group represents a point in 2D space. Can be defined in various ways including:</p
><section
><p
>• 2 orthogonalLengths</p
><p
>• An orthogonalAngle and a radialLength</p
><p
>• A direction3D and a radialLength</p
></section
></section
></behaviordef
><behaviordef
name="point3D"
><label
key="point3D"
set="acnbase.lset"
></label
><refines
name="spatialCoordinate"
set="acnbase.bset"
></refines
><section
><hd
>Point in 3 Dimensions</hd
><p
>Group represents a position in 3D space.</p
><p
>Can be defined in various ways including:</p
><section
><p
>• 3 orthogonalLength (cartesian coordinates)</p
><p
>• two orthogonalAngles and a radialLength (spherical polar coordinates)</p
><p
>• one orthogonalAngle a radialLength and a polar-height (cylindrical polar coordinates)</p
><p
>• a position2D and an orthogonalLength</p
><p
>• a direction3D and a radialLength</p
></section
></section
></behaviordef
><behaviordef
name="direction"
><label
key="direction"
set="acnbase.lset"
></label
><refines
name="multidimensionalGroup"
set="acnbase.bset"
></refines
><section
><hd
>Direction</hd
><p
>Abstract property group defining a direction.</p
><p
>Direction may be defined by a wide variety of sub-properties depending on the number of dimensions.</p
></section
></behaviordef
><behaviordef
name="orientation"
><label
key="orientation"
set="acnbase.lset"
></label
><refines
name="multidimensionalGroup"
set="acnbase.bset"
></refines
><section
><hd
>Orientation</hd
><p
>Abstract property group defines an orientation (e.g. pitch roll and yaw in 3 dimensions).</p
><p
>Orientation may be defined by a wide variety of sub-properties depending on the number of dimensions.</p
></section
></behaviordef
><!--Control over motion or orientation in three dimensions is
based on a conventional six-axis model of x,y,z plus pitch,
yaw and roll. However, actual devices may not offer control
over all of these, may use other dimensional measures or
coordinate systems to achieve the same ends. Also
terminology varies widely e.g. pitch, yaw, roll vs pan.
tilt, rotate vs bearing, azimuth etc.--><behaviordef
name="direction3D"
><label
key="direction3D"
set="acnbase.lset"
></label
><refines
name="direction"
set="acnbase.bset"
></refines
><section
><hd
>Direction in 3 Dimensions</hd
><p
>Group defines a direction in space.</p
><p
>In three dimensions, a direction differs from an orientation in that it only specifies two degrees of freedom rather than three. For example in an aircraft, pitch and yaw define a direction while pitch, yaw and roll define an orientation.</p
><p
>May be defined by a wide variety of sub-properties including:</p
><section
><p
>• 3 orthogonalLength properties (magnitude of vector is ignored)</p
><p
>• 2 orthogonalAngle properties (e.g. theta, phi)</p
><p
>• a point3D (magnitude of vector is ignored)</p
></section
></section
></behaviordef
><behaviordef
name="orientation3D"
><label
key="orientation3D"
set="acnbase.lset"
></label
><refines
name="orientation"
set="acnbase.bset"
></refines
><section
><hd
>Orientation in 3 Dimensions</hd
><p
>A group defining a 3 dimensional orientation. (e.g. pitch, yaw and roll).</p
><p
>May be defined by a wide variety of sub-properties including:</p
><section
><p
>• 3 orthogonalAngles</p
><p
>• a direction3D and an orthogonalAngle</p
></section
></section
></behaviordef
><behaviordef
name="position3D"
><label
key="position3D"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Position in 3 Dimensions</hd
><p
>A group defining a full six axis 3 dimensional position. (e.g. x, y, z, pitch, yaw and roll).</p
><p
>May be defined by a wide variety of sub-properties including:</p
><section
><p
>• a point3D and an orientation3D</p
><p
>• Any combination of coordinate systems which uniquely fix a position and orientation in space.</p
></section
></section
></behaviordef
><!--Publishing behaviors govern the details of Event Generation in DMP--><behaviordef
name="publishParam"
><label
key="publishParam"
set="acnbase.lset"
></label
><section
><hd
>Publish Parameter</hd
><p
>Abstract behavior Behaviors refined from this one represent parameters which control publishing of events from a property.</p
><p
>DMP provides the bare minimum at the protocol level to enable one component to advise another that it wishes to subscribe to events from a given property. It leaves the device description to specify which properties are capable of generating events, and to control the event generation criteria. The dmpprop element in DDL declares whether a property may generate events but goes no further.</p
><p
>In general many devices – particularly smaller ones – may only be able to generate events on a few properties or none at all, or may have restrictions on the number of properties for which events can be generated at one time.</p
><p
>An event message generated for a property declares the value of that property at the time the message is generated and the default action is to generate an event whenever the value of the property changes.</p
><p
>Events can never be generated for properties which have no value (group properties), for properties with immediate values, or for networked properties with constant values.</p
><p
>Events may be generated by implied properties (which have neither read nor write access) provided a DMP location is provided for the property. However, it is preferable that such a property should be made readable.</p
></section
></behaviordef
><behaviordef
name="publishMinTime"
><label
key="publishMinTime"
set="acnbase.lset"
></label
><refines
name="publishParam"
set="acnbase.bset"
></refines
><refines
name="timePeriod"
set="acnbase.bset"
></refines
><section
><hd
>Minimum Publish Time</hd
><p
>This property defines a minimum time between publishing events by the parent property. If the parent's value is changing more rapidly than the rate allowed by this property then intermediate values are discarded.</p
></section
></behaviordef
><behaviordef
name="publishMaxTime"
><label
key="publishMaxTime"
set="acnbase.lset"
></label
><refines
name="publishParam"
set="acnbase.bset"
></refines
><refines
name="timePeriod"
set="acnbase.bset"
></refines
><section
><hd
>Maximum Publish Time</hd
><p
>This property defines a maximum time between publishing events generated by it's parent property. If the parent's value has not changed within this time an event is generated anyway.</p
><p
>As a special case, if the value of a publishMaxTime property is zero, then behavior shall be as though this property were not present – that is, as if Maximum Publish Time were infinity.</p
></section
></behaviordef
><behaviordef
name="publishThreshold"
><label
key="publishThreshold"
set="acnbase.lset"
></label
><refines
name="publishParam"
set="acnbase.bset"
></refines
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Publish Threshold</hd
><p
>This property specifies the minimum difference between the last published value of the parent property and the current value required to trigger an event. Its parent property shall have measure behavior (or a refinement of it).</p
><p
>If not present or set to zero an event shall potentially be triggered by any detectable change in the parent's value (which is device and datatype dependent).</p
></section
></behaviordef
><behaviordef
name="publishEnable"
><label
key="publishEnable"
set="acnbase.lset"
></label
><refines
name="publishParam"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><section
><hd
>Publish Enable</hd
><p
>This property enables generation of events from the parent property. Note that other conditions may also need to be met for events to be generated - see DMP specification for subscription details.</p
><p
>Where multiple publishEnable properties are applied to a single parent (either directly or via a devPublisher) , they must all be true for events to be enabled.</p
><section
><hd
>Property Publishing Algorithm</hd
><p
>The algorithm for combining the publishing control properties is:</p
><section
><p
xml:space="preserve"
>If (((ΔV ≥ Pth ∧ T ≥ Tpmin) ∨ (T ≥ Tpmax)) ∧ En_1 ∧ En_2 ∧ ... ∧ En_n)
  Then publish(currentValue)</p
></section
><p
>where:</p
><section
><p
xml:space="preserve"
> En_x = Publisher enable Nº x (a boolean) for x=1 to x=n
    T = time since last published event
    V = current value of published variable
   ΔV = change in value of the published variable since last published value
Tpmin = Publish Min Time property
Tpmax = Publish Max Time property
  Pth = Publish Threshold</p
></section
></section
></section
></behaviordef
><!--Polling intervals--><behaviordef
name="pollInterval"
><label
key="pollInterval"
set="acnbase.lset"
></label
><refines
name="timePeriod"
set="acnbase.bset"
></refines
><section
><hd
>Network Polling Interval</hd
><p
>A variety of network operations which can be performed by devices with very limited controller functionality, require polling the network. For example in a pull binding, a slave may need to poll the master to know when its value changes (see binding behaviors).</p
><p
>In general network polling mechanisms do not scale well and should not be used. However, where they are the only mechanism possible to implement required functionality, poll interval behavior provides a means to regulate the operation.</p
><p
>Without refinement, this behavior represents a regular periodic polling interval for the operation represented by its logical parent property. Refinements of this behavior may represent parameters of more sophisticated polling algorithms.</p
><p
>The polling operation to which this refers is determined by the behavior of the parent property and need not be one taking place on the same network or using the same protocol.</p
></section
></behaviordef
><behaviordef
name="minPollInterval"
><label
key="minPollInterval"
set="acnbase.lset"
></label
><refines
name="pollInterval"
set="acnbase.bset"
></refines
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Minimum Polling Interval</hd
><p
>Some polling methods are not suitable for specification of a fixed polling interval using unrefined pollInterval behavior. For example, methods which are asynchronous and depend on how busy a device is, or adaptive methods. In these cases, this behavior allows a property to be used to specify the minimum polling interval and so put a limit on the frequency of network polling operations.</p
></section
></behaviordef
><behaviordef
name="maxPollInterval"
><label
key="maxPollInterval"
set="acnbase.lset"
></label
><refines
name="pollInterval"
set="acnbase.bset"
></refines
><refines
name="limit"
set="acnbase.bset"
></refines
><section
><hd
>Maximum Polling Interval</hd
><p
>Some polling methods are not suitable for specification of a fixed polling interval using unrefined pollInterval behavior. For example, methods which are asynchronous and depend on how busy a device is, or adaptive methods. In these cases, this behavior allows a property to be used to specify the maximum acceptable polling interval and so put a limit on the latency of network polling operations.</p
></section
></behaviordef
><!--Behaviors for Error conditions --><behaviordef
name="errorReport"
><label
key="errorReport"
set="acnbase.lset"
></label
><section
><hd
>Error Report</hd
><p
>The property reports error conditions. The position of the error reporter in the device description tree, indicates the device function which the error is associated with.</p
><p
>Error reporters may generate events when error status changes. An error condition may be a string, or possibly a behavior. Opaque error codes (enumerations) should not be used directly, but error strings, with multilingual replacements may be represented “on the wire” by a concise numeric value by using a selector – see “choice” behavior for an example of string value selection.</p
></section
></behaviordef
><behaviordef
name="connectionDependent"
><label
key="connectionDependent"
set="acnbase.lset"
></label
><section
><hd
>Connection Dependent Property</hd
><p
>Abstract behavior indicating that this property is in some way dependent on the state of a connection from a controller. The connection could be the one controlling this property (see connectedSwitch) or that controlling another property (typically the parent – see connectionReporter).</p
></section
></behaviordef
><behaviordef
name="connectedSwitch"
><label
key="connectedSwitch"
set="acnbase.lset"
></label
><refines
name="connectionDependent"
set="acnbase.bset"
></refines
><refines
name="enumSelector"
set="acnbase.bset"
></refines
><section
><hd
>Connection Dependent Switch</hd
><p
>In DMP controllers talk to devices via connections. For example when DMP operates over SDT the connection is an SDT session. and it is often desirable that operation is tied to a connection remaining open – if the connection from the controller is lost, some “safe” default action must be taken. A connectedSwitch property is a selector whose state reverts automatically to a set value if the connection used to set it is terminated or broken.</p
><p
>This selector has two values “disconnected” and “connected” which are represented by binary values 0 and 1 respectively. It may be set to either state by any controller in the normal way. However, the property shall only remain in “connected” state so long as the connection from the controller which most recently set it is maintained. If that connection is broken or terminated, the property's value immediately reverts to “disconnected”. The property may also be set by the controller to “disconnected” explicitly to indicate that any choices associated with the connected state are to be deselected, even though the connection remains.</p
><p
>A connectedSwitch property may usefully contain a connectionReporter property which identifies (e.g. for other for controllers) the connection which it is currently associated with.</p
><p
>The connectedSwitch behavior does not indicate that access to this property is restricted or is exclusive to a particular controller or connection. It depends only on the connection last used to set it. If another controller (or the same controller via a different connection) sets this property, its state will then be bound to that connection until either that connection terminates or another connection is used to set this property.</p
><section
><hd
>connectedSwitch example</hd
><p
>A motor's speed is controlled via a connectedSwitch property. To control the motor the connectedSwitch must first be set to “connected”. If connection is lost, the connectedSwitch reverts to “disconnected” and selects a default speed of “stopped”.</p
><p
>In this example, a connectionReporter sub-property is also provided which allows controllers to interrogate the device to find which connection the switch is currently bound to.</p
><p
xml:space="preserve"
>&lt;!-- The principle property "motor speed" is tied to the controller connection --&gt;

    &lt;property valuetype="network"&gt;
      &lt;label&gt;Motor Speed&lt;/label&gt;
      &lt;behavior name="rate1st" set="acnbase.bset" /&gt;
      &lt;behavior name="selected" set="acnbase.bset" /&gt;
      &lt;protocol protocol="ESTA.DMP"&gt;
        &lt;propref_DMP read="true" size="2" loc="0"/&gt;
      &lt;/protocol&gt;

&lt;!-- To control the motor, first set this property to “connected” --&gt;

      &lt;property valuetype="network"&gt;
        &lt;label&gt;Connection Establish&lt;/label&gt;
        &lt;behavior name="connectedSwitch" set="acnbase.bset" /&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP read="true" size="1" write="true" loc="1"/&gt;
        &lt;/protocol&gt;
      &lt;/property&gt;

&lt;!-- This is the default speed (zero) which the motor reverts to if connection is lost --&gt;

      &lt;property valuetype="immediate"&gt;
        &lt;label&gt;Disconnected speed&lt;/label&gt;
        &lt;behavior name="rate1st" set="acnbase.bset" /&gt;
        &lt;behavior name="choice" set="acnbase.bset" /&gt;
        &lt;value type="sint"&gt;0&lt;/value&gt;
      &lt;/property&gt;

&lt;!-- Setting this property controls motor speed, but only while the connection is active --&gt;

      &lt;property valuetype="network"&gt;
        &lt;label&gt;Connected speed control&lt;/label&gt;
        &lt;behavior name="rate1st" set="acnbase.bset" /&gt;
        &lt;behavior name="choice" set="acnbase.bset" /&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP read="true" size="2" write="true" loc="2"/&gt;
        &lt;/protocol&gt;
      &lt;/property&gt;

&lt;!-- This optional property allows the currently active control connection to be identified --&gt;

      &lt;property valuetype="network"&gt;
        &lt;label&gt;Most recent controller&lt;/label&gt;
        &lt;behavior name="connectionReporter" set="acnbase.bset" /&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP read="true" size="18" loc="3"/&gt;
        &lt;/protocol&gt;
      &lt;/property&gt;
    &lt;/property&gt;</p
></section
></section
></behaviordef
><behaviordef
name="connectionReporter"
><label
key="connectionReporter"
set="acnbase.lset"
></label
><refines
name="transportConnection"
set="acnbase.bset"
></refines
><refines
name="connectionDependent"
set="acnbase.bset"
></refines
><section
><hd
>Connection Reporter Property</hd
><p
>This property, which must be read-only, identifies the connection which was used by the controller which last set the value of this property's parent all the while that connection is active. Once that connection has expired or been terminated, it indicates “no connection”. This property also indicates “no connection” if the parent property has not been set since the device was last reset or powered on.</p
><p
>The connectionReporter property does not imply that the state of the connection has any affect on the parent property but simply reflects the state of the connection which was last used to set it. (other behaviors such as “connectedSwitch may tie a value to the state of a connection).</p
><p
>The connectionReporter property does not indicate that access to the parent property is restricted or is exclusive to a particular connection. It registers the connection last used to set the parent and indicates whether that connection is still active. If another controller (or the same controller via a different connection) sets the parent property, the connectionReporter changes to indicate the connection used.</p
><p
>The value of “no connection” depends on the particular transport used – see individual transport's refinements of connectionID for details.</p
></section
></behaviordef
><!--Binding properties together--><behaviordef
name="binding"
><label
key="binding"
set="acnbase.lset"
></label
><section
><hd
>Property Binding</hd
><p
>A binding is a mechanism which allows two properties to be connected such that when one property is changed all other is automatically updated to the same value, by internal action between properties within the same appliance, by network reads or writes or by any other suitable mechanism.</p
><p
>Behaviors refined from binding identify the bound properties and also include a range of behaviors for control and management of binding relationships.</p
><section
><hd
>Network Bindings</hd
><p
>The most generic case of a binding allows the bound properties to be in arbitrary devices within the system or even on “foreign” networks. In these cases implementing the binding requires network messages to be exchanged to ensure synchronism of the bound properties. Because, the bound properties may be within devices which come and go independently, there are often additional properties needed to set up, enable, monitor or otherwise control the operation of the binding. These properties are called binders.</p
></section
><section
><hd
>Internal Bindings</hd
><p
>When both properties which are bound together are constrained to be within the same appliance for all possible configurations of the binding, the binding as termed an “internal binding”. In an internal binding, the mechanism for maintaing synchronism between property values is entirely within the operating code of the appliance and can be effectively instantaneous.</p
><p
>Any binding which is not an internal binding is considered to be a network binding, even if its mechanism does not require transactions on the access network.</p
></section
><section
><hd
>Terminology</hd
><section
><hd
>Anchor and Remote Properties</hd
><p
>All bindings are described with an “anchor property” which is a permanent member and a “remote property” which is identified by reference. If the reference is a variable value, then the choice of remote property is variable. Note the term “remote property” is used even if it happens to be in the same device as the anchor property.</p
><p
>The terminology and description of bindings including “push” and “pull”, always takes the viewpoint of the anchor property.</p
></section
><section
><hd
>Driver Appliance</hd
><p
>In any network binding, there is one appliance which does the work of implementing the binding by initiating the necessary network transactions or configuration. This is called the “driver appliance”.</p
></section
><section
><hd
>Binder Properties</hd
><p
>Any binding may require additional properties to turn it on and off or otherwise control and monitor the action of the binding. All such properties which are directly concerned with a binding, but are not themselves bound are called “binder” properties.</p
></section
><section
><hd
>Push</hd
><p
>In a push binding, the anchor property's value is pushed into the remote property whenever it changes in a master to slave fashion. If the remote property is in another appliance, this is normally performed using a network write operation. If the remote property is in the same appliance, then no network operation is required and the mechanism should be internal to the appliance.</p
></section
><section
><hd
>Pull</hd
><p
>In a pull binding, the anchor property's value is a slave to the remote property and when that property changes, the anchor property is updated to match. If the remote property is in another appliance, this can be performed using a network read operation or by the remote device publishing the value of the remote property when it changes. If the remote property is in the same appliance, then no network operation is required and the mechanism can be a completely automatic opaque operation within the appliance.</p
></section
><section
><hd
>Master/slave or unidirectional Bindings</hd
><p
>Straightforward push and pull bindings are both unidirectional. One property changes as a result of other processes, and the other then follows as a result of the binding.</p
><p
>A unidirectional binding is also called a master/slave binding, because the slave property always changes to match the master, while the master is independent.</p
><p
>On its own, a push binding is one where the anchor property is the master, while in a purely pull binding, the anchor property is the slave.</p
></section
><section
><hd
>Bidirectional Bindings</hd
><p
>In a bidirectional binding, values are both pushed and pulled between the two bound properties. This means that if either the anchor or the remote property is changed by some other method than via the binding, the result will be reflected in the other property. Bidirectionality can also make a difference to the way the binding is initialized when it is first activated – see Initialization below.</p
><p
>A whole group of properties may be bound together bidirectionally such that they all maintain the same value.</p
></section
><section
><hd
>Multiway Bindings</hd
><p
>A multiway binding is one between more than two properties.</p
><p
>In the terminology of these rules, any multi-way binding is treated as a group of single-way bindings, each between just two properties, but with one or more properties being involved in multiple bindings.</p
><p
>A common case is a single “master” which is bound to multiple “slaves”. In the terminology of these behaviors, each master/slave combination is a separate binding but all share the same master. This allows different mechanisms and methods to be used for each slave without complicating ing the rules.</p
><p
>Another example of a multiway binding described as a combination of multiple single-way bindings is a third party binding.</p
><p
>According to the rules below, a single anchor property shared by multiple bindings is easily constructed by containing each binding within the same anchor. Other multiple binding structures can use the usual shareing or reference mechanisms of DDL.</p
></section
><section
><hd
>Third Party Bindings</hd
><p
>It is possible for a third party driver appliance to implement a binding between two arbitrary properties, by first pulling the value from one, and then pushing it to the other. This is called a third party binding or sometimes a pull-push binding. Any third party binding can be described by introducing an intermediate anchor property within the driver appliance which is bound to one remote with a pull binding and to the other with a push binding.</p
><p
>This intermediate anchor property may have an implied value.</p
></section
></section
><section
><hd
>Syntactic Structure of a Binding</hd
><p
>The primary constituents of any binding are the anchor property, the remote property and any necessary binder properties.</p
><section
><hd
>Anchor Property</hd
><p
>The anchor property shall be directly identified by bindingAnchor behavior (or a refinement). Because it is directly identified, the choice of anchor within a particular binding cannot be varied so the anchor property must be a permanent member of the binding.</p
><p
>For any network binding, the anchor property shall be a property within the driver appliance. It is the property which initiates and implements the binding.</p
><p
>For an internal binding, the anchor property may be chosen more freely and may be any property which is a permanent member of the binding.</p
><p
>The anchor property serves to group all the other properties of a binding.</p
></section
><section
><hd
>Remote Property</hd
><p
>Each binding shall have one remote property reference. The property pointed to by this reference is the remote property of the binding.</p
><p
>A remote property reference shall be a property which carries both a refinement of propertyRef behavior identifying the type of reference, and a refinement of bindingMechanism behavior indicating the direction or mechanism(s) used to implement the binding.</p
><p
>The remote property reference shall be a logical child of the anchor property.</p
><p
>If at any time, the remote property reference does not resolve to a real property then the binding is inactive. The binding may also be inactive because of the state of associated binder properties.</p
><section
><hd
>Multiple Remote Property References</hd
><p
>An anchor property may contain multiple remote property references. Each one constitutes a separate binding with all these bindings sharing the same anchor property.</p
></section
></section
><section
><hd
>Binder Properties</hd
><p
>Binder properties have behaviors which are refined from binder behavior. A binder shall be associated with a binding in one of four ways:</p
><section
><p
>It may be a logical child of the anchor property. This associates the binder with all bindings whose remote references are also logical children of the anchor property.</p
></section
><section
><p
>It may be the target of a binderRef which is a logical child of the anchor property. This associates the binder with all bindings whose remote references are also logical children of the anchor property.</p
></section
><section
><p
>It may be a logical child of a remote reference property. This associates the binder with the binding described by the remote reference.</p
></section
><section
><p
>It may be the target of a binderRef which is a logical child of a remote reference property. This associates the binder with the binding described by the remote reference.</p
></section
></section
><section
><hd
>Note: Logical Children</hd
><p
>A logical child within the context of this definition includes descendant properties where they act at the child level. For example the a property which is declared at the root level of a subdevice which is included by reference is considered to be a logical child of the property containing the device-reference.</p
></section
></section
><section
><hd
>Combinatorial Binding Relationships Disallowed</hd
><p
>Creating bindings in which multiple master properties contribute to a slave value using combinatorial algorithms is not permitted. These should instead be described using driver and driven property relationships with multiple driver properties, since this provides a more explicit description of the capabilities of the device. The driver properties in such a description can then be bound as necessary using separate bindings if required.</p
></section
></section
><section
><hd
>More on Internal and Network Bindings</hd
><section
><hd
>Internal Bindings</hd
><p
>An internal binding is constrained so that both bound properties are within a single appliance. This allows a permanent relationship to be declared between two or more properties which is always operative and has no mechanism to modify the relationship or turn it off (this is called a fixed binding). It is not possible to implement such a permanent binding using network mechanisms since a network is not a permanent connection.</p
><p
>Internal bindings also allow more complex relationships such as dynamic bindings and window access behaviors (see windowAccess behaviorset). Thus although internal bindings appear similar to network bindings and use similar descriptions, their use is often very different.</p
><p
>Internal bindings are a versatile tool in describing interrelationships between properties – in the case of fixed bindings, the bound properties are effectively the same property at different locations in the device tree, and when the binding is bidirectional can also allow the same property to be readable or writable at multiple addresses. Since all properties in a fixed binding must contain the same value, it is not necessary for all properties to be accessible and some may be declared as implied properties.</p
><p
>In an internal binding the disctinction between the anchor and remote properties is purely one of the way the binding is described rather than one of implementation, so the anchor and remote properties can be selected at will to suit description structure.</p
><section
><hd
>Fixed Bindings vs Shared Properties</hd
><p
>A fixed binding within a device is similar in many ways to a shared property – there is a single value which applies in more than one place in the tree. However, shared properties share not only their value but also their tree of sub-properties. A fixed binding simply links the values.</p
></section
><section
><hd
>Internal Bindings across Devices</hd
><p
>Since an internal binding is by definition constrained to properties within an appliance, it cannot be established between arbitrary independent devices since they can have separate existences. The presence of an internal binding between properties in different devices therefore implies that the two devices must be within the same appliance.</p
><p
>Within DMP an appliance corresponds to a ACN component, so while in theory a single network host could implement an internal binding between two components which it contained, in practice these must be treated as generic network bindings.</p
></section
></section
><section
><hd
>Network Bindings</hd
><p
>Any binding in which the bound properties are or may be within different appliances is defined to be a network binding. This does not mean that the binding necessarily operates using access protocol operations. For example, a binding whose remote property is configurable, may be configured so that the remote happens to be within the same appliance as the anchor. In this case the driver appliance should not send messages to itself which appear on the external network (this depends on the nature of the access protocol). Another example is the case of xeno bindings, where the remote property is part of a “foreign” network (see xenoBindingMech), the binding operation is not visible on the access network. Xeno bindings can occur whenever two devices are connected by a “foreign” network even if both are also visible on the access network.</p
><p
>A network binding is asymmetrical. The work of implementing the binding must be done entirely by the driver appliance. For example, by reading or writing values from the remote properties. Depending on the specific protocol, a network binding may require significant additional capabilities of the driver appliance.</p
><p
>In a network binding, auxilliary binder properties are usually required to configure, enable and disable the binding.</p
><p
>The network mechanism used to implement a binding must be specified by refinement of bindingMechanism.</p
></section
><section
><hd
>Cross Protocol or Xeno Bindings</hd
><p
>A cross-protocol binding is a way to represent a connection in a gateway device between a property in the access protocol of DDL (the “native” protocol or property) and a function or property in some other (“foreign”) protocol. The native property is a proxy for the foreign property which is accessed by the gateway device.</p
><p
>Cross protocol bindings are only possible where the action of the foreign protocol can be modelled in terms of devices and properties. They are useful for representing generic gateways to such protocols, but they require a controller to have some knowledge and understanding of the foreign protocol and can not provide greater functionality than that of the foreign protocol.</p
><p
>A cross protocol binding represents an intermediate between two extremes. At one extreme, no modelling at all is provided for devices on the foreign network and its data is simply tunneled within the native protocol. A controller must construct the packets of the foreign protocol and handle all translation. At the other extreme, a gateway device implements a true device proxy for each foreign device, representing it on the native network as a full native device and handling the translation between protocols in a hidden way. By representing devices on the foreign network using cross-protocol bindings, a gateway is presenting proxy properties but not a full proxy device model.</p
><p
>Cross Protocol bindings follow the rules for network bindings except where explicitly stated. Any foreign protocol requires one or more refinements of xenoPropRef behavior to be defined which specifies how “properties” of devices accessed using the foreign protocol are represented and referenced.</p
></section
><section
><hd
>Fixed and Variable Bindings</hd
><p
>The remote properties of a binding are identified by references. If the values of these references are fixed constants then the binding is said to be a fixed binding. If the references have writable or otherwise variable values, the binding is said to be variable. Almost all cases of network bindings will be variable bindings, while internal bindings are often fixed, or variable only within strict limits.</p
></section
></section
><section
><hd
>Activation Synchronization</hd
><p
>Whenever a binding between two properties is activated, the driver appliance shall ensure that an initial synchronization occurs between the two properties.</p
><section
><hd
>Push Bindings</hd
><p
>For a push binding, the remote property must be initialized by changing it to match the current value of the anchor property.</p
></section
><section
><hd
>Pull Bindings</hd
><p
>For a pull binding, the anchor property must be initialized by changing it to match the current value of the remote property.</p
></section
><section
><hd
>Bidirectional Bindings</hd
><p
>For a bidirectional binding binding, initial synchronization could take place in either direction. Either the anchor property value is pushed to the remote (push precedence), or the remote is pulled to the anchor (pull precedence).</p
><p
>For any bidirectional binding the precedence must be defined. If it is not explicitly defined by the description of the behaviors which make up the binding, then pull precedence shall be used by default.</p
></section
></section
><section
><hd
>Note on Parsing</hd
><p
>Bindings introduce the concept that properties may exist in the device model which only have use when bound to other properties. For example blocks which represent a processing algorithm (such as a “highest takes priority” merge which is common in lighting control) are useful only when they are connected to other properties (e.g. ones representing lighting intensity). The input and or output properties of such blocks are designed to be bound to other properties and in the case of fixed bindings, these properties may be implied ones with no counterpart on the network. Bindings represent a flexible method whereby these processing blocks may be connected together.</p
><p
>The controller parsing descriptions containing bindings, must be prepared to encounter seeming “dead-end” properties in the chain of control which then turn out to be the target of binding references.</p
></section
><section
><hd
>See also</hd
><p
>bindingMaster, bindingSlave, bindingBiDi, bindingMasterRef, bindingSlaveRef, bindingBiDiRef, binder, bindingMechanism.</p
></section
></behaviordef
><behaviordef
name="boundProperty"
><label
key="boundProperty"
set="acnbase.lset"
></label
><refines
name="binding"
set="acnbase.bset"
></refines
><section
><hd
>Bound Property</hd
><p
>This property is one which is or may be bound to another.</p
><p
>For most ordinary bindings, only the anchor property is identified explicitly as bound, while remote properties are identified only by being the target of references. For some more complex binding types, both ends of the binding may need explicit identification by refinement of this behavior.</p
></section
></behaviordef
><behaviordef
name="bindingAnchor"
><label
key="bindingAnchor"
set="acnbase.lset"
></label
><refines
name="boundProperty"
set="acnbase.bset"
></refines
><section
><hd
>Binding Anchor Property</hd
><p
>This behavior directly identifies the property bearing it as being the anchor property of zero or more bindings.</p
><p
>This property shall contain zero or more remote property references as its logical children.</p
><p
>Note: normally an anchor property will have at least one remote property reference, but it is possible for a binding anchor to be constructed where the number of bindings within it is unspecified and can therefore be zero.</p
><p
>If this is a network binding then the appliance containing this property must have the capability to initiate, manage and operate the binding.</p
><p
>Any binder behavior which is the logical child of a bindingAnchor property shall be associated with all bindings which are represented by remote property references which are logical children of the same property.</p
><p
>Any binder behavior which is the target of a binderRef property which is a logical child of a bindingAnchor property shall be associated with all bindings which are represented by remote property references which are logical children of the same bindingAnchor property.</p
></section
></behaviordef
><behaviordef
name="windowProperty"
><label
key="windowProperty"
set="acnbase.lset"
></label
><refines
name="boundProperty"
set="acnbase.bset"
></refines
><section
><hd
>Window Property</hd
><p
>A window property forms one end of a binding which exists solely to be bound and does not perform any other external or internal function. Thus, it is a “window” through which the property at the other end of the binding is accessed.</p
><section
><hd
>On-demand Polling</hd
><p
>A special case occurs where the anchor property exists purely as an proxy property for the remote property and does not directly represent a controllable aspect of a device, drive other properties or generate its own events (this situation may be common with xeno bindings where DDL properties act as proxies for properties in remote devices accessed using different protocols). In this case, the driver appliance need not poll the remote property until the anchor property itself is read. At this time it reads the remote property and then returns the value obtained as the value of the proxy property.</p
></section
></section
></behaviordef
><behaviordef
name="bindingMechanism"
><label
key="bindingMechanism"
set="acnbase.lset"
></label
><refines
name="binding"
set="acnbase.bset"
></refines
><section
><hd
>Mechanism and Reference to a Remote Property in a Binding</hd
><p
>Any property which carries one or more bindingMechanism behaviors shall also carry a propertyRef behavior indicating the remote property which is bound using the specified mechanisms. See binding behavior for additional details.</p
><p
>Refinements of this behavior describe specific binding mechanisms and declares that the binding conforms to the rules specified in the description of that refinement.</p
><section
><hd
>Multiple Mechanisms on One Property</hd
><p
>It is allowable for one property to carry multiple refinements of bindingMechanism, where alternative mechanisms are available or apply.</p
><p
>Variable bindings may be implemented by different mechanisms in different cases. For example, a pull binding with a variable remote reference may be implemented differently depending whether the chosen target property is able to publish its value or needs to be polled. In these cases all implemented mechanisms shall be declared as separate behaviors.</p
><p
>Where multiple bindingRemoteRef behaviors are present, the component implementing the binding is assumed to make the decision as to which to use (subject to any rules specified by those behaviors) unless a selection mechanism is provided. Specific mechanisms may be explicitly selected using a combination of selected and behaviorRef behaviors.</p
><p
>For bidirectional bindings, two or more mechanisms may be required to make the binding work in both directions.</p
></section
></section
></behaviordef
><behaviordef
name="binder"
><label
key="binder"
set="acnbase.lset"
></label
><refines
name="binding"
set="acnbase.bset"
></refines
><section
><hd
>Property Binder</hd
><p
>A binder property is an auxilliary property or property group which describes or controls a binding (see binding behavior).</p
><p
>Any property used to configure, control or simply describe a binding but which is not one of the bound properties (or reference to one) should inherit from binder behavior.</p
></section
></behaviordef
><behaviordef
name="binderRef"
><label
key="binderRef"
set="acnbase.lset"
></label
><refines
name="binding"
set="acnbase.bset"
></refines
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to a Binder Property</hd
><p
>This property is a propertyRef which identifies or “points to” a binder property. The specific type of the binder property shall be indentified by behaviors attached to the target of this reference. The type pf reference shall be specified by additional behaviors of this property.</p
></section
></behaviordef
><behaviordef
name="pushBindingMechanism"
><label
key="pushBindingMechanism"
set="acnbase.lset"
></label
><refines
name="bindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Push Binding Mechanism</hd
><p
>This is a binding mechanism implemented by the driver appliance writing the value of the anchor property to the remote property(s) whenever it changes. It is an abstract behavior with refinements describing more specific mechanisms.</p
><p
>By itself, a pushBindingMechanism implements a master/slave binding in which the anchor property is the master.</p
><p
>If there is also a pullBindingMechanism present on the same remote reference then the binding is bidirectional.</p
><p
>If multiple pushBindingMechanism refinements are present on the same property then these may be alternatives which are selected by the driver appliance as described in binding behavior.</p
></section
></behaviordef
><behaviordef
name="pullBindingMechanism"
><label
key="pullBindingMechanism"
set="acnbase.lset"
></label
><refines
name="bindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Pull Binding Mechanism</hd
><p
>This is a binding mechanism Implemented by the driver appliance receiving the value of the remote property, then updating the anchor property to match. It is an abstract behavior with refinements describing more specific mechanisms.</p
><p
>By itself, a pullBindingMechanism implements a master/slave binding in which the anchor property is the slave.</p
><p
>If there is also a pushBindingMechanism present on the same remote reference then the binding is bidirectional.</p
><p
>If the target of the reference is constrained to be within the same appliance, (e.g. it is a localDDLpropertyRef) then this is an internal binding.</p
><p
>If multiple pullBindingMechanism refinements are present on the same property then these may be alternatives which are selected by the driver appliance as described in binding behavior.</p
></section
><section
><hd
>Network Mechanisms</hd
><p
>The following only applies to network bindings.</p
><p
>Most protocols fall into one of two models for implementation of pull bindings: polled reading, or asynchronous subscription. Some protocols (including DMP) allow both.</p
><section
><hd
>Polled Binding Mechanisms</hd
><p
>If the protocol allows the driver appliance to perform reads of the remote property, then a polled binding can be implemented by the driver appliance polling the remote property for changes. Polled bindings are generally inefficient – they must either generate large amounts of network traffic if the polling interval is small, or operate with long latencies between remote and anchor changes if the polling interval is long. For this reason, polled bindings should only be used in exceptional circumstances. They are however, the only mechanism possible if the remote property is not capable of publishing its value asynchronously.</p
><p
>No specific polling algorithm is specified by this behavior. A regular, fixed interval poll is one implementation, but more complex algorithms may be implemented to provide a balance between network usage and latency, or the needs and characteristics of specific properties.</p
><p
>Where specific polling methods must be exposed, then this can be done by refinement of this behavior, or implied by the presence of other properties specific to those methods.</p
><p
>A pollInterval child property may be used to regulate the polling operation of this behavior and in its unrefined form indicates a simple regular polling scheme. Refinements indicate more sophisticated schemes. See polltiming behaviorset for details.</p
><section
><hd
>On-demand Polling</hd
><p
>A special case occurs where the anchor property exists purely as an proxy property for the remote property and does not directly represent a controllable aspect of a device, drive other properties or generate its own events (this situation may be common with xeno bindings where DDL properties act as proxies for properties in remote devices accessed using different protocols). In this case, the driver appliance need not poll the remote property until the anchor property itself is read. At this time it reads the remote property and then returns the value obtained as the value of the proxy property.</p
></section
></section
><section
><hd
>Subscription Binding Mechanisms</hd
><p
>If the protocol allows the device containing the remote property to publish the value of that property asynchronously, when it changes (e.g. in DMP by generating events) then a subscription binding can be implemented. The driver device must take what steps are necessary (if any) to set up the remote device to publish the value of the remote property and to subscribe to those publishing events. It then updates its local property with the received value.</p
><p
>A subscription based mechanism is usually much more efficient than a polled one since a message is only generated when the remote property's value changes. However, an initial synchronization operation is generally needed to prevent an interval between activating the binding and the first change of remote value, during which the anchor property has an undefined value.</p
></section
></section
></behaviordef
><behaviordef
name="internalSlaveRef"
><label
key="internalSlaveRef"
set="acnbase.lset"
></label
><refines
name="pushBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Internal Binding Slave Property Reference</hd
><p
>As with any bindingMechanism behavior, this property shall also carry a propertyRef refinement which identifies a specific remote property which is bound. The propertyRef shall point to a property within the same appliance. If it resolves to a property within another appliance, then this is an error and no binding action shall take place.</p
><p
>The target of this reference is bound as a slave to the anchor property which must be the logical parent of this one.</p
><p
>The binding is internal, so is implemented entirely within the appliance containing both properties.</p
></section
></behaviordef
><behaviordef
name="internalMasterRef"
><label
key="internalMasterRef"
set="acnbase.lset"
></label
><refines
name="pullBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Internal Binding Master Property Reference</hd
><p
>As with any bindingMechanism behavior, this property shall also carry a propertyRef refinement which identifies a specific remote property which is bound. The propertyRef shall point to a property within the same appliance. If it resolves to a property within another appliance, then this is an error and no binding action shall take place.</p
><p
>The anchor property which must be the logical slave of this one is bound as a slave to the target of this reference.</p
><p
>The binding is internal, so is implemented entirely within the appliance containing both properties.</p
></section
></behaviordef
><behaviordef
name="internalBidiRef"
><label
key="internalBidiRef"
set="acnbase.lset"
></label
><refines
name="pushBindingMechanism"
set="acnbase.bset"
></refines
><refines
name="pullBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Internal Bidirectional Binding Property Reference</hd
><p
>As with any bindingMechanism behavior, this property shall also carry a propertyRef refinement which identifies a specific remote property which is bound. The propertyRef shall point to a property within the same appliance. If it resolves to a property within another appliance, then this is an error and no binding action shall take place.</p
><p
>The anchor property which must be the logical slave of this one is bound bidirectionally to the target of this reference.</p
><p
>The binding is internal, so is implemented entirely within the appliance containing both properties.</p
></section
></behaviordef
><behaviordef
name="DMPbinding"
><label
key="DMPbinding"
set="acnbase.lset"
></label
><refines
name="bindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>DMP Network Binding</hd
><p
>Properties with this behavior implement a network binding using DMP. For full details of bindings and terminology see binding behavior in this behaviorset. This is an abstract behavior and specific DMP mechanisms are refined from it.</p
><section
><hd
>Push Bindings</hd
><p
>Only one mechanism exists for push bindings in DMP. See DMPsetPropBinding.</p
></section
><section
><hd
>Pull Bindings</hd
><p
>A DMP pull binding can be implemented in two ways. See DMPeventBinding and DMPgetPropBinding.</p
></section
></section
></behaviordef
><behaviordef
name="DMPsetPropBinding"
><label
key="DMPsetPropBinding"
set="acnbase.lset"
></label
><refines
name="DMPbinding"
set="acnbase.bset"
></refines
><refines
name="pushBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>DMP Set Property Binding</hd
><p
>A DMP push binding which is implemented by the component containing a anchor property (either a master or bidirectionally bound property) using Set Property messages. The remote property is identified by reference. Any remote property chosen must be writable by the driver component and must have the same network size, but otherwise requires no special characteristics or capabilities.</p
><p
>When the binding is activated, the driver component shall establish a connection to the remote property(s) and shall send an initial Set Property message to establish synchronization. Subsequently whenever the anchor property changes, its component must update all remotes to match by sending Set-Property messages.</p
></section
></behaviordef
><behaviordef
name="DMPgetPropBinding"
><label
key="DMPgetPropBinding"
set="acnbase.lset"
></label
><refines
name="DMPbinding"
set="acnbase.bset"
></refines
><refines
name="pullBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>DMP Get Property Binding</hd
><p
>A polled pull binding which is implemented by the driver component using DMP Get Property messages to poll the value of a remote property. The remote property (which must be a master or bidirectionally bound property) is identified by reference. Any chosen remote property must be readable by the driver component and must have the same network size, but otherwise requires no special characteristics or capabilities.</p
><p
>When the binding is activated, the driver component shall commence polling using Get Property messages to establish synchronization with the remote property. Whenever the remote property is seen to have changed the driver component must update the anchor property to match.</p
><p
>DMPgetPropBinding is very network inefficient – see pullBindingMechanism for discussion.</p
><p
>Wherever a component implements both DMPgetPropBinding and DMPeventBinding behavior on the same property (see bindingMechanism behavior section “Multiple Bindings”) the DMPeventBinding mechanism shall be used by default. DMPgetPropBinding shall only be used if the configured master property cannot generate events.</p
><p
>A pollInterval child property may be used to regulate the polling operation of this behavior.</p
></section
></behaviordef
><behaviordef
name="DMPeventBinding"
><label
key="DMPeventBinding"
set="acnbase.lset"
></label
><refines
name="DMPbinding"
set="acnbase.bset"
></refines
><refines
name="pullBindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>DMP Event Binding</hd
><p
>A subscription pull binding which is implemented by the driver component using DMP Subscribe and Event messages. The remote property (which must be a master or bidirectionally bound property) is identified by reference. Any property chosen as a master of the binding must be capable of generating events to the driver component and must have the same network size, but otherwise requires no special characteristics or capabilities. When the binding is activated, the driver component shall establish a connection to the remote device and shall subscribe to events from the remote property. Subsequently whenever an event is received from the remote property the driver component must update the anchor property to match.</p
><p
>Wherever a component implements both DMPgetPropBinding and DMPeventBinding behavior on the same property (see bindingMechanism behavior section “Multiple Bindings”) the DMPeventBinding mechanism shall be used by default. DMPgetPropBinding shall only be used if the configured remote property cannot generate events.</p
></section
></behaviordef
><behaviordef
name="bindingState"
><label
key="bindingState"
set="acnbase.lset"
></label
><refines
name="binder"
set="acnbase.bset"
></refines
><refines
name="boolean"
set="acnbase.bset"
></refines
><section
><hd
>Binding State</hd
><p
>A boolean value which is true is the binding is connected (active) and false otherwise. If the state is false then the values of other properties associated with the binding may be unspecified or invalid.</p
><p
>Where a bindingState property is writeable then setting its value to true activates the binding. In the case of a generic network binding, this means that the component containing the property must initiate whatever network action is necessary to implement the binding (e.g. in the case of a DMP pull binding, setting the bindingState to true would cause the component to subscribe to events from the indicated master property).</p
></section
></behaviordef
><!--Interfaces to "properties" on other networks--><behaviordef
name="xenoPropRef"
><label
key="xenoPropRef"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to a Property on a Foreign Network</hd
><p
>This property is a reference or pointer to a property within a device accessed using some network and/or protocol which is different from the access network and protocol defined for the description.</p
><p
>The capacity to represent entities accessed using a foreign protocol as properties requires some translation which may be complex or trivial. This translation must be defined by refinements of this behavior applicable to specific protocols.</p
></section
></behaviordef
><behaviordef
name="xenoBinder"
><label
key="xenoBindingMechanism"
set="acnbase.lset"
></label
><refines
name="bindingMechanism"
set="acnbase.bset"
></refines
><section
><hd
>Xeno or Foreign Protocol Binding Mechanism</hd
><p
>A xeno binding is a connection between a DMP anchor property and a property in a remote device controlled via another protocol – see binding. The details will depend on the remote protocol and the capabilities it provides.</p
><p
>This property shall also carry a refinement of xenoPropRef behavior which specifies how the remote property is referenced and represented. Together, the addressing mechanism and access mechanism provide a way to specify translation between the access network and interfaces to foreign protocols.</p
><p
>As with other bindings, xeno bindings may be master/slave or bidirectional, depending on the characteristics of the foreign protocol and representation of its properties.</p
></section
></behaviordef
><!--Access windows ? properties giving access to other properties--><behaviordef
name="accessWindow"
><label
key="accessWindow"
set="acnbase.lset"
></label
><refines
name="boundProperty"
set="acnbase.bset"
></refines
><section
><hd
>Access Window</hd
><p
>This property forms a “window” through which zero or more bound properties are “visible” (accessible). The properties bound to an access window are called “window bound” properties. The choice of window bound property visible in an accessWindow can change dynamically from one access to another (for example, according to the component performing the access). This allows an accessWindow to provide a “view” of multiple window bound properties which are “visible” through it depending on the enabling binders on each binding. It can operate bidirectionally allowing reading and/or writing of the window bound property.</p
><section
><hd
>Structure and Syntax</hd
><p
>An accessWindow property shall form the endpoint of zero or more bindings. For each binding the accessWindow may be either the anchor, or the remote property. Note though that for network bindings, implementation may requires the accessWindow to be the anchor property.</p
><section
><hd
>Access Enabling</hd
><p
>Each binding to the accessWindow shall have an associated accessMatch binder which specifies when the binding is active. A window bound property is “visible” through the accessWindow whenever the accessMatch condition is satisfied. The binder is associated with the binding in any of the ways allowed for ordinary bindings.</p
><p
>In order for the window bound property to be read as well as written via the accessWindow, the matching mechanism must be mutually exclusive such that no more than one window bound property is accessed for any given network operation. This restriction does not apply where the accessWindow is write only.</p
><p
>The exact mechanism controlling the accessWindow is defined by the particular accessMatch type used. Refinements can be used to describe many access control mechanisms used for resource sharing, security controls, authorization etc.</p
><p
>The match criteria specified by an accessMatch property may be static or dynamic. See accessMatch behavior for more details.</p
></section
><section
><hd
>Initial Synchronization</hd
><p
>See Activation Synchronization section in binding behavior.</p
><p
>If the binding is bidirectional then the accessWindow shall not have prioprity over the window bound property on activation. The value of a window bound property shall never be changed just by activation of the binding – the only way to change its value via the binding shall be by writing the accessWindow whilst the binding is active.</p
><p
>Where an accessWindow is the remote property of a bidirectional binding, this rule overrides the default rule of pull-precedence.</p
></section
><section
><hd
>No Side Effects</hd
><p
>The accessWindow exists purely as a mechanism to access other properties and must not itself control or affect the device directly.</p
><p
>The accessWindow property shall not affect the values of other properties, except those involved in its bindings. For example, it must not be a driver in a driver/driven relationship. This restriction does not apply to those properties bound to the accessWindow, nor to the binder properties associated with those bindings.</p
><p
>The accessWindow property shall not itself represent or affect external functions of the device, whether physical functionality, or external network functions. This restriction does not apply to those properties bound to the accessWindow, nor to the binder properties associated with those bindings.</p
><section
><hd
>On-demand Polling</hd
><p
>The rules enforcing no side effects mean that an access window always meets the criteria for on-demand polling as described in pullBindingMechanism.</p
></section
></section
><section
><hd
>Unmatched Access</hd
><p
>An attempt to read or write an accessWindow property for which no binding match is satisfied shall generate a failure message. For DMP this shall be Get/Set Property Fail, or Subscribe Fail as appropriate.</p
></section
></section
></section
><section
><hd
>Notes on Usage and Applications</hd
><section
><hd
>Read, Write and Read/Write access</hd
><p
>A window bound property may be writable in the normal way or may be only writable via the accessWindow. It may have an implied value with its only access occurring via the window. If a window bound property is writable directly, then conflicts can easily arise if attempts are made to control it simultaneously both directly and via the accessWindow.</p
></section
><section
><hd
>Resource Protection and Allocation</hd
><p
>Dynamic matching of access criteria provides a versatile way to implement and describe a variety of mechanisms for restricting or sharing access to properties. See accessMatch behavior for a discussion of dynamic matching.</p
></section
><section
><hd
>Multiple Windows on a Single Window Bound Property</hd
><p
>The case of multiple accessWindows on the same window bound property is permitted. The window bound property may then be read or written via any accessWindow which is enabled.</p
></section
><section
><hd
>Multiple Windows Controlled by the Same set of Matches</hd
><p
>This is a common situation where a group of accessWindows are controlled by the same accessMatch property or set of properties. This allows a single property access control mechanism to apply to a group of properties. This may be specified either by sharing or by referencing the same accessMatch properties from multiple bindings.</p
></section
><section
><hd
>Behaviors and Constraints on Window Bound Properties</hd
><p
>When access to a window bound property is enabled via an accessWindow, the behaviors and constraints (min, max, units etc.) on that property shall be the aggregation of all behaviors and constraints on the accessWindow and on the window bound property itself. This rule excludes those behaviors directly concerned in describing the accessWindow/window bound relationship. It is an error if there is a contradiction between a behavior or constraint on the accessWindow and one on the window bound property.</p
></section
></section
></behaviordef
><behaviordef
name="accessMatch"
><label
key="accessMatch"
set="acnbase.lset"
></label
><refines
name="binder"
set="acnbase.bset"
></refines
><section
><hd
>Match Property Enabling an Access Window</hd
><p
>This property enables a binding between an accessWindow and a window bound property. It must be associated with the binding in the same way as any binder association.</p
><section
><hd
>Static and Dynamic Matching</hd
><p
>The match criteria specified by an accessMatch property may be static or dynamic.</p
><section
><hd
>Static</hd
><p
>A static match corresponds to a boolean property value which is enabled or disabled based on the device state (including property values etc.). This allows the accessWindow to provide a multiplexed access to multiple properties, controlled by simple match values.</p
></section
><section
><hd
>Dynamic</hd
><p
>A dynamic match occurs when the accessMatch specifies a condition which depends dynamically on the access operation. An example of a dynamic match is a connectionMatch where the activation of the binding depends on which connection is used to read or write to the window. With a single binding, it means that only accesses via the specified connection will “see” the window bound property. With multiple bindings, it means that different controllers (which by definition use different connections) can see different viewed properties at the same window address.</p
></section
></section
><section
><hd
>Value of accessMatch Property</hd
><p
>An accessMatch binder whether static or dynamic has two states: enabled and disabled. These two states should be directly associated with a boolean value (which may be driven by other conditions) for example see accessInhibit. A dynamic match property may also have additional match criteria which must be satisfied dynamically during the access for the match to succeed. These criteria typically use additional sub-properties of the accessMatch, but must be specified by refinements of this behavior.</p
></section
><section
><hd
>Special Case – Window Bound accessMatch</hd
><p
>A special case occurs when an accessMatch is itself window bound and the binding specifies the same accessMatch property as its binder (by reference). In this special case, the access is enabled by the window bound accessMatch itself. This special case means that when the match is enabled, it may be accessed via the window, but when diabled it may not.</p
><p
>This special case is useful where a accessMatch is shared between several accessWindows and where matching is dynamic. The properties which are locked or assigned (e.g. to a particular connection), may be released by disabling the accessMatch via its own access window. Once disabled though, its window cannot normally be used to re-enable it and some other mechanism may be used.</p
></section
></section
></behaviordef
><behaviordef
name="accessEnable"
><label
key="accessEnable"
set="acnbase.lset"
></label
><refines
name="accessMatch"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><section
><hd
>Access Window Enable</hd
><p
>This is a boolean property in which a value of true enables the associated accessWindow.</p
><p
>Refinements or driven values of accessEnable allow a wealth of enable conditions to be described. Note that refinements, particularly dynamic ones, may require additional criteria beyond a value of true, for the window to be matched.</p
></section
><section
><hd
>See also</hd
><p
>accessInhibit</p
></section
></behaviordef
><behaviordef
name="accessInhibit"
><label
key="accessInhibit"
set="acnbase.lset"
></label
><refines
name="accessMatch"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><section
><hd
>Access Window Inhibit</hd
><p
>This is a boolean property in which a value of false enables the associated accessWindow.</p
><p
>Refinements or driven values of accessInhibit allow a wealth of enable conditions to be described.</p
><p
>Because of the definition of type.boolean, this allows a multivalued value to be used in which any value except zero (false) disconnects the accessWindow.</p
></section
><section
><hd
>See also</hd
><p
>accessEnable</p
></section
></behaviordef
><behaviordef
name="dynamicAccessEnable"
><label
key="dynamicAccessEnable"
set="acnbase.lset"
></label
><refines
name="accessEnable"
set="acnbase.bset"
></refines
><section
><hd
>Dynamic Access Window Matching Property</hd
><p
>This property describes a dynamic access matching criterion enabling context sensitive access to a window bound property via an accessWindow. The accessWindow associated with a binding controlled by a dynamicAccessEnable property shall have contextMatchWindow behavior or a refinement of it (contextMatchWindow is a refinement of accessWindow).</p
><p
>The value of the property represents a static enabled or disabled state in the same way as accessEnable behavior. When false, access is inhibited. When true, access is dependent on dynamic matching criteria such as the connection used for the access operation. The specific criterion for access must be defined by refinements of this behavior.</p
><p
>Dynamic criteria used in satisfying a dynamicAccessEnable apply specifically to accesses to any accessWindow property which forms one endpoint of any binding this property controls.</p
><p
>When dynamic matching is used as a method for allocating access to properties (see accessWindow), the value of the dynamicAccessEnable boolean indicates whether the window bound property is allocated or unallocated.</p
></section
></behaviordef
><behaviordef
name="connectionMatch"
><label
key="connectionMatch"
set="acnbase.lset"
></label
><refines
name="dynamicAccessEnable"
set="acnbase.bset"
></refines
><section
><hd
>Dynamic AccessMatch by Transport Connection</hd
><p
>The value of this property is a dynamicAccessEnable but shall also carry a connectedState behavior. Its boolean state is either false (unallocated) or true and associated with a specific transportConnection (see connectedState and conntrack behaviors). The tracked-connection must then be dynamically matched by any access command to the accessWindow for it to succeed. Access from the specified connection is enabled only while that connection is active or until the connectedState is written false (if it is writable). This has the effect of locking access to the window bound property exclusively to the specified connection.</p
><p
>Note that the boolean value of the connectionMatch property itself (and its transportConnection child if provided) are not dynamic and if they are network readable, they can be read from any connection (subject to other access rules). This allows other controllers to interrogate the state of a connectionMatch.</p
></section
><section
><hd
>Explicit vs Automatic Assignment to Connections</hd
><p
>A connectionMatch property shall also have a connectedState behavior. The connection to be matched shall then be the connection which is tracked by that connectedState. connectedState has two principal refinements which determine the way the connection is assigned.</p
><section
><hd
>Explicit Assignment of Matched Connection</hd
><p
>Where this connectionMatch property also has explicitConnectedState behavior, assignment of a track-target property to the associated window is explicitly determined as described in explicitConnectedState.</p
></section
><section
><hd
>Auto Assigned Connection Matching</hd
><p
>Where the connectionMatch property also has autoConnectedState behavior, it is automatically assigned and its associated accessWindow shall be an autoAssignContextWindow (or refinement). In this case, the track-target property is defined to be the window bound property as seen through the autoAssignContextWindow.</p
></section
></section
><section
><hd
>See also</hd
><p
>contextMatchWindow, autoAssignContextWindow</p
><p
>conntrack behaviorset (995b6b00-ac75-11db-a7b5-0017316c497d): connectedState, explicitConnectedState, autoConnectedState.</p
></section
></behaviordef
><behaviordef
name="contextMatchWindow"
><label
key="contextMatchWindow"
set="acnbase.lset"
></label
><refines
name="accessWindow"
set="acnbase.bset"
></refines
><section
><hd
>Dynamic Context Sensitive Access Window</hd
><p
>This is an accessWindow type which requires a dynamic context sensitive match to enable access. Examples include matches which depend on aspects of the network protocols used in accessing the accessWindow, such as the transport connection, source address and network, routing parameters, etc.</p
><p
>The accessEnable property associated with each binding to a contextMatchWindow shall have dynamicAccessEnable behavior (or refinement). If not specified by refinement of this behavior, the context matching criteria must be determined by the specific refinement of the dynamicAccessEnable used. For example, see connectionMatch.</p
></section
><section
><hd
>See also</hd
><p
>dynamicAccessEnable, connectionMatch, autoAssignContextWindow.</p
></section
></behaviordef
><behaviordef
name="autoAssignContextWindow"
><label
key="autoAssignContextWindow"
set="acnbase.lset"
></label
><refines
name="contextMatchWindow"
set="acnbase.bset"
></refines
><section
><hd
>Automatic Assigning Context Match Window</hd
><p
>Context sensitive dynamic matching provides a mechanism which allows an accessWindow property to expose or hide target window bound properties depending on the context of the access to the window (see contextMatchWindow and dynamicAccessEnable). This allows resources (represented by access to the window bound property) to be allocated or restricted in a flexible manner.</p
><p
>An autoAssignContextWindow extends this mechanism to provide a generic first come, first served resource allocation mechanism.</p
><p
>Each binding to an autoAssignContextWindow shall be controlled by a dynamicAccessEnable property. This may be false (binding unallocated) or true (binding allocated to a specific context defined by refinement of dynamicAccessEnable). With this behavior, when an access attempt is made from a new context, an unallocated binding (if one is available) is automatically assigned to the new context, the value of its dynamicAccessEnable binder set to true and its match criteria set up accordingly. This then “locks” the binding to accesses matching that context. The lock may be relinquished by explicitly setting the dynamicAccessEnable property to false (unallocated).</p
><p
>If there is no unallocated binding associated with this window, then any access from an unallocated context shall fail.</p
><p
>Where multiple bindings are available for assignment, only one shall be assigned to any new context. The choice of which one is not specified in this behavior and is therefore dependent on the implementation. Further refinements may be more specific.</p
><p
>If not specified by refinement of this behavior, the context matching criteria must be determined by the specific refinement of the dynamicAccessEnable used.</p
><section
><hd
>Special Case – Window Bound dynamicAccessEnable properties with Auto Assignment</hd
><p
>The special case of a window bound accessMatch is mentioned in accessMatch behavior. By extension from that, a dynamicAccessEnable may be window bound.</p
><p
>If the binding from an autoAssignContextWindow to a dynamicAccessEnable property references the same dynamicAccessEnable property as its own binder, then it is allowable to write the value enabled (true) to the autoAssignContextWindow. Then if the connection used to write the value is not already assigned a binding, and there is one or more unassigned bindings available, one binding will be assigned and its dynamicAccessEnable set true, enabling the binding which it is itself gating.</p
><p
>Simply reading the dynamicAccessEnable via its autoAssignContextWindow shall not cause assignment and shall fail if the connection performing the read is not already assigned a binding.</p
></section
></section
><section
><hd
>Examples</hd
><section
><hd
>Straightforward Auto Assigning Connection Lock</hd
><p
>The first property has an implied value but is accessed via an autoAssignContextWindow and a connectionMatch which when enabled, associates access with a specific connection. This automatically locks to a connection on a first come first served basis. Once assigned, that connection gains exclusive access until either it is terminated, or a value of false is written to the connectionMatch property.</p
><p
xml:space="preserve"
>&lt;property valuetype="implied" xml:id="TARGETPROP"&gt;
  &lt;label&gt;Connection Locked Target Property&lt;/label&gt;
  ...
&lt;/property&gt;

&lt;property valuetype="network"&gt;
  &lt;label&gt;Access Property for Connection Locked target&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="autoAssignContextWindow"&gt;
  &lt;behavior set="acnbase.bset" name="bindingAnchor"&gt;
  &lt;!-- define read/write access to window property --&gt;
  &lt;protocol ...&gt;...&lt;/protocol&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;label&gt;Pointer to target&lt;/label&gt;
    &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
    &lt;behavior set="acnbase.bset" name="internalBidiRef"/&gt;
    &lt;value type="string"&gt;TARGETPROP&lt;/value&gt;
    &lt;property valuetype="network"&gt;
      &lt;label&gt;Connection Match&lt;/label&gt;
      &lt;behavior set="acnbase.bset" name="connectionMatch"&gt;
      &lt;behavior set="acnbase.bset" name="writeConnectedState"&gt;
      &lt;!--
        define read/write access to match property
        allowing assigned match to be relinquished
      --&gt;
      &lt;protocol ...&gt;...&lt;/protocol&gt;
      &lt;property valuetype="network"&gt;
        &lt;label&gt;The assigned connection&lt;/label&gt;
        &lt;behavior set="acnbase.bset" name="connection.ESTA.SDT.ESTA.DMP"/&gt;
        &lt;behavior set="acnbase.bset" name="autoTrackedConnection"/&gt;
        &lt;!--
          define read only access to the connection number
          and owner CID as defined in connection.ESTA.SDT.ESTA.DMP
        --&gt;
      &lt;/property&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
></section
><section
><hd
>Allocation of Two “Slots” (window bound properties) via a Common Window</hd
><p
>Two upermost properties labelled Slot 1 and Slot 2 are defined which are both bound to a common autoAssignContextWindow. In this case, the slots are the anchors of their respective bindings. This automatically assigns a slot to each of the first two connections on a first come first served basis. Once assigned, a connection gains exclusive access to its slot until either it is terminated, or a value of false is written to the connectionMatch property. The connectionMatch property is itself accessed via a autoAssignConnectionMatchWindow providing a single point of access irrespective of which slot has been assigned.</p
><p
>It is presumed that each slot represents some resource wich is used in further structures in the device – for example they could represent separate inputs to a dimmer device which are then combined on an HTP basis.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;Slot 1&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="bindingAnchor"&gt;
  &lt;!-- define read-only access to slot property --&gt;
  &lt;protocol ...&gt;...&lt;/protocol&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior set="acnbase.bset" name="internalBidiRef"&gt;
    &lt;value type="string"&gt;SLOT-WINDOW&lt;/value&gt;
    &lt;property valuetype="network" xml:id="CONNMATCH1"&gt;
      &lt;label&gt;Connection Match&lt;/label&gt;
      &lt;behavior set="acnbase.bset" name="connectionMatch"&gt;
      &lt;behavior set="acnbase.bset" name="writeConnectedState"&gt;
      &lt;behavior set="acnbase.bset" name="bindingAnchor"&gt;
      &lt;protocol ...&gt;...&lt;/protocol&gt;
      &lt;property valuetype="network"&gt;
        &lt;label&gt;The assigned connection&lt;/label&gt;
        &lt;behavior set="acnbase.bset" name="connection.ESTA.SDT.ESTA.DMP"/&gt;
        &lt;behavior set="acnbase.bset" name="autoTrackedConnection"/&gt;
        &lt;protocol ...&gt;...&lt;/protocol&gt;
        &lt;property ...&gt;...&lt;/property&gt;
      &lt;/property&gt;
      &lt;property valuetype="immediate"&gt;
        &lt;behavior set="acnbase.bset" name="internalBidiRef"&gt;
        &lt;value type="string"&gt;STATE-WINDOW&lt;/value&gt;
        &lt;property valuetype="immediate"/&gt;
          &lt;behavior set="acnbase.bset" name="connectionMatch"/&gt;
          &lt;value type="string"&gt;CONNMATCH1&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;

&lt;property valuetype="network"&gt;
  &lt;label&gt;Slot 2&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="bindingAnchor"&gt;
  &lt;!-- define read-only access to slot property --&gt;
  &lt;protocol ...&gt;...&lt;/protocol&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior set="acnbase.bset" name="internalBidiRef"&gt;
    &lt;value type="string"&gt;SLOT-WINDOW&lt;/value&gt;
    &lt;property valuetype="network" xml:id="CONNMATCH2"&gt;
      &lt;label&gt;Connection Match&lt;/label&gt;
      &lt;behavior set="acnbase.bset" name="connectionMatch"&gt;
      &lt;behavior set="acnbase.bset" name="writeConnectedState"&gt;
      &lt;behavior set="acnbase.bset" name="bindingAnchor"&gt;
      &lt;protocol ...&gt;...&lt;/protocol&gt;
      &lt;property valuetype="network"&gt;
        &lt;label&gt;The assigned connection&lt;/label&gt;
        &lt;behavior set="acnbase.bset" name="connection.ESTA.SDT.ESTA.DMP"/&gt;
        &lt;behavior set="acnbase.bset" name="autoTrackedConnection"/&gt;
        &lt;protocol ...&gt;...&lt;/protocol&gt;
        &lt;property ...&gt;...&lt;/property&gt;
      &lt;/property&gt;
      &lt;property valuetype="immediate"&gt;
        &lt;behavior set="acnbase.bset" name="internalBidiRef"&gt;
        &lt;value type="string"&gt;STATE-WINDOW&lt;/value&gt;
        &lt;property valuetype="immediate"/&gt;
          &lt;behavior set="acnbase.bset" name="connectionMatch"/&gt;
          &lt;value type="string"&gt;CONNMATCH2&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;

&lt;property valuetype="network" xml:id="SLOT-WINDOW"&gt;
  &lt;label&gt;Window onto slot 1 or slot 2&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="autoAssignContextWindow"&gt;
  &lt;!-- define read-write access to window property --&gt;
  &lt;protocol ...&gt;...&lt;/protocol&gt;
&lt;/property&gt;

&lt;property valuetype="network" xml:id="STATE-WINDOW"&gt;
  &lt;label&gt;Window onto slot 1 or slot 2 assigned state&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="autoAssignContextWindow"&gt;
  &lt;!-- define read-write access to window property --&gt;
  &lt;protocol ...&gt;...&lt;/protocol&gt;
&lt;/property&gt;</p
></section
></section
></behaviordef
><!--Preferred Values--><behaviordef
name="preferredValue.abstract"
><label
key="preferredValue.abstract"
set="acnbase.lset"
></label
><section
><hd
>Introduction – Preferred Stopping Points</hd
><p
>In many cases a property may take any value over a range (scalar behaviour), yet has ?nautral? or preferred, stopping points.</p
><p
>An example would be the position of a train along a track? the train can be positioned anywhere but there are “natural” stopping points at stations and signals.</p
><p
>In the case of the train, the stopping points occur at arbitrary positions while in other systems they occur at regularly repeating values throughout the range. For example, the hand of a clock might have “natural” positions at each minute mark.</p
></section
><section
><hd
>Abstract Preferred Value</hd
><p
>A preferred value property indicates an preferred “stopping point” in the range of its parent property. A property may have any number of preferred values. this is an abstract behavior. Refinements allow individual arbitrary values or regular patterns of values to be defined.</p
><p
>Label properties may be attached to preferred values (as sub-properties) which allows a label to be associated with each position.</p
></section
></behaviordef
><behaviordef
name="preferredValue"
><label
key="preferredValue"
set="acnbase.lset"
></label
><refines
name="preferredValue.abstract"
set="acnbase.bset"
></refines
><section
><hd
>Preferred Value</hd
><p
>A preferred value property defines a single “stopping point” value. Size and type shall be the same as the parent property.</p
></section
></behaviordef
><behaviordef
name="repeatPrefVal"
><label
key="repeatPrefVal"
set="acnbase.lset"
></label
><refines
name="preferredValue.abstract"
set="acnbase.bset"
></refines
><section
><hd
>Repeating Preferred Value</hd
><p
>This defines a repeating set of preferred values at regular intervals. Its value is the interval or gap between repeats. It may contain children further defining the position or range of repeating values.</p
><p
>If no position or range are defined otherwise by child properties, then there shall be a preferred value at value zero and repeating values at the given interval in both positive and negative directions up to the limits of the parent property.</p
></section
></behaviordef
><behaviordef
name="repeatPrefValOffset"
><label
key="repeatPrefValOffset"
set="acnbase.lset"
></label
><refines
name="preferredValue.abstract"
set="acnbase.bset"
></refines
><section
><hd
>Repeating Preferred Value Offset</hd
><p
>This property shall be a child of a repeatPrefVal property and offsets the entire set of repeating values generated such that one value occurs at the position given by this property.</p
><section
><hd
>example – value with 'click stops'</hd
><p
>The first property “clickstop 1” has no repeatPrefValOffset child and has preferred values at ...-20, -10, 0, 10, 20, 30...</p
><p
>The second property “clickstop 2” has preferred values at ... -17, -7, 3, 13, 23, 33...</p
><p
xml:space="preserve"
>    &lt;property valuetype="network"&gt;
      &lt;label&gt;clickstop 1&lt;/label&gt;
      &lt;behavior name="type.sint" set="acnbase.bset" /&gt;
      &lt;protocol protocol="ESTA.DMP"&gt;...&lt;/protocol&gt;
      
      &lt;property valuetype="immediate"&gt;
        &lt;behavior name="repeatPrefVal" set="acnbase.bset" /&gt;
        &lt;value type="uint"&gt;10&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;
      
    &lt;property valuetype="network"&gt;
      &lt;label&gt;clickstop 2&lt;/label&gt;
      &lt;behavior name="type.sint" set="acnbase.bset" /&gt;
      &lt;protocol protocol="ESTA.DMP"&gt;...&lt;/protocol&gt;
      
      &lt;property valuetype="immediate"&gt;
        &lt;behavior name="repeatPrefVal" set="acnbase.bset" /&gt;
        &lt;value type="uint"&gt;10&lt;/value&gt;

        &lt;property valuetype="immediate"&gt;
          &lt;behavior name="repeatPrefVal" set="acnbase.bset" /&gt;
          &lt;value type="sint"&gt;3&lt;/value&gt;
        &lt;/property&gt;

      &lt;/property&gt;
    &lt;/property&gt;</p
></section
></section
></behaviordef
><!--Selectors and choices--><behaviordef
name="selected"
><label
key="selected"
set="acnbase.lset"
></label
><refines
name="driven"
set="acnbase.bset"
></refines
><section
><hd
>Selected Property</hd
><p
>A selected property is a driven property whose value is taken from one of a limited set of alternatives which have choice behavior. It contains a selector child property (a driver) which is used to select one of the available alternative values. The set of alternative values are children of the selector.</p
><p
>Each selected property shall have exactly one selector child.</p
></section
></behaviordef
><behaviordef
name="selector"
><label
key="selector"
set="acnbase.lset"
></label
><refines
name="driver"
set="acnbase.bset"
></refines
><section
><hd
>Selector Property</hd
><p
>A selector is used to select one value from a number of alternative choices or cases (see “selected” behavior). Refinements of this behavior determine how the selection is made.</p
><p
>The set of alternatives from which the selection is made have choice behavior and are descendents of the selector which must contain sufficient choices to satisfy each selector value.</p
></section
></behaviordef
><behaviordef
name="choice"
><label
key="choice"
set="acnbase.lset"
></label
><refines
name="driver"
set="acnbase.bset"
></refines
><section
><hd
>Choice Property</hd
><p
>This property provides one choice item of a multiple choice selection. It's value type must match the value of the property whose value it is driving. Choice is a child (or descendent) of the selector property.</p
><p
>Depending on the selection method (see refinements of selector) listing of choice properties is often order sensitive and must match the ordering of selector values. If the choices have inherent order then the selector shall be declared with “ordered” behavior.</p
><p
>“label” sub-properties attached to choice properties provide a useful way to associate variable labels flexibly with the choices.</p
><p
>See enumSelector and case for examples.</p
></section
></behaviordef
><behaviordef
name="enumSelector"
><label
key="enumSelector"
set="acnbase.lset"
></label
><refines
name="selector"
set="acnbase.bset"
></refines
><refines
name="type.enum"
set="acnbase.bset"
></refines
><section
><hd
>Enumerated Selector Property</hd
><p
>A selector which simply enumerates the choices. The value ranges from zero to N-1 where N is the number of choices.</p
><section
><hd
>Example of selected, enumerated selector and choice usage</hd
><p
>The mains voltage for the device is selected via a selector property with three choices.</p
><p
>The networked property at DMP address 25 takes the value 0 for 100V, 1 for 120V or 2 for 230V. Only one networked property is declared, the choices and labels have immediate values and the voltage property is implied – a driven property which is not itself accessible.</p
><p
>The labels on the voltages are declared as string references and so are subject to language substitution.</p
><p
xml:space="preserve"
>&lt;!-- The voltage property itself is not accessible (implied property value) --&gt;
    &lt;property valuetype="implied"&gt;
      &lt;label&gt;Mains Voltage&lt;/label&gt;
      &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
      &lt;behavior name="selected" set="acnbase.bset" /&gt;

&lt;!-- the selector chooses which voltage is used --&gt;
      &lt;property valuetype="network"&gt;
        &lt;label&gt;Voltage selector&lt;/label&gt;
        &lt;behavior name="enumSelector" set="acnbase.bset" /&gt;
        &lt;behavior name="ordered" set="acnbase.bset" /&gt;
        &lt;behavior name="persistent" set="acnbase.bset" /&gt;
        &lt;protocol protocol="ESTA.DMP"&gt;
          &lt;propref_DMP abs="true" read="true" write="true" size="1" loc="25"/&gt;
        &lt;/protocol&gt;

        &lt;!--
            now an array of three possible choices
            each has an associated label
        --&gt;
        &lt;property array="3" valuetype="immediate"&gt;
          &lt;behavior name="choice" set="acnbase.bset" /&gt;
          &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
          &lt;value type="uint"&gt;100&lt;/value&gt;
          &lt;value type="uint"&gt;120&lt;/value&gt;
          &lt;value type="uint"&gt;230&lt;/value&gt;

          &lt;property valuetype="immediate"&gt;
            &lt;behavior name="labelString" set="acnbase.bset" /&gt;
            &lt;value type="string"&gt;Japan&lt;/value&gt;
            &lt;value type="string"&gt;Americas&lt;/value&gt;
            &lt;value type="string"&gt;Europe&lt;/value&gt;
          &lt;/property&gt;
        &lt;/property&gt;
      &lt;/property&gt;
    &lt;/property&gt;</p
></section
></section
></behaviordef
><behaviordef
name="fractionalSelector"
><label
key="fractionalSelector"
set="acnbase.lset"
></label
><refines
name="selector"
set="acnbase.bset"
></refines
><refines
name="measure"
set="acnbase.bset"
></refines
><section
><hd
>Fractional Selector Property</hd
><p
>This property selects an item using a mechanism which can allow intermediate positions between the selections. This is common in real applications where selection involves physical movement.</p
><section
><hd
>Example</hd
><p
>Consider a color selection wheel used in lighting or photographic applications which has a number of different colored panes arranged around its axis. Rotating the wheel allows one of the colored panes to be selected. However, in many cases, there is nothing to prevent the wheel being positioned at an intermediate point so that a part of each of two panes is selected.</p
><p
>In some cases this behavior is desirable while in others it could be disastrous (consider the tool selector on a machine press punch!).</p
><p
>Note this color wheel could also be a cyclic property, but that is not implicit to fractional selectors and would have to be declared separately.</p
></section
><p
>A fractional selector uses “case” properties to mark the positions where a normal selection is made, and any value in between will be a fractional selection. Each case must have a choice child-element indicating the choice associated with that position.</p
><p
>The meaning of intermediate positions is not defined, although refinements may do so – for example a fractional selector could be used to select apertures in a photographic application. If the underlying mechanism was an iris, then intermediate values could well represent intermediate apertures, but if the mechanism was a wheel with a set of fixed apertures around it then intermediate values would produce apertures which were off-centre or no aperture at all.</p
><p
>For an example of use see case behavior (the example given uses a positional selector which is a refinement of this behavior).</p
></section
></behaviordef
><behaviordef
name="positionalSelector"
><label
key="positionalSelector"
set="acnbase.lset"
></label
><refines
name="fractionalSelector"
set="acnbase.bset"
></refines
><section
><hd
>Positional Selector</hd
><p
>This property is a fractional selector where the mechanism of selection is explicitly one where the selected items are arranged in sequence on a carrier and chosen by moving into position.</p
><p
>Progression from selection point to the next moves the first selection more and more out of alignment and then the second gradually into alignment. Typical examples are rotating wheel style selectors or sliding linear positioners.</p
><p
>For an example of use see case behavior.</p
></section
></behaviordef
><behaviordef
name="case"
><label
key="case"
set="acnbase.lset"
></label
><refines
name="preferredValue"
set="acnbase.bset"
></refines
><refines
name="driver"
set="acnbase.bset"
></refines
><section
><hd
>Case – One of a Set of Cases</hd
><p
>This property defines a set of properties applied when an associated property (such as a fractional selector) matches its value.</p
><p
>A property may contain any number of case sub-properties. When the value of the associated fractional selector (or similar) property matches the value of the case property, all children of the case property are considered to be applied to its parent as though they were its direct children.</p
><section
><hd
>Example of Positional Selctor and Case Behaviors</hd
><p
>In this example a property with (ficticious) behavior “apertureSize” is driven by a wheel containing three apertures with sizes 0.1, 0.2, 0.5. whose position is driven by DMP property at location 5. The actual position of the wheel is accessible at DMP location 4. When the wheel is at intermediate positions, the apertures are out of alignment as described in positional selector behavior.</p
><p
xml:space="preserve"
>&lt;property valuetype="implied"&gt;
  &lt;label&gt;Selected aperture&lt;/label&gt;
  &lt;behavior name="apertureSize" set="a.n.other" /&gt;
  &lt;behavior name="driven" set="acnbase.bset" /&gt;
  &lt;behavior name="type.float" set="acnbase.bset" /&gt;

&lt;!-- specify scale and units for aperture here --&gt;

  &lt;property valuetype="network"&gt;
    &lt;label&gt;Aperture selector position&lt;/label&gt;
    &lt;behavior name="positionalSelector" set="acnbase.bset" /&gt;
    &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
    &lt;behavior name="cyclic" set="acnbase.bset" /&gt;
    &lt;behavior name="driven" set="acnbase.bset" /&gt;
    &lt;protocol protocol="ESTA.DMP"&gt;
      &lt;propref_DMP abs="true" read="true" size="1" loc="4"/&gt;
    &lt;/protocol&gt;

&lt;!-- this property selects the desired wheel position --&gt;
    &lt;property valuetype="network"&gt;
      &lt;label&gt;Desired aperture selection&lt;/label&gt;
      &lt;behavior name="target" set="acnbase.bset" /&gt;
      &lt;protocol protocol="ESTA.DMP"&gt;
        &lt;propref_DMP abs="true" read="true" write="true" size="1" loc="5"/&gt;
      &lt;/protocol&gt;
    &lt;/property&gt;

  &lt;!-- Aperture 1: size 0.1 at wheel position 0 --&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;label&gt;Aperture position&lt;/label&gt;
      &lt;behavior name="case" set="acnbase.bset" /&gt;
      &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
      &lt;value type="uint"&gt;0&lt;/value&gt;      
      &lt;property valuetype="immediate"&gt;
        &lt;label&gt;Aperture position&lt;/label&gt;
        &lt;behavior name="apertureSize" set="a.n.other" /&gt;
        &lt;behavior name="choice" set="acnbase.bset" /&gt;
        &lt;behavior name="type.float" set="acnbase.bset" /&gt;
        &lt;value type="float"&gt;0.1&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;

  &lt;!-- Aperture 2: size 0.2 at wheel position 64 --&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;label&gt;Aperture position&lt;/label&gt;
      &lt;behavior name="case" set="acnbase.bset" /&gt;
      &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
      &lt;value type="uint"&gt;64&lt;/value&gt;      
      &lt;property valuetype="immediate"&gt;
        &lt;label&gt;Aperture position&lt;/label&gt;
        &lt;behavior name="apertureSize" set="a.n.other" /&gt;
        &lt;behavior name="choice" set="acnbase.bset" /&gt;
        &lt;behavior name="type.float" set="acnbase.bset" /&gt;
        &lt;value type="float"&gt;0.2&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;

  &lt;!-- Aperture 3: size 0.5 at wheel position 128 --&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;label&gt;Aperture position&lt;/label&gt;
      &lt;behavior name="case" set="acnbase.bset" /&gt;
      &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
      &lt;value type="uint"&gt;128&lt;/value&gt;      
      &lt;property valuetype="immediate"&gt;
        &lt;label&gt;Aperture position&lt;/label&gt;
        &lt;behavior name="apertureSize" set="a.n.other" /&gt;
        &lt;behavior name="choice" set="acnbase.bset" /&gt;
        &lt;behavior name="type.float" set="acnbase.bset" /&gt;
        &lt;value type="float"&gt;0.5&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;

  &lt;/property&gt;
&lt;/property&gt;</p
></section
></section
></behaviordef
><!--Behaviors for control of Cyclic properties--><behaviordef
name="cyclicPath"
><label
key="cyclicPath"
set="acnbase.lset"
></label
><refines
name="algorithm"
set="acnbase.bset"
></refines
><section
><hd
>Cyclic Path</hd
><p
>If a cyclic property does not simply change instantly from one value to another but needs to progress through intermediate values – either because of its mechanical configuration or because of other constraints such as rate of change restrictions – then a controller may need to know or control the algorithm for doing this. cyclicPath is an abstract behavior whose refinements identify specific algorithms used by devices to determine which path to use.</p
><section
><hd
>Example of static Cyclic Path declaration</hd
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;Ship's bearing&lt;/label&gt;
  &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
  &lt;behavior name="cyclic" set="acnbase.bset" /&gt;
  &lt;behavior name="cyclicPath.shortest" set="acnbase.bset" /&gt;
  &lt;protocol protocol="ESTA.DMP"&gt;
    &lt;propref_DMP abs="true" read="true" size="1" loc="5"/&gt;
  &lt;/protocol&gt;

  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="limitMinInc" set="acnbase.bset" /&gt;
    &lt;value type="uint"&gt;0&lt;/value&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="limitMaxExc" set="acnbase.bset" /&gt;
    &lt;value type="uint"&gt;360&lt;/value&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="units" set="acnbase.bset" /&gt;
    &lt;value type="string"&gt;°&lt;/value&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
><p
>The ship's bearing property (at DMP address 5) varies from 0°—360° and takes the shortest path to reach any new value. Thus if current bearing is 315° (NW) and a new bearing of 45° (NE) is specified, the ship will turn clockwise through North.</p
></section
><section
><hd
>Example of dynamic Cyclic Path Assignment</hd
><p
>This example takes advantage of the selector and behaviorRef behaviors.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;Turntable position&lt;/label&gt;
  &lt;behavior name="type.uint" set="acnbase.bset" /&gt;
  &lt;behavior name="cyclic" set="acnbase.bset" /&gt;
  &lt;behavior name="angle-deg" set="acnbase.bset" /&gt;
  &lt;protocol protocol="ESTA.DMP"&gt;
    &lt;propref_DMP abs="true" read="true" size="2" loc="28"/&gt;
  &lt;/protocol&gt;

  &lt;property valuetype="implied"&gt;
    &lt;behavior name="behaviorRef" set="acnbase.bset" /&gt;
    &lt;behavior name="selected" set="acnbase.bset" /&gt;

&lt;!-- this property selects the behavior --&gt;
    &lt;property valuetype="network"&gt;
      &lt;label&gt;Operating mode select&lt;/label&gt;
      &lt;behavior name="enumSelector" set="acnbase.bset" /&gt;
      &lt;protocol protocol="ESTA.DMP"&gt;
        &lt;propref_DMP abs="true" read="true" write="true" size="1" loc="30"/&gt;
      &lt;/protocol&gt;
    &lt;/property&gt;

&lt;!-- here are the four behavior choices --&gt;
    &lt;property array="4" valuetype="immediate"&gt;
      &lt;behavior name="choice" set="acnbase.bset" /&gt;
      &lt;behavior name="behaviorRef" set="acnbase.bset" /&gt;
      &lt;value type="string"&gt;cyclicPath.shortest&lt;/value&gt;
      &lt;value type="string"&gt;cyclicPath.increasing&lt;/value&gt;
      &lt;value type="string"&gt;cyclicPath.decreasing&lt;/value&gt;
      &lt;value type="string"&gt;cyclicPath.linear&lt;/value&gt;
      &lt;property valuetype="immediate"&gt;
        &lt;behavior name="behaviorset" set="acnbase.bset" /&gt;
        &lt;value type="object"&gt;71576eac-e94a-11dc-b664-0017316c497d&lt;/value&gt;
      &lt;/property&gt;
      &lt;property valuetype="immediate"&gt;
        &lt;behavior name="labelString" set="acnbase.bset"/&gt;
        &lt;value type="string"&gt;Shortest&lt;/value&gt;
        &lt;value type="string"&gt;Clockwise&lt;/value&gt;
        &lt;value type="string"&gt;Counter-clockwise&lt;/value&gt;
        &lt;value type="string"&gt;No wraparound&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;
  &lt;/property&gt;

  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="limitMinInc" set="acnbase.bset" /&gt;
    &lt;value type="uint"&gt;0&lt;/value&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="limitMaxExc" set="acnbase.bset" /&gt;
    &lt;value type="uint"&gt;360&lt;/value&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior name="scale" set="acnbase.bset" /&gt;
    &lt;value type="float"&gt;0.1&lt;/value&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
><p
>The turntable position property is at DMP address 28 and represents the desired position of the turntable (0°—360° in 0.1° steps). A selector property at DMP address 30 allows the controller to select which algorithm is to be used to rotate to any new target from four different choices labelled: “Shortest”, “Clockwise”, “Counter-clockwise” or “No wraparound”. See specific behaviors cyclicPath.shortest, cyclicPath.increasing, cyclicPath.decreasing and cyclicPath.linear for detailed definitions.</p
></section
></section
></behaviordef
><behaviordef
name="cyclicDir.increasing"
><label
key="cyclicPath.increasing"
set="acnbase.lset"
></label
><refines
name="cyclicPath"
set="acnbase.bset"
></refines
><section
><hd
>Increasing Cyclic Direction</hd
><p
>A cyclic property with this behavior will always move between two values in an increasing direction. If the target value is smaller than the current value, the value will increase towards maximum, “wrap around” to minimum and then increase to the target.</p
><p
>See cyclic behavior and cyclicPath for more discussion.</p
></section
></behaviordef
><behaviordef
name="cyclicDir.decreasing"
><label
key="cyclicPath.decreasing"
set="acnbase.lset"
></label
><refines
name="cyclicPath"
set="acnbase.bset"
></refines
><section
><hd
>Decreasing Cyclic Direction</hd
><p
>A cyclic property with this behavior will always move between two values in a decreasing direction. If the target value is larger than the current value, the value will decrease towards minimum, “wrap around” to maximum and then decrease to the target.</p
><p
>See cyclic behavior and cyclicPath for more discussion.</p
></section
></behaviordef
><behaviordef
name="cyclicDir.shortest"
><label
key="cyclicPath.shortest"
set="acnbase.lset"
></label
><refines
name="cyclicPath"
set="acnbase.bset"
></refines
><section
><hd
>Shortest Path Cyclic Direction</hd
><p
>A cyclic property with this behavior will move between two values by the shortest path. The shortest path may be the same as for a linar property or may be by “wrapping around” through the maximum/minimum value depending on the current position and the target.</p
><p
>The device exposing the property chooses the shortest path and may or may not take into account considerations such as current speed and momentum in determining which direction to take. Refinements of this behavior may provide greater detail on specific algorithms.</p
><p
>See cyclic behavior and cyclicPath for more discussion.</p
></section
></behaviordef
><behaviordef
name="cyclicPath.scalar"
><label
key="cyclicPath.scalar"
set="acnbase.lset"
></label
><refines
name="cyclicPath"
set="acnbase.bset"
></refines
><section
><hd
>Scalar Path Cyclic Direction</hd
><p
>A cyclic property with this behavior will ignore the possibility of “wrapping around” through the maximum/minimum value and always move between positions as though it were a scalar property.</p
><p
>This behavior is unnecessary for static declaration since a property declared this way is identical to one declared without cyclic behavior. However, where cyclicPath behaviors may be dynamically determined, this behavior is a common possibility.</p
><p
>See cyclic behavior and cyclicPath for more discussion.</p
></section
></behaviordef
><!--Tracking the state of connections--><behaviordef
name="connectedState"
><label
key="connectedState"
set="acnbase.lset"
></label
><refines
name="connectionContextDependent"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><refines
name="trippable"
set="acnbase.bset"
></refines
><section
><hd
>State of a Connection</hd
><p
>A connectedState property tracks the state of a connection. It has two states connected (true) and unassigned (false). Once assigned to an active connection, it will remain true all the while that connection is alive, but reverts to false if that connection is terminated. A state of false means that the connectedState property is not tracking any particular connection. It does not mean that a particular connection does not exist. This also means that if a connection which was being tracked terminates, re-establishing that connection does not cause the connectedState to revert to true, because it will have become unassigned.</p
><section
><hd
>Writable connectedState values</hd
><p
>Setting a writable connectedState to false forces the connectedStat property to stop tracking the connection and return to the unassigned state. It does not force the device to break the connection (an action which should occur at the transport layer if required). Where the connectedState is combined with other behaviors to allocate or reserve resources in some way, these resources may be relinquished by setting the transportConnection child to unassigned.</p
></section
><section
><hd
>Determination of Connection Tracked</hd
><p
>The connection which it refers may be explicitly specified by a transportConnection child property, or it may be automatically assigned by whichever connection accesses some other tracked property. Two refinements of this behavior determine which case operates.</p
></section
></section
><section
><hd
>See Also</hd
><p
>connectedSwitch, autoConnectedState, explicitConnectedState.</p
><section
><hd
>Note: Relationship to connectedSwitch Behavior</hd
><p
>The connectedSwitch behavior provides a mechanism for describing simple connection dependency. However, unless the controller accesses the properties concerned in the correct order, there is risk of indeterminate intermediate conditions – for example if the connectedSwitch is turned on before the control property it selects has been initialized. This connectedState behavior is more flexible and allows a group of properties to attach behaviors to the state of a single connection.</p
></section
></section
></behaviordef
><behaviordef
name="autoConnectedState"
><label
key="autoConnectedState"
set="acnbase.lset"
></label
><refines
name="connectedState"
set="acnbase.bset"
></refines
><section
><hd
>State of a Connection – Connection Automatically Determined</hd
><p
>A connectedState property tracks the state of a connection – see connectedState behavior. Properties with this autoConnectedState refinement shall be automatically assigned to track whichever connection is used to access some particular track-target property or properties.</p
><p
>When the track-target is accessed by a particular connection, the autoConnectedState is assigned to that connection and becomes true (connected) and remains so while that connection is active. Where, more than one track-target property is present, then access to any of them assigns the autoConnectedState. It may become false (unassigned) by two means:</p
><p
>The tracked connection is terminated.</p
><p
>An explicit write of false (if it is writable).</p
><p
>Meanwhile, if the track-target property is accessed by another connection, the autoConnectedState shall be re-assigned to that connection and remains true.</p
><section
><hd
>Determination of Track-Target</hd
><p
>The track-target properties shall be determined by the following rules in order of precedence – highest first.</p
><section
><hd
>Specification by Reference</hd
><p
>If the autoConnectedState property has one or more trackTargetRef logical child properties, then those properties determines the track-target properties.</p
></section
><section
><hd
>Refinement of this Behavior</hd
><p
>A refinement of this behavior may explicitly specify the track-target property or properties.</p
></section
><section
><hd
>Refinement of another Behavior</hd
><p
>If the behavior of an associated property explicitly calls for an autoConnectedState property to track connections to a particular target property or properties, then that behavior determines the track-target(s).</p
></section
><section
><hd
>Implicit Parent</hd
><p
>If none of the above cases apply, then autoConnectedState indicates the state of the last access to its logical parent property. It thus indicates whether the connection last used to minitor or control that property is still active.</p
></section
></section
></section
><section
><hd
>Declaration of Tracked Connection</hd
><p
>Unless the connection tracked is declared elsewhere, an autoConnectedState property should have a autoTrackedConnection child property which is automatically updated to the connection being tracked whenever the autoConnectedState is assigned (true).</p
></section
><section
><hd
>See Also</hd
><p
>connectedSwitch, writeConnectedState, readConnectedState, autoTrackedConnection.</p
></section
></behaviordef
><behaviordef
name="explicitConnectedState"
><label
key="explicitConnectedState"
set="acnbase.lset"
></label
><refines
name="connectedState"
set="acnbase.bset"
></refines
><section
><hd
>State of an Explicitly Specified Connection</hd
><p
>A connectedState property tracks the state of a connection – see connectedState behavior. Properties with this explicitConnectedState refinement shall have an associated transportConnection property which is explicitly set to determine the connection to be tracked.</p
><p
>The connection to be tracked shall be specified in a transportConnection property which is either a logical child of this property, or is the target of a propertyRef which is a logical child of this property.</p
></section
></behaviordef
><behaviordef
name="writeConnectedState"
><label
key="writeConnectedState"
set="acnbase.lset"
></label
><refines
name="autoConnectedState"
set="acnbase.bset"
></refines
><section
><hd
>State of a Connection used for Writing</hd
><p
>This behavior shall follow all the rules of autoConnectedState except that the connection to be tracked shall only be assigned by write accesses to the track-target property, while read accesses have no effect. Thus it reflects the state of the connection last used to write to the track-target irrespective of intervening read accesses.</p
></section
></behaviordef
><behaviordef
name="readConnectedState"
><label
key="readConnectedState"
set="acnbase.lset"
></label
><refines
name="autoConnectedState"
set="acnbase.bset"
></refines
><section
><hd
>State of a Connection used for Reading</hd
><p
>This behavior shall follow all the rules of autoConnectedState except that the connection to be tracked shall only be assigned by read accesses to the track-target property, while write accesses have no effect. Thus it reflects the state of the connection last used to read to the track-target irrespective of intervening write accesses.</p
><p
>Note, that for DMP connections, read accesses include both get-property and subscribe-event actions.</p
></section
></behaviordef
><behaviordef
name="autoTrackedConnection"
><label
key="autoTrackedConnection"
set="acnbase.lset"
></label
><refines
name="transportConnection"
set="acnbase.bset"
></refines
><section
><hd
>Connection Tracked by an autoConnectedState Property</hd
><p
>This property is a transportConnection identifier which is automatically updated to the identity of the connection being tracked by its parent property which shall be an autoConnectedState property. By definition, it must not be directly writable.</p
></section
><section
><hd
>See also</hd
><p
>connectionReporter, autoConnectedState.</p
></section
></behaviordef
><behaviordef
name="trackTargetRef"
><label
key="trackTargetRef"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to a Track-Target Property</hd
><p
>This property is a propertyRef which points to a track-target property as defined in autoConnectedState behavior.</p
></section
><section
><hd
>See also</hd
><p
>autoConnectedState.</p
></section
></behaviordef
><!--Actions triggering state changes--><behaviordef
name="loadOnAction"
><label
key="loadOnAction"
set="acnbase.lset"
></label
><section
><hd
>Value Loaded on Execution of an Action</hd
><p
>This property holds a value which is loaded into its parent on the execution of some action. The action of loading the value is transient and the parent may be writable or changeable by other means.</p
><p
>The action(s) required to trigger the load are specified by one or more actionSpecifier properties which are logical children of this one.</p
><p
>If more than one actionSpecifier children ar present, then the load will take place whenever any one of the specified actions occurs.</p
><p
>If the value of a loadOnAction property changes as a result of the same action that loads it into its parent, then it is the value before the action which is loaded. This means a cascade of loadOnAction properties nested within each other will act like a shift register with values shifting up one place each time the action occurs.</p
></section
></behaviordef
><behaviordef
name="actionSpecifier"
><label
key="actionSpecifier"
set="acnbase.lset"
></label
><section
><hd
>Property Specifying an Action Condition</hd
><p
>An action is a momentary condition such as a state transition, property load, connection termination etc. Such conditions can trigger other changes such as property load events (see loadOnAction behavior). The vast majority of action conditions can and should be expressed as property value loads or changes. It is deprecated to create arbitrary behaviors describing action conditions. For example, an event triggered by the termination of a connection must be expressed as a value change in a connectionState property.</p
><p
>Actions are distinct from states which persist for some possibly short time and can be decribed using other static constructions in DDL such as selectors.</p
><p
>Refinements of actionSpecifier are provided to express common conditions such as property transitions and device state transitions.</p
></section
></behaviordef
><behaviordef
name="actionProperty"
><label
key="actionProperty"
set="acnbase.lset"
></label
><refines
name="actionSpecifier"
set="acnbase.bset"
></refines
><section
><hd
>Property on Which Actions Occur</hd
><p
>This is a part of the action specifier behaviors used to describe transient conditions. When a load, value change or other action on a property is to be specified, the property on which the action takes place is called the action property and can carry actionProperty behavior subject to the rules below. This may be in addition to any other behaviors it carries.</p
><p
>Any property which is the permanent actionProperty for a propertyAction specifier shall carry actionProperty behavior, whether or not that propertyAction is active (e.g. becaue it is subject to a selector). Where a propertyAction specifier references arbitrary action properties (e.g. via a variable propertyReference), then the action property can only be idenified by reference and need not carry actionProperty behavior.</p
></section
></behaviordef
><behaviordef
name="propertyActionSpecifier"
><label
key="propertyActionSpecifier"
set="acnbase.lset"
></label
><refines
name="actionSpecifier"
set="acnbase.bset"
></refines
><section
><hd
>Action on a Property</hd
><p
>Action specifiers can refer to conditions such as connection state changes which are not directly related to the state of a device as expressed in its property values. A propertyActionSpecifier is an action specifier which defines an action directly related to a device property (called the actionProperty (q.v.). The two principal property actions are value changes and value loads. Refinements of this behavior exist for both of these cases and may be defined for others.</p
><p
>A propertyActionSpecifier shall contain zero or more actionState sub-properties which express constraints on the value the actionProperty must have before and/or after the load or change for the action to occur. (for example - an action which only occurs when the actionProperty‘s value changes from 0 to 1). The action condition is met when any of its actionState conditions is met. It there are no actionState children at all, then the action is not dependent on the value of the actionProperty at all and the state conditions are always met. For example, a propertyChangeAction with no qualifying actionState sub-properties is defined to occur for ANY change of the actionProperty.</p
><p
>The property on which the action must occur (the actionProperty) shall be specified in one of the following ways – highest precedence first.</p
><section
><hd
>Direct Specification</hd
><p
>Since actionSpecifiers are behaviors which require no value themselves, it is allowable for the actionProperty itself to carry an additional propertyActionSpecifier behavior. For example if a property carries both actionProperty and propertyLoadAction behaviors, then any load of this property is considered to meet the action condition (subject to qualification by actionState children). This is useful when actionProperties exist purely to perform the action function.</p
></section
><section
><hd
>Specification by Reference</hd
><p
>If a propertyActionSpecifier carries propertyReference behavior, then the action property is the target of the reference. In most cases this will be a localDDLpropertyRef, but any propertyRef type is permitted.</p
><p
>It is not, permitted for the actionProperty to be in a different component from the actionSpecifier as that would imply a network action to monitor the actionProperty for the necessary conditions. If a remote property is to be used as an action trigger, it can easily be described using a local actionProperty with a binding to the remote property.</p
></section
><section
><hd
>Specification by Child</hd
><p
>Otherwise this property shall have no value is, but shall contain exactly one logical child property with actionProperty behavior which is the action property.</p
></section
></section
><section
><hd
>Property Action Specifier Examples</hd
><section
><p
>The top level property labeled “example 1” may be read and written normally, but is reset to zero each time the value of property with xml:id="TRIGGER_PROP" (defined elsewhere) changes.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;example 1&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="type.sint"/&gt;
  &lt;!-- define read/write property address --&gt;
  &lt;protocol&gt;...&lt;/protocol&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior set="acnbase.bset" name="loadOnAction"/&gt;
    &lt;value type="sint"&gt;0&lt;/value&gt;
    &lt;property valuetype="immediate"&gt;
      &lt;behavior set="acnbase.bset" name="propertyChangeAction"/&gt;
      &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
      &lt;value type="string"&gt;TRIGGER_PROP&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
></section
><section
><p
>This common example is a “calibrate” property: The target property always represents the “desired state” of the calibration state. However, this property is also an action property with propertyLoadAction behavior, and is associated with a loadOnAction value of 0. If the calibration state and target are both 1 (calibrated), then a load of 1 into the target property will first set calibration state to 0 (uncalibrated) but because the target is still 1, a re-calibration will occur – so re-calibration can be achieved simply by loading 1 into the target rather than having to set it to zero and then back to 1 again.</p
><p
xml:space="preserve"
>&lt;property valuetype="network"&gt;
  &lt;label&gt;Calibrated state&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="initializationBool"/&gt;
  &lt;behavior set="acnbase.bset" name="driven"/&gt;
  &lt;protocol&gt;...[read only access]...&lt;/protocol&gt;
  &lt;property valuetype="network" xml:id="calib-target"&gt;
    &lt;label&gt;Calibration target&lt;/label&gt;
    &lt;behavior set="acnbase.bset" name="target"/&gt;
    &lt;behavior set="acnbase.bset" name="actionProperty"/&gt;
    &lt;protocol&gt;..[write access to target]...&lt;/protocol&gt;
  &lt;/property&gt;
  &lt;property valuetype="immediate"&gt;
    &lt;behavior set="acnbase.bset" name="loadOnAction"/&gt;
    &lt;value type="uint"&gt;0&lt;/value&gt;
    &lt;property valuetype="immediate"/&gt;
      &lt;behavior set="acnbase.bset" name="propertyLoadAction"/&gt;
      &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
      &lt;value type="string"&gt;calib-target&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
></section
></section
><section
><hd
>See also</hd
><p
>propertyChangeAction, propertyLoadAction, actionState</p
></section
></behaviordef
><behaviordef
name="propertyLoadAction"
><label
key="propertyLoadAction"
set="acnbase.lset"
></label
><refines
name="propertyActionSpecifier"
set="acnbase.bset"
></refines
><section
><hd
>Action Condition Triggered by Property Load</hd
><p
>Specifies an action condition caused by loading a property. This different from a property change in several ways.</p
><p
>On a property load, the value after the load may be the same as it was before (beacuse the same value was re-loaded). This action can only occur therefore when a property has a clear load event such as a network write or a local operator action (e.g. pushing a button). It would not make sense for example to use a propertyLoadAction referencing a temperature sensor – that should use a propertyChangeAction.</p
><p
>As specified in propertyActionSpecifier behavior, the load action may be constrained to occur only for certain values of the property before or after the load.</p
></section
><section
><hd
>See also</hd
><p
>propertyChangeAction, actionState</p
></section
></behaviordef
><behaviordef
name="propertyChangeAction"
><label
key="propertyChangeAction"
set="acnbase.lset"
></label
><refines
name="propertyActionSpecifier"
set="acnbase.bset"
></refines
><section
><hd
>Action Condition Triggered by Property Value Change</hd
><p
>Specifies an action condition caused by a change of property value.</p
><p
>By definition this action requires a change in the property value. Actions which are triggered by a property load, even when the value does not change can be expressed with propertyLoadAction.</p
><p
>As specified in propertyActionSpecifier behavior, the change action may be constrained to occur only for certain values of the property before or after the load.</p
></section
><section
><hd
>See also</hd
><p
>propertyLoadAction, actionState</p
></section
></behaviordef
><behaviordef
name="actionState"
><label
key="actionState"
set="acnbase.lset"
></label
><refines
name="actionSpecifier"
set="acnbase.bset"
></refines
><section
><hd
>State Value Required to Meet an Action Condition</hd
><p
>There are two actionState refinements: actionStateBefore and actionStateAfter. Each expresses a value which the action property must have in order for the action condition to occur.</p
><section
><hd
>Nesting of actionState Properties</hd
><p
>When the action condition is a state change (i.e. within a propertyChangeAction) a top level actionState property may contain one or more actionState sub-properties of the opposite sense (before vs after) to express a particular transition. The sub-property shall not contain further actionState children.</p
><p
>When the action condition is a load action (see propertyLoadAction) then actionState properties shall not be nested.</p
></section
></section
></behaviordef
><behaviordef
name="actionStateBefore"
><label
key="actionStateBefore"
set="acnbase.lset"
></label
><refines
name="actionState"
set="acnbase.bset"
></refines
><section
><hd
>Required State Value Before an Action</hd
><p
>This is the value an actionProperty must have before the action event in order for the action condition to be met.</p
><p
>If this property is the child of a propertyChangeAction then it may have one or more actionStateAfter children which further restrict the states required to meet the action condition.</p
><p
>If this property is the child of a propertyActionSpecifier and has no actionStateAfter children, then the condition is independent of the state of the action property after the action event. i.e. any value will meet the condition.</p
></section
></behaviordef
><behaviordef
name="actionStateAfter"
><label
key="actionStateAfter"
set="acnbase.lset"
></label
><refines
name="actionState"
set="acnbase.bset"
></refines
><section
><hd
>Required State Value After an Action</hd
><p
>This is the value an actionProperty must have after the action event in order for the action condition to be met.</p
><p
>If this property is the child of a propertyChangeAction then it may have one or more actionStateBefore children which further restrict the states required to meet the action condition.</p
><p
>If this property is the child of a propertyActionSpecifier and has no actionStateBefore children, then the condition is independent of the state of the action property before the action event. i.e. any value will meet the condition.</p
></section
></behaviordef
><behaviordef
name="initializer"
><label
key="initializer"
set="acnbase.lset"
></label
><refines
name="loadOnAction"
set="acnbase.bset"
></refines
><section
><hd
>Initial Value</hd
><p
>This property holds the value which the parent property will take following some initialization process. This behavior or one or more of its refinements are intended to replace initialValue in the original dmpBase behaviorset.</p
><section
><hd
>Implicit Action Specifier</hd
><p
>Initializer behavior is intended as a compact indication of a property's default , power-on or “factory reset” value. Its actionSpecifier is implied and must not be provided separately. For more specific action conditions, a loadOnAction property and explicit actionSpecifier must be used.</p
><p
>The implied actionSpecifier is defined as follows:</p
><section
><hd
>Non-persistent Network Properties</hd
><p
>For a network accessible property which is not persistent, the initializer is loaded at power-on and on any reset operation which is equivalent to power-on such as full reset. In devices with multiple reset operations, such as warm and cold boots, or standby conditions, the definition of what level of reset is required to re-initialize the parent property is beyond the scope of this definition.</p
></section
><section
><hd
>Persistent Network Properties</hd
><p
>For a network accessible property which is declared persistent, the value is expected to survive power-cycles and resets. In this case, the initializer is the value which will be set when the product is first manufactured and will be loaded by any operation intended to return the device to its state when initially manufactured (e.g. a “Set Factory Defaults” operation).</p
></section
><section
><hd
>Other Property Types</hd
><p
>For other property types, an initializer has no useful meaning.</p
></section
></section
><section
><hd
>Notes</hd
><section
><p
>In many cases, if either a device or a controller has been off-line, there is no implicit way for the controller to know whether the device has been reset or not unless special properties are provided to supply that information.</p
></section
><section
><p
>A property which is also volatile, may have had another value provided by other means between reset and first read or write of the property.</p
></section
></section
></section
></behaviordef
><!--Power on and initialization--><behaviordef
name="initializationState"
><label
key="initializationState"
set="acnbase.lset"
></label
><refines
name="enumeration"
set="acnbase.bset"
></refines
><refines
name="ordered"
set="acnbase.bset"
></refines
><section
><hd
>Initialization State</hd
><p
>Some device functions take a significant time to initialize after power on or require an explicit signal from the controller before they will initialize. The controller may also wish to force them to re-initialize during operation. An initialization state property indicates or controls the initialization state of its parent property.</p
><p
>This property is an ordered enumeration with zero representing a fully uninitialized state and its upper limit value representing the fully initialized, normal operation condition. Intermediate states represent a progression of conditions between the two.</p
><p
>The upper limit of an initialization state shall be defined. This may be either using an explicit limit property indicating the maximum value, or by the implicit limit of the type representing the state (e.g. 1 or “true” for a boolean). Definition of the upper limit within a derived behavior is possible but discouraged, because applications which do not recognize that behavior will be unable to deduce the limit.</p
><p
>As with any enumeration, child properties such as labels or choices can further describe the possible states.</p
><p
>If an initialization process takes time then a driven initializationState should be driven by a target in the normal way.</p
><p
>Unless indicated explicitly by descriptions of individual states, a property which is not fully initialized (the parent of the initialization state property) must be treated as invalid. Writing values to it or its driver properties may be ignored and shall not have any direct effect except that they may be stored for use when the property becomes initialized. Attempts to read its value or the value of any properties derived from it may be rejected or return undefined results such as garbage values.</p
></section
></behaviordef
><behaviordef
name="initialization.enum"
><label
key="initialization.enum"
set="acnbase.lset"
></label
><refines
name="initializationState"
set="acnbase.bset"
></refines
><refines
name="type.enum"
set="acnbase.bset"
></refines
><section
><hd
>Enum Initialization State</hd
><p
>An initializationState property represented by type.enum encoding.</p
><p
>The upper limit of this enum (as specified by a sub-property with limit behavior, or implicitly by the enumerations or other typing behavior) represents the fully initialized state while a value of zero represents the most uninitialized state.</p
></section
></behaviordef
><behaviordef
name="initializationBool"
><label
key="initializationBool"
set="acnbase.lset"
></label
><refines
name="initializationState"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><section
><hd
>Boolean Initialization State</hd
><p
>An initializationState property represented by type.boolean encoding. A value of false implies uninitialized and true represents fully initialized.</p
></section
></behaviordef
><!--References into and from array properties--><behaviordef
name="refInArray"
><label
key="refInArray"
set="acnbase.lset"
></label
><refines
name="propertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference Within Array</hd
><p
>Behaviorset dmpBase defines reference behaviors but ambiguity can arise when the reference “points” from or to a property array.</p
><p
>The ambiguity is only possible for certain mechanisms where the reference may resolve to a whole array rather than an individual element within it. For example, a namedPropertyRef refers to a DDL property by its xml:id attribute which identifies the entire array rather than individual properties within it. A DMPpropertyAddress, on the other hand is a value which always refers to a single DMP property.</p
><p
>refInArray behavior is only applicable where the reference can resolve to an entire array. Declaring refInArray behavior in addition to any of the reference behaviors defined in dmpBase indicates that the property reference follows these rules:</p
></section
><section
><hd
>Terminology</hd
><p
>References are behaviors declared on a referencing property. Within these rules, this is called the “pointer” property. The pointer property points at some other property which is called the “target” property.</p
><p
>Either pointer or target property (or both) may be declared to be an array using the standard array mechanism of DDL, either directly or because it is part of a subtree rooted in an array property. This can give rise to a pointer array P[i] or target array T[i].</p
></section
><section
><hd
>Unambiguous Targets</hd
><p
>Whether or not, the pointer property is a member of an array, if the target(s) are a single properties, then there is no ambiguity. In the case of a pointer array of immediate values, the standard rules of DDL mean that either a single common value, or a list of individual values may be ascribed. For example with pointer array P[1..n], pointer P[i] points to target T, for all i.</p
><p
>Since this is the assumption of the reference behaviors in dmpBase, it is not necessary to declare refInArray behavior for these cases.</p
></section
><section
><hd
>References to Arrays</hd
><p
>It is legal to make a reference which resolves to an array of properties when the reference property is itarrayRef a matching array of the same size. In this case, each pointer shall be considered to point to the property at the same index position within the target array (P[i] points to T[i] for each i). This is possible for both single and multidimensional arrays (P[i, j] points to T[i, j]).</p
><section
><hd
>Arrays of Identical Size and Dimension</hd
><p
>In the absence of explicit indexing declaration as specified in the next section (when there are no rangeOver child properties to the refInArray), the reference array and target array shall have the same number and order of dimensions and shall be exactly the same size in each dimension and there shall be a one to one correspondence between pointer and target.</p
></section
><section
><hd
>Explicit Declaration of Indexing</hd
><p
>In the general case of multidimensional arrays (implemented in DDL as arrays containing arrays), it is often useful to have fewer dimensions in the target array than in the reference array. For example, a two dimensional array of many-to-one pointers into a single dimensional target array (P[i, j] points to target T[i] for each i and for any j). These cases are permitted, but require explicit specification of those dimensions (or subscripts) in the reference array which have an indexing correspondence with target array and those that are many-to one. Extending the preceding example, we need to specify whether P[i, j] points to T[i] or to T[j].</p
><p
>Specification of the dimensions used, relies on the fact that each property in DDL can only have a single array dimension, so multidimensional arrays require descendant properties to be nested within them. Each dimension of the array is therefore identified by the property which declares it, and this property can be referenced to specify the dimension.</p
><p
>An refInArray reference may contain one or more rangeOver properties which are themselves property references and identify the array dimension which the pointer array indexes with. Note that while the outer refInArray property references the target array, the rangeOver properties are pointers to ancestors of the pointer array.</p
><p
>The target array shall have exactly the same number of dimensions as the pointer array has rangeOver children.</p
><p
>For each rangeOver which the pointer specifies, the target array shall have a dimension of identical size.</p
><p
>The order of the rangeOver specifiers shall match the nesting level of the target array, with the first specifier matching the outermost target array. This allows pointers to reference target arrays whose dimension ordering differs from that of the pointer array.</p
></section
></section
><section
><hd
>Examples</hd
><section
><hd
>Single Dimensional Array</hd
><p
>This example implements a fixed binding (see binding behavior) between two arrays of properties, the target property is declared as an array with ID “TARGET_ID” and labelled "Masters".</p
><p
>There is a corresponding array of properties labelled “Slaves” which are bound one-to-one to the properties in the target array.</p
><p
>The references themselves form an array becauyse they are children of an array property.</p
><p
xml:space="preserve"
>&lt;property array="100" xml:id="TARGET_ID" valuetype="network"&gt;
  &lt;label&gt;masters&lt;/label&gt;
  ...
&lt;/property&gt;

&lt;property array="100" valuetype="implied"&gt;
  &lt;label&gt;Slaves&lt;/label&gt;
  &lt;behavior set="acnbase.bset" name="bindingSlave"/&gt;
  ...
  &lt;property valuetype="immediate"&gt;
    &lt;label&gt;Pointer property&lt;/label&gt;
    &lt;behavior set="acnbase.bset" name="bindingMasterRef"/&gt;
    &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
    &lt;behavior set="acnbase.bset" name="refInArray"/&gt;
    &lt;value type="string"&gt;TARGET_ID&lt;/value&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
></section
><section
><hd
>Multidimensional Array</hd
><p
>In this example, the target array of 100 masters is the same as previously. However, there are now five outer arrays each containing 100 slaves which reference the same master array. Each of the five outer arrays is bound to the same set of masters so each master drives 5 slaves. A rangeOver property defines that the references indexes over the inner array..</p
><p
xml:space="preserve"
>&lt;property array="100" xml:id="TARGET_ID" valuetype="network"&gt;
  &lt;label&gt;Masters&lt;/label&gt;
  ...
&lt;/property&gt;

&lt;property array="5" xml:id="OUTER"&gt;
  &lt;label&gt;Array of arrays&lt;/label&gt;
  ...
  &lt;property array="100" valuetype="implied" xml:id="INNER"&gt;
    &lt;label&gt;Slaves&lt;/label&gt;
    &lt;behavior set="acnbase.bset" name="bindingSlave"/&gt;
    ...
    &lt;property valuetype="immediate"&gt;
      &lt;label&gt;Pointer property&lt;/label&gt;
      &lt;behavior set="acnbase.bset" name="bindingMasterRef"/&gt;
      &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
      &lt;behavior set="acnbase.bset" name="refInArray"/&gt;
      &lt;value type="string"&gt;TARGET_ID&lt;/value&gt;
      &lt;property valuetype="immediate&gt;
        &lt;behavior set="acnbase.bset" name="rangeOver"/&gt;
        &lt;value type="string"&gt;INNER&lt;/value&gt;
      &lt;/property&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
><p
>Had the rangeOver property specified “OUTER” instead of “INNER” then it would be referencing a target array of 5 masters, each bound to an array of 100 slaves.</p
></section
><section
><hd
>Multidimensional Array – Illegal Usage</hd
><p
>Here the target array of 100 masters is the same. But now the pointers are an array of 5 arrays of 20. Despite the fact that there are the same total number of pointers as targets, this is NOT LEGAL because there is not a direct match dimension, by dimension between the arrays.</p
><p
xml:space="preserve"
>&lt;property array="100" xml:id="TARGET_ID" valuetype="network"&gt;
  &lt;label&gt;Masters&lt;/label&gt;
  ...
&lt;/property&gt;

&lt;property array="5"&gt;
  &lt;label&gt;Array of arrays&lt;/label&gt;
  ...
  &lt;property array="20" valuetype="implied"&gt;
    &lt;label&gt;Slaves&lt;/label&gt;
    &lt;behavior set="dmp2" name="bindingSlave"/&gt;
    ...
    &lt;property valuetype="immediate"&gt;
      &lt;label&gt;Pointer property&lt;/label&gt;
      &lt;behavior set="acnbase.bset" name="bindingMasterRef"/&gt;
      &lt;behavior set="acnbase.bset" name="localDDLpropertyRef"/&gt;
      &lt;behavior set="acnbase.bset" name="refInArray"/&gt;
      &lt;value type="string"&gt;TARGET_ID&lt;/value&gt;
    &lt;/property&gt;
  &lt;/property&gt;
&lt;/property&gt;</p
></section
></section
><section
><hd
>See also</hd
><p
>rangeOver</p
></section
></behaviordef
><behaviordef
name="rangeOver"
><label
key="rangeOver"
set="acnbase.lset"
></label
><refines
name="namedPropertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Array Reference Index Range Specifier</hd
><p
>This declares an ancestor of a reference property with which that reference indexes when used as a reference into a target array.</p
><p
>See refInArray behavior for full details of array references and examples.</p
><p
>The logical parent of a rangeOver property shall be a propertyRef with refInArray behavior (or refinement).</p
><p
>The target of the rangeOver reference shall be its ancestor. This is permitted to be in a parent device, subject to the scoping rules of the referencing mechanism.</p
><p
>The target of the rangeOver reference should be a property with an array declaration. If it is not, or if it is has an array size declared as 1, then the rangeOver has no effect.</p
></section
><section
><hd
>See also</hd
><p
>refInArray</p
></section
></behaviordef
><!--Properties whose value depends on the access context--><behaviordef
name="contextDependent"
><label
key="contextDependent"
set="acnbase.lset"
></label
><section
><hd
>Context Dependent Value</hd
><p
>The value of this property differs depending on the context from which it is accessed. Refinements of this behavior define what context difference affects the value and how.</p
><section
><hd
>Examples</hd
><p
>A property whose value differs depending on the controller which is accessing it.</p
><p
>A property whose value returns the source IP address from the IP datagram requesting the value (value is dependent on network context - including NAT etc).</p
></section
><p
>In general context dependent values should be avoided – a property value should be the same for all observers. However, some useful behaviors – particularly some relating to access control, or network and system management can be implemented as refinements of this one.</p
></section
></behaviordef
><behaviordef
name="controllerContextDependent"
><label
key="controllerContextDependent"
set="acnbase.lset"
></label
><refines
name="contextDependent"
set="acnbase.bset"
></refines
><section
><hd
>Controller Context Dependent Property</hd
><p
>The value of the property may appear different when read by different controllers. Writing of the property by different controllers may similarly have differing effects.</p
><p
>Note that the dependency is on the controller identity (CID in the case of DMP) but not on the connection used. If a refinement of this behavior allocates access to resources based on controller context, then care must be taken that it is possible to de-allocate those resources without the controller being present – it may not be available. And asking a user to power cycle a device before they can access it from their backup controller is probably not sufficient – a timeout would be better!</p
></section
></behaviordef
><behaviordef
name="connectionContextDependent"
><label
key="connectionContextDependent"
set="acnbase.lset"
></label
><refines
name="contextDependent"
set="acnbase.bset"
></refines
><section
><hd
>Connection Context Dependent</hd
><p
>The value of the property may appear different when read using different connections. In DMP a controller may establish multiple connections to a device and the device keeps track of the state of each connection. In other protocols which have the concept of a connection, the same may apply.</p
><p
>Note that the dependency is on the connection identity (CID and session number in the case of DMP) so where the protocol permits multiple connections from the same controller, each one establishes a different context.</p
></section
><section
><hd
>Note: Relationship to connectionDependent behavior in dmpBase Behaviorset</hd
><p
>This behavior replaces connectionDependent in dmpBase. Use of connectionContextDependent is preferred because its derivation from contextDependent relates its use to other context dependencies in a common way.</p
></section
></behaviordef
><!--Generic network interfaces--><behaviordef
name="netInterface"
><label
key="netInterface"
set="acnbase.lset"
></label
><refines
name="deviceSupervisory"
set="acnbase.bset"
></refines
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><section
><hd
>Network Interface</hd
><p
>This property represents a network interface – this can be either a physical interface or a logical interface such as an IP interface (which could be attached to one of several physical interfaces such as Ethernet, Wifi, Modem, etc.). This is a generalized supervisory group which serves two purposes:</p
><section
><p
>It is a repository for configuration parameters relating to the interface and provides a place where these may be exposed or configured within DMP (or other access protocol as defined by DDL);</p
></section
><section
><p
>It is a property representing the interface which may be referenced to indicate which interface other items relate to, for example in xenoPropRefs which together with xenoBindings can be built into bridge or gateway devices.</p
></section
><p
>The specific network to which these parameters apply may be a network underlying the access protocol (e.g. The Ethernet or IP network in a DMP/SDT/UDP system) or may be another network to which the device has a connection (e.g. in the case of a gateway or bridge device).</p
><p
>Network is used in the broadest sense and items within this group may apply to any connection or datalinks whether or not this is a not true networks in a stricter sense. For example, Serial or parallel ports, DMX512 connections, USB connections etc.</p
><p
>Each separate network connection, layer or configuration shall have its own netInterface. These shall be nested to indicate layering with higher layers (those nearer application layer), being contained within lower layers (those nearer the physical layer). The normal shared property mechanisms may be used and multiple alternatives may be present. Grouping using this behavior shall also be used to separate independent configurations at the same layer into separate groups. Thus a component exposing two IPv4 configurations (for example through multihoming) must expose these in two separate groups, so that the netmask from one is clearly associated with the network address and default gateway for the same configuration.</p
><p
>Where multiple groups for the same protocol or network type are present they are assumed to all be operational unless explicitly indicated by selectors, active status indicators or other mechanisms. Thus for example an Ethernet interface group which contains multiple IPv4 configuration groups indicates that multiple IP address configurations are active on the same interface.</p
></section
><section
><hd
>Multiple Devices or Components</hd
><p
>A Network Configuration is often applicable to an entire appliance rather than an individual device. In cases where there is a single component with just one root device, then there is no ambiguity provided that the root device contains just one case of any specific configuration parameter. netInterface properties should be declared as “high” (as close to the root) in the device tree as possible.</p
><p
>For cases, where there are multiple components within an appliance, the user must beware, that configuration changes applied to one component may affect the function of another, whether or not the other component declares the same configuration properties itself.</p
><p
>In the case of multiple components within an appliance, the preferred implementation is to create a separate supervisory component which is the single place where appliance-wide configurations are declared.</p
></section
><section
><hd
>Labelling Interfaces and Configurations</hd
><p
>In an appliance with multiple interfaces available, it is usually important to assign a meaningful label to each one since these are likely to be used by a user in configuring the system. Where, netInterfaces refer to physical interfaces the label should relate directly to the product marking for that physical connection. For example if a product has two Ethernet interfaces which are identified “Primary” and “Backup” on the case, then the same terms should be used in labelling the interfaces in the description rather than simply using “Eth0” and “Eth1” or no label at all.</p
></section
><section
><hd
>Applicability</hd
><p
>A netInterface may be present for any protocol or interface, however many protocols incorporate their own configuration information and messages and unless it it desired to expose this configuration at the device description level, no netInterface is required or expected. For example, the DMP protocol when operating on SDT and UDP is configured entirely using DMPs own messages and a DMP device would not normally expose a netInterface for DMP. It could expose it's Ethernet address or IP address within appropriate netInterfaces for information purposes if desired – this would enable controllers whose primary or only protocol was DMP to access those values.</p
><p
>For any protocol where DMP (or whatever other access protocol the DDL is written for) is to be used to configure or manage it in some way, it is necessary to create netInterface to contain those configuration items. It is also likely to be necessary to create a netInterface to represent the interface to which any proxy properties or other protocol bridge or gateway items relate.</p
></section
></behaviordef
><behaviordef
name="netInterfaceItem"
><label
key="netInterfaceItem"
set="acnbase.lset"
></label
><section
><hd
>Network Interface Item</hd
><p
>A generic behavior covering items which serve to describe or change the representation or configuration of a network or datalink interface.</p
><p
>Network interface items do not include values which typically vary dynamically from message to message within the network (e.g. destination address for outgoing messages), but apply to parameters of the network which apply to the implementation within the device (e.g. its own network address).</p
><p
>There may well be other rules which apply to network items and many network protocols provide their own configuration mechanisms which are to be preferred and may be mandatory.</p
><section
><hd
>Warning</hd
><p
>Where network configuration items are writable and the network or network layer concerned is one which underlies the access protocol, then the strong probability exists that changes to these properties may cause disruption of the connection used to change them. In general, it is the responsibility of the designer of the device to ensure that such disruption is avoided if possible, and handled cleanly where dispruption is unavoidable. This means ensuring that values which are set are legal and realistic, both individually and in the combination in which they occur before allowing disruption. It also means that disruption must follow the appropriate rules, for exaample by closing connections in an orderly manner before changing some underlying parameter.</p
></section
></section
></behaviordef
><behaviordef
name="netInterfaceRef"
><label
key="netInterfaceRef"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><refines
name="DDLpropertyRef"
set="acnbase.bset"
></refines
><section
><hd
>Reference to Network Configuration Group</hd
><p
>This property is a reference or pointer to a network configuration group. It may be used wherever it is necessary to relate property to a specific network interface or configuration, when this is not done by containment.</p
></section
></behaviordef
><behaviordef
name="accessNetInterface"
><label
key="accessNetInterface"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>Network Interface used for Device Access</hd
><p
>This group describes the configuration of a network interface which is or may be used to access the device as described by the Device Description in which it appears.</p
><p
>An interface which cannot be used to access the device, or whose interface to the device would require a different description, shall not be described using this behavior.</p
><p
>Whether, or not this is an interface through which the device is currently accessible, or through which the device is currently being accessed, depends on system configuration. Where this information is not explicitly available via properties and values (often hard to implement), it may be possible to infer it by examination of link layer headers, routing tables, ARP (address resolution protocol) or other network information.</p
></section
></behaviordef
><behaviordef
name="netInterfaceDirection"
><label
key="netInterfaceDirection"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><refines
name="type.boolean"
set="acnbase.bset"
></refines
><section
><hd
>Direction of an Interface</hd
><p
>While true networking interfaces are usually fully bidirectional, there are a large number of communications protocols which are either entirely unidirectional (for example DMX512, MIDI) or are higly asymmetrical with a master/slave or controller/device configuration (for example GPIB, USB, RDM). In these protocols, a netInterfaceDirection property indicates whether its parent netInterface property is an input or an output, or a master or slave.</p
><p
>In all cases direction is relative to the described device.</p
><p
>For simple cases, the value of a netInterfaceDirection will be fixed, however some devices may have interfaces which are configurable for one or other role. A netInterfaceDirection property should not be used to attempt dynamic direction switching as a part of a protocol (e.g. for dynamically reversing the line as a part of an inherently bidirectional protocol).</p
><p
>For unidirectional protocols, a value of 1 (true) indicates an output interface while a value of 0 (false) indicates an input interface.</p
><p
>For asymmetrical bidirectional protocols, a value of true indicates a master or controller interface, whilst a value of false indicates a slave or device.</p
><p
>If the identifiecation of the master and slave roles for a particular protocol is not clear, then the sense of a netInterfaceDirection property must be determined either in the refinement of netInterface for that protocol, or in a refinement of this behavior.</p
></section
></behaviordef
><behaviordef
name="netAddress"
><label
key="netAddress"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><section
><hd
>An Address or Host Identifier Used by a Network Protocol</hd
><p
>Any protocol which uses addressing for delivery of messages typically requires that its nodes (the name host is used from IP, but is used here to refer to any network node) are assigned an address. Some protocols also require that addresses of other hosts are configured for correct functioning (e.g. a router address). This behavior can be refined to describe any such address.</p
><p
>This behavior is typically refined in two ways. Firstly to specify the network protocol to which the address applies and so the format of the property; and secondly to specify the node to which the address property relates – for example is it the address of the host exposing it, of a default gateway, of a DHCP server etc. Any network address property exposed within a device must therefore carry behaviors which express both of these dimensions. Usually these will be two distinct behaviors on the same property.</p
></section
></behaviordef
><behaviordef
name="myNetAddress"
><label
key="myNetAddress"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>An Address of this Appliance</hd
><p
>This property is a network address which can be used in messages which should be received by this appliance. Typically this is the destination address of incoming messages and the source address of outgoing messages. The use of myNetAddress behavior serves to unambiguously distinguish the address of the node rather than other addresses which may be present in a configuration group. e.g. addresses of routers, nodes bound to this one etc.</p
><p
>Any appliance may have many network addresses. this is because it may have many physical interfaces, many different protocols and protocol layers and some protocols may allow multiple addresses at the same layer and physical interface.</p
><p
>This behavior should be used in addition to a network protocol specific behavior which defines the format of the property.</p
></section
><section
><hd
>Inactive Address or Configuration Group</hd
><p
>If a myNetAddress property has an invalid value, then it means that this address is inactive. If all myNetAddress properties within a configuration group are invalid, then the entire configuration shall be inactive.</p
><p
>Network address behaviors for individual specific network types should define a value of network address which is invalid and shall then be used for purposes of signifying an inactive address or configuration.</p
><p
>Where a protocol has no address value available which can be used to signify an unused address or configuration, than an explicit disable switch must be provided in order to report or set an inactive configuration.</p
></section
><section
><hd
>Warning. Re-configuration of Network Address</hd
><p
>Assigning network addresses or other configuration parameters, via the DDL access protocol (e.g. using DMP) may be convenient in devices which do not have other interfaces available for configuration. However, this can give rise to problems.</p
><section
><hd
>Disruption of Connections</hd
><p
>Changing a network address in many cases will mean that network connections cease to work and need to be re-established. See general warning in netInterfaceItem.</p
><p
>By setting incorrect or inappropriate values, it is easy to make a device un-reachable, such that it requires manual intervention or even disassembly to restore communications.</p
></section
><section
><hd
>IPv4 and ACN EPI13 Violation of Protocol Rules</hd
><p
>Use of a static IP address as the only available address on an interface is forbidden by ACN epi13. Similar rules apply to address assignments for other protocols. Therefore if a device is to comply with EPI13, the only way that it can support assignment of IP addresses by this method is for the DMP assigned (static) address to coexist with the EPI13 assigned address – the ability to support multiple concurrent addresses on the same physical interface is often called multi-homing.</p
></section
></section
></behaviordef
><behaviordef
name="routerAddress"
><label
key="routerAddress"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>The Address of a Network Router</hd
><p
>This property represents the address of a router to which the appliance may send messages for forwarding to other networks.</p
><p
>This behavior may also be used for gateways or routers which translate messages into other protocols and forward them on networks of different types.</p
></section
></behaviordef
><behaviordef
name="serviceAddress"
><label
key="serviceAddress"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>The Address of a Network Service</hd
><p
>This property is the network address of some sort of network service which is or may be used by the appliance. Examples include DHCP server, Printing service, SLP Directory Agent, RDM Master Controller.</p
><p
>Note that for most higher level services, the most appropriate way to specify a service is frequently through the use of a URL. See dmpBase behaviors.</p
></section
></behaviordef
><behaviordef
name="netInterfaceState"
><label
key="netInterfaceState"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><refines
name="initializationState"
set="acnbase.bset"
></refines
><section
><hd
>Network Interface Initialization State</hd
><p
>This is a refinement of initialization state applied to a network interface and represents the operational state of the interface.</p
><p
>Many network interfaces may take some time before they are operational, during which they are acquiring addresses, discovering their network context and so on. The simplest netInterfaceState is a boolean with 1 (true) representing “up”, and 0 (false representing “down”).</p
><p
>As with other properties, a netInterfaceState may be a driven value controlled by a target value which can then be used to explicitly take the interface up (if this is possible) or down.</p
></section
></behaviordef
><!--IEEE802.3 and Ethernet interfaces--><behaviordef
name="netInterfaceIEEE802.3"
><label
key="netInterfaceIEEE802.3"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>IEEE802.3 (Ethernet) Network Interface</hd
><p
>This group describes the configuration of an IEEE802.3 (commonly called “Ethernet”) network interface.</p
></section
></behaviordef
><behaviordef
name="netInterfaceIEEE802.11"
><label
key="netInterfaceIEEE802.11"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>IEEE802.11 (Wireless Ethernet or WiFi) Network Interface</hd
><p
>This group describes the configuration of an IEEE802.11 wireless network interface.</p
><p
>Note that WiFi is a brand originally detailing a subset of IEEE802.11 but applies to most devices in practical use.</p
></section
></behaviordef
><behaviordef
name="netAddressIEEE-EUI"
><label
key="netAddressIEEE-EUI"
set="acnbase.lset"
></label
><refines
name="type.fixBinob"
set="acnbase.bset"
></refines
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>IEEE Extended Unique Identifier</hd
><p
>This property is a Extended Unique Identifier as defined by the Institute of Electrical and Electronics Engineers (IEEE). This is the network identifier used in a number of media access layer protocols including Ethernet, Firewire, Token-ring, Bluetooth etc.</p
><p
>Two sizes of EUI exist: EUI-48 and EUI-64. These may be distinguished by their declared property size, as well as by the protocol to which they are applied.</p
><p
>EUIs may be universally administered or locally assigned (refer to IEEE for details). An EUI will not normally be writable although re-configuration within certain restrictions may be allowed.</p
><p
>EUIs shall have a size of 6 octets (EUI-48) or 8 octets (EUI-64). Transmission order of octets in DMP (or other access protocol) shall be the same as that defined by the IEEE.</p
></section
><section
><hd
>See also</hd
><p
>IEEE_OUI</p
></section
></behaviordef
><!--Internet Protocol--><behaviordef
name="netIfaceIPv4"
><label
key="netIfaceIPv4"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>Internet Protocol version 4 (IPv4) Configuration</hd
><p
>This group describes an IPv4 configuration.</p
><p
>This group should appear within another netInterface which represents the interface to which this IP configuration is bound.</p
><p
>It is permissible to have multiple netIfaceIPv4 groups within a single interface group, each representing a different configuration. As noted in netAddressIPv4 and myNetAddress behaviors, if the myNetAddress property within a netIfaceIPv4 group is 0 then this configuration is inactive.</p
></section
><section
><hd
>Note on EPI-13, DHCP an IPv4LL addressing</hd
><p
>ACN EPI-13 provides a set of rules for automatic assignment of addresses which ensures that inaccessible devices should not get into a state where they must be re-configured locally in order to gain access to them. A netIfaceIPv4 group may be declared for either IPv4LL addressing or for DHCP addressing. These configurations within these groups must by their nature be read-only, but serve to expose those configurations at the device level if required.</p
><p
>For devices only capable of supporting a single IP address on an interface, a single netIfaceIPv4 group can describe the current configuration. However, the recommended implementation is to allow IPv4LL and DHCP configurations to exist concurrently (at least during transitions) and in this case, two separate netIfaceIPv4 groups must be declared.</p
><p
>By declaring a myNetAddress group with writable, persistent values for the relevant properties, a static IP configuration may be declared which is configurable by device write operations. This MUST be an additional configuration to the DHCP/IPv4LL configuration(s) to conform with EPI-13.</p
><p
>When writing configuration properties, care must be taken to ensure that the new configuration is updated as a whole. See the note in myNetAddress behavior. Simultaneous re-configuration of multiple properties can be accomplished by using atomicLoad behaviors.</p
></section
></behaviordef
><behaviordef
name="netAddressIPv4"
><label
key="netAddressIPv4"
set="acnbase.lset"
></label
><refines
name="type.fixBinob"
set="acnbase.bset"
></refines
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>IPv4 network Address</hd
><p
>This property is a network address for Internet Protocol version 4. It shall have a size of 4 octets and shall be transmitted in network byte order – the same order in which addresses appear within IPv4 headers.</p
><p
>A value of 0 is an invalid address and signifies that this property is inactive, unavailable or not configured. For example a property with both myNetAddress and netAddressIPv4 behaviors whose value is 0, implies that this entire configuration is inactive, while a defaultRouteAddress property with a value of 0 implies that no default router is configured and (except where other explicit routes are provided), traffic will be confined to the local network.</p
></section
></behaviordef
><behaviordef
name="netAddressIPv6"
><label
key="netAddressIPv6"
set="acnbase.lset"
></label
><refines
name="type.fixBinob"
set="acnbase.bset"
></refines
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>IPv6 network Address</hd
><p
>This property is a network address for Internet Protocol version 6. It shall have a size of 16 octets and shall be transmitted in network byte order – the same order in which addresses appear within IPv6 headers.</p
></section
></behaviordef
><behaviordef
name="netMask"
><label
key="netMask"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><section
><hd
>Network Mask</hd
><p
>The concept of network masking is extensively used in IP version 4 and in IPv6. This behavior may also be refined for any other protocol which uses a similar concept to separate an address into a network part and an address on the network part.</p
><p
>In IP networks, a netMask property must appear in the same netInterface as the address to which it applies.</p
></section
></behaviordef
><behaviordef
name="netMaskIPv4"
><label
key="netMaskIPv4"
set="acnbase.lset"
></label
><refines
name="netMask"
set="acnbase.bset"
></refines
><refines
name="type.uint"
set="acnbase.bset"
></refines
><section
><hd
>Network Mask for IPv4 Networks</hd
><p
>This property contains the network mask for an IPv4 network.</p
><p
>The format shall be an integer in the range 0..32. Its size shall be 1 octet. its value shall be the number of bits starting at the most significant bit, of an associated IPv4 address which constitute the network address (or subnet in some termionology). When IPv4 addresses are written using the common terminology for example 169.254.213.94/16, this is the number following the “/” (16 in this case).</p
><p
>The use of a bitcount instead of the other commonly written bitmask format (e.g. 255.255.0.0) is chosen because it is inherently more robust because masks with “holes” (which are not permitted) cannot be accidentally generated. It is also more space efficient.</p
></section
></behaviordef
><behaviordef
name="netNetworkAddress"
><label
key="netNetworkAddress"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>Address of a Network (Subnet Address)</hd
><p
>This is the address of a network (or subnet).</p
><section
><hd
>Use in Networks Using IPv4 and IPv6 Network and Address Semantics</hd
><p
>In IP networks the property follows the same format as a host address, but only the most significant N bits are significant where N is the netmask bitcount. Transmitters shall transmit bits which are not significant as 0, whilst receivers shall ignore them.</p
><section
><hd
>Determination of Netmask</hd
><p
>The size of the netmask is determined as follows:</p
><section
><p
>If this property has a logical child property with netMask behavior, then this specifies the netmask.</p
></section
><section
><p
>Otherwise, if this property is a logical child of a netInterface property which itself includes a netMask property which applies to the entire interface, then this specifies the netmask.</p
></section
><section
><p
>Otherwise the description is in error.</p
></section
></section
></section
><section
><hd
>Use and Structure</hd
><p
>netNetworkAddress and netHostAddress properties may be defined as drivers for a driven parent property with netAddress behavior. In this case, the parent property shall be constructed by combining the two parts according to the rules of the relevant protocol.</p
><p
>Where the format or protocol of a netNetworkAddress is not explicitly provided by behaviors attached to it, it shall conform to the format and protocol of the parent it is driving.</p
><p
>In other contexts where a network address is required (e.g. in routing tables) the structure must be determined by the associated behaviors.</p
></section
><section
><hd
>Note</hd
><p
>With modern classless IP addressing there is no real distinction between a network and a subnet. For other networks or cases where the distinction is made, thie behavior must be refined accordingly.</p
></section
></section
></behaviordef
><behaviordef
name="netHostAddress"
><label
key="netHostAddress"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>Host Part of a Network Address</hd
><p
>This is the host part of a network address.</p
><section
><hd
>Use in Networks Using IPv4 and IPv6 Network and Address Semantics</hd
><p
>In IP networks the property follows the same format as a full IP address, but those bits which constitute the Network address as determined by the netmask, are not significant. Transmitters shall transmit bits which are not significant as 0, whilst receivers shall ignore them.</p
><section
><hd
>Determination of Netmask</hd
><p
>The size of the netmask is determined as follows:</p
><section
><p
>If this property has a logical child property with netMask behavior, then this specifies the netmask.</p
></section
><section
><p
>Otherwise, if this property is a logical child of a netInterface property which itself includes a netMask property which applies to the entire interface, then this specifies the netmask.</p
></section
><section
><p
>Otherwise the description is in error.</p
></section
></section
></section
><section
><hd
>Use and Structure</hd
><p
>netNetworkAddress and netHostAddress properties may be defined as drivers for a driven parent property with netAddress behavior. In this case, the parent property shall be constructed by combining the two parts according to the rules of the relevant protocol.</p
><p
>Where the format or protocol of a netHostAddress is not explicitly provided by behaviors attached to it, it shall conform to the format and protocol of the parent it is driving.</p
><p
>In other contexts where a host-part address is required the structure must be determined by the associated behaviors.</p
></section
><section
><hd
>Note</hd
><p
>With modern classless IP addressing there is no real distinction between a network and a subnet. For other networks or cases where the distinction is made, thie behavior must be refined accordingly.</p
></section
></section
></behaviordef
><behaviordef
name="myAddressDHCP"
><label
key="myAddressDHCP"
set="acnbase.lset"
></label
><refines
name="myNetAddress"
set="acnbase.bset"
></refines
><refines
name="volatile"
set="acnbase.bset"
></refines
><section
><hd
>My Address as Assigned by DHCP</hd
><p
>This property is a network address which was assigned by Dynamic Host Configuration Protocol. A property with this behavior cannot be written. It may change.</p
></section
></behaviordef
><behaviordef
name="myAddressLinkLocal"
><label
key="myAddressLinkLocal"
set="acnbase.lset"
></label
><refines
name="myNetAddress"
set="acnbase.bset"
></refines
><refines
name="volatile"
set="acnbase.bset"
></refines
><section
><hd
>My Link-local Address as Assigned by Zeroconf</hd
><p
>This property is a network address which was assigned by the IPv4LL Link-local addressing specification [IETF RFC]. A property with this behavior cannot be written. It may change.</p
><p
>Note that the IPv4LL specification explicitly defines the netmask as 16-bits and forbids use of a router. Therefore, no netmask or defaultroute properties need be associated with this one. If they are, they must take the values 16-bits and 0.0.0.0 respectively.</p
></section
></behaviordef
><behaviordef
name="myAddressStatic"
><label
key="myAddressStatic"
set="acnbase.lset"
></label
><refines
name="myNetAddress"
set="acnbase.bset"
></refines
><section
><hd
>My Address Statically Assigned</hd
><p
>This property is a network address which was statically assigned.</p
></section
></behaviordef
><behaviordef
name="defaultRouteAddress"
><label
key="defaultRouteAddress"
set="acnbase.lset"
></label
><refines
name="routerAddress"
set="acnbase.bset"
></refines
><section
><hd
>Address of the Default Route</hd
><p
>The concept of a default route is used in IP networks. This property represents the address of a default router. It can be used for IPv4, IPv6 and any other protocol which uses the concept.</p
></section
></behaviordef
><behaviordef
name="DHCPserviceAddress"
><label
key="DHCPserviceAddress"
set="acnbase.lset"
></label
><refines
name="serviceAddress"
set="acnbase.bset"
></refines
><section
><hd
>The Address of a DHCP Server</hd
><p
>This property is the network address of a DHCP server.</p
><p
>Unless otherwise specified by refinement, if this appears within a netInterface representing the configuration of an interface of the device, it is the address of the DHCP server which supplied the configuration described by that group. If the group is inactive bcause no DHCP server has been found, or because some alternate configuration mechanism is in use, then this property shall carry the specified invalid value for the protocol it uses (e.g. 0 for IPv4).</p
></section
></behaviordef
><behaviordef
name="DHCPLeaseTime"
><label
key="DHCPLeaseTime"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><section
><hd
>DHCP Lease Time</hd
><p
>This property represents the time for which a DHCP lease was granted.</p
></section
></behaviordef
><behaviordef
name="DHCPLeaseRemaining"
><label
key="DHCPLeaseRemaining"
set="acnbase.lset"
></label
><refines
name="netInterfaceItem"
set="acnbase.bset"
></refines
><section
><hd
>DHCP Lease Remaining</hd
><p
>This property represents the time remaining on a DHCP lease.</p
></section
></behaviordef
><behaviordef
name="DHCPclientState"
><label
key="DHCPclientState"
set="acnbase.lset"
></label
><refines
name="netInterfaceState"
set="acnbase.bset"
></refines
><refines
name="initialization.enum"
set="acnbase.bset"
></refines
><section
><hd
>DHCP Client State</hd
><p
>This property represents the states of a DHCP client as defined in the DHCP specification [IETF RFC2131]. The relationship of property value to state is as follows:</p
><p
xml:space="preserve"
>Value State
  0   INIT
  1   INIT-REBOOT
  2   SELECTING
  3   REBOOTING
  4   REQUESTING
  5   REBINDING
  6   RENEWING
  7   BOUND</p
><p
>A DHCPclientState property should declare a maximum (of 7) for the benefit of controllers which understand the generic underlying enum behaviors but not this specific refinement.</p
><p
>Note: A DHCP client may be controlled by adding a target as a child of the DHCPclientState property. In this case a target of INIT will disable the DHCP interface, a target of BOUND enables the DHCP client and will enable the interface if a DHCP address is successfully obtained. To force renewal of the lease the DHCPclientState value can be set to INIT-REBOOT. Setting the target to other values could conflict with the DHCP state machine and should be ignored by the device.</p
></section
></behaviordef
><!--DMX512A and E1.31 interfaces--><behaviordef
name="netInterfaceDMX512"
><label
key="netInterfaceDMX512"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 Interface Configuration Group</hd
><p
>This group describes a DMX512 or DMX512-A protocol configuration, corresponding to a single DMX512 interface or a logical interface in the case that DMX512 data is carried within another transport.</p
><p
>DMX512 is a unidirectional protocol with a very clear distinction between controllers which transmit DMX512, and devices which receive it. A DDL description may apply to either type of appliance and therefore a netInterfaceDirection property shall be present as a logical child of this property to indicate which role the group represents.</p
></section
></behaviordef
><behaviordef
name="universeIdDMX512"
><label
key="universeIdDMX512"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 Universe Identifier</hd
><p
>This property holds a universe identifier which may be used in configuring a receiver for the DMX512 or DMX512-A protocol.</p
><p
>Physical DMX512 interfaces correspond to a single universe of DMX512 and so the netInterface and universeIdDMX512 must be the same property, but in interfaces capable of carrying multiple universes (e.g. ESTA ANSI E1.31), this property specifies which universe, is received or transmitted.</p
><p
>Refinements of this behavior specify how its value relates to to universes within the system.</p
><p
>Note that there is no restriction on the number of universeIdDMX512 properties within a device, because devices may be able to receive or transmit multiple universes.</p
><p
>ACN epi26 specifies a method and syntax for describing access to DDL properties using DMX-512 and this behavior is referenced by that document. Note though, that this behavior is not tied exclusively to ACN epi26 and may be used to specify a DMX universe in any description.</p
></section
></behaviordef
><behaviordef
name="netInterfaceDMX512pair"
><label
key="netInterfaceDMX512pair"
set="acnbase.lset"
></label
><refines
name="netInterfaceDMX512"
set="acnbase.bset"
></refines
><refines
name="universeIdDMX512"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 Physical Layer Interface</hd
><p
>This group represents the configuration for a physical DMX512 interface within an appliance. That is a single DMX data pair using RS485 signalling. This may be an input, or an output as determined using a netInterfaceDirection property (see netInterfaceDMX512).</p
><p
>Note that the selection of Male or Female connectors to determine the data direction is unreliable with many modern devices where DMX512 ports may be configured for either input or output.</p
><p
>This interface is independent of other pairs within the apparatus. Where there are multiple independent DMX512 data pairs within a single connector (e.g. the two pairs within an XLR-5), multiple configuration groups with this behavior can be used.</p
><p
>This behaviors refinement of universeIdDMX512 means that this property also serves to identify a universe of DMX512 since each DMX512 pair corresponds by definition to a single universe of data.</p
></section
></behaviordef
><behaviordef
name="netDMX512-XLRpri"
><label
key="netDMX512-XLRpri"
set="acnbase.lset"
></label
><refines
name="netInterfaceDMX512pair"
set="acnbase.bset"
></refines
><section
><hd
>Primary Pair of a DMX512 XLR Connection</hd
><p
>This is a configuration group representing the primary (in many case the only) pair of a physical DMX512 connection using a standard connector.</p
><p
>While the use of XLR-3 or other connectors is forbidden by the DMX512 standard, this behavior may be applied to such other connectors where they occur. It shall not apply to secondary or subsequent datalinks, or to datalinks which use different siignalling technology such as Ethernet or wireless.</p
></section
></behaviordef
><behaviordef
name="netDMX512-XLRsec"
><label
key="netDMX512-XLRsec"
set="acnbase.lset"
></label
><refines
name="netInterfaceDMX512pair"
set="acnbase.bset"
></refines
><section
><hd
>Secondary Pair of a DMX512 XLR Connection</hd
><p
>This is a configuration group representing the secondary pair of a physical DMX512 connection using a standard connector.</p
><p
>While the use of other connectors is forbidden by the DMX512 standard, this behavior may be applied to such other connectors where they occur. It shall not apply to any except the secondary datalink in a connector carrying multiple datalinks, or to datalinks which use different siignalling technology such as Ethernet or wireless.</p
></section
></behaviordef
><behaviordef
name="netIfaceE1.31"
><label
key="netIfaceE1.31"
set="acnbase.lset"
></label
><refines
name="netInterface"
set="acnbase.bset"
></refines
><section
><hd
>E1.31 Configuration</hd
><p
>This group describes an ESTA E1.31 protocol interface.</p
><p
>This group should appear within a netIfaceIPv4group which represents the IP configuration to which this E1.31 configuration is bound.</p
><p
>It is permissible to have multiple netIfaceE1.31 groups within a single netIfaceIPv4group, each representing a different E1.31 stream. As noted in netAddressIPv4 and myNetAddress behaviors, if the myNetAddress property within a netIfaceIPv4 group is 0 then this configuration is inactive.</p
></section
></behaviordef
><behaviordef
name="universeIdE1.31"
><label
key="universeIdE1.31"
set="acnbase.lset"
></label
><refines
name="universeIdDMX512"
set="acnbase.bset"
></refines
><refines
name="type.enum"
set="acnbase.bset"
></refines
><section
><hd
>E1.31 Protocol Universe Identifier</hd
><p
>This property specifies a DMX512 universe as transported by ESTA E1.31.</p
><p
>The value of this property shall be a 16-bit unsigned integer which must match the universe number as defined by the E1.31 specification.</p
><p
>The E1.31 protocol is a means for transporting DMX512 data within the ACN Architecture. Because it is specified to be DMX512 data, all descriptions can follow the extensions and behaviors for DMX512. However, E1.31 is capable of carrying multiple universes over a single interface, and therefore a refinement of the generic universeIdDMX512 is required to fit the specific specification method of E1.31.</p
><section
><hd
>Connected Component Indication</hd
><p
>When an E1.31 device is also accessible using a bidirectional or reporting protocol such as DMP, a universeIdE1.31 property should include a componentReference property which indicates the source component of the currently connected stream matching the configured universe.</p
></section
></section
></behaviordef
><behaviordef
name="slotAddressDMX512"
><label
key="slotAddressDMX512"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><refines
name="type.uint"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 Slot Address</hd
><p
>This property holds a slot address which refers to a DMX512 slot.</p
><section
><hd
>Property Value</hd
><p
>Because this property type or its refinements may be used in association with an offset, it is not possible to place limits on its range within this behavior specification. However, the result of any combination of slotAddressDMX512 with an offset must be in the range 1 - 512. If any value outside this range is used, the result is not only unspecified, and may affect the operation of other properties.</p
><p
>Limitations on the value of slotAddressDMX512 properties should be described using limit behaviors in the normal way.</p
></section
><section
><hd
>Universe Specification</hd
><p
>In many applications, a slotAddressDMX512 property must be unambiguously associated with a DMX512 universe. This may be done by containment or by reference.</p
><p
>If the baseAddressDMX512 is the logical child of a universeIdDMX512 property then that property specifies the universe.</p
><p
>If the baseAddressDMX512 is the logical parent of a propertyRef property which points to a universeIdDMX512 property then that property specifies the universe.</p
><p
>Whichever method is used, it is required that any baseAddressDMX512 is unambiguously associated with a single universe at any time.</p
><p
>The method chosen will depend on the nature of the application and the constraints which apply to independent selection of universe and base address.</p
></section
></section
></behaviordef
><behaviordef
name="baseAddressDMX512"
><label
key="baseAddressDMX512"
set="acnbase.lset"
></label
><refines
name="slotAddressDMX512"
set="acnbase.bset"
></refines
><refines
name="myNetAddress"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 Base Address</hd
><p
>This property holds a base address which is used in configuring a receiver for the DMX512 or DMX512-A protocol.</p
><p
>In most ordinary devices which use some fixed number of DMX512 slots, this is the first address in that range, often called the “start address”.</p
><section
><hd
>Universe Specification</hd
><p
>Each baseAddressDMX512 property shall be unambiguously associated with a DMX512 universe using the methods specified in slotAddressDMX512 behavior (which this refines).</p
></section
></section
><section
><hd
>Relationship to ACN EPI-26</hd
><p
>ACN epi26 specifies a method and syntax for describing access to DDL properties using DMX-512 and this behavior is referenced by that document. When used in that context, this property must carry an xml:id attribute. Note though, that this behavior is not tied exclusively to ACN epi26 and may be used to specify a DMX base address in any description.</p
></section
></behaviordef
><behaviordef
name="STARTCode"
><label
key="STARTCode"
set="acnbase.lset"
></label
><refines
name="netAddress"
set="acnbase.bset"
></refines
><section
><hd
>DMX512 START Code</hd
><p
>This property contains a DMX512 START Code. It shall be 1 octet in size. Its use depends on the context in which it occurs.</p
><p
>Because alternate START Codes access different functionality in devices, they are considered to be a part of the addressing information. However, some START Codes do not to relate to device functionality at all but represent system information or additional protocol functionality.</p
></section
></behaviordef
><behaviordef
name="DMXpropRef"
><label
key="DMXpropRef"
set="acnbase.lset"
></label
><refines
name="xenoPropRef"
set="acnbase.bset"
></refines
><section
><hd
>DMX Property Reference</hd
><p
>This property is a reference to a DMX512 “property” within a remote device which is accessed using DMX512 Data from the device being described.</p
><p
>This behavior is simply a reference, however the property may also carry a xenoBinder behavior (e.g. DMXbinding) which makes it into a binding to a remote DMX512 property.</p
><p
>The device containing the reference and which is the subject of the description is referred to as the local device. This device has one or more outgoing DMX512 data connections which can control remote devices.</p
><p
>The outgoing DMX512 data may be a physical DMX512 RS485 connection or may be a connection which transports a DMX512 Data stream by other methods, for example an outgoing E1.31 connection.</p
><section
><hd
>Value and Child Properties</hd
><p
>The value of a DMXpropRef shall be the “address” of the property.</p
><p
>Refinements of this behavior must specify specific address mechanisms and property sizes.</p
><p
>If the reference is a member of an array, its value represents the address of the first element which may be subject to incrementing or offsetting for subsequent elements.</p
><p
>Where multiple output streams (interfaces or universes) are available, the particular stream must be identified.</p
><p
>The following child properties may be present to qualify the reference:</p
><section
><hd
>universeIdDMX512</hd
><p
>If the output interface is capable of supporting multiple universes of DMX512 data, then this property specifies which is to be used.</p
></section
><section
><hd
>arrayInc</hd
><p
>If the reference is part of an array, this specifies the increment to add to the value of the reference for each iteration of the array.</p
></section
></section
><section
><hd
>START Code</hd
><p
>The use of START Codes in DMX512 is very variable and while many use similar addressing mechanisms, few use exactly the same mechanism. Therefore, to properly represent addressing for each START Code, a refinement of this behavior is required. The START Code is therefore implicit in the reference rather than explicitly specified.</p
></section
></section
><section
><hd
>See also</hd
><p
>DMX512 Specification: ANSI E1.11-2004</p
></section
></behaviordef
><behaviordef
name="DMXpropRef-SC0"
><label
key="DMXpropRef-SC0"
set="acnbase.lset"
></label
><refines
name="DMXpropRef"
set="acnbase.bset"
></refines
><refines
name="slotAddressDMX512"
set="acnbase.bset"
></refines
><section
><hd
>DMX Property Reference for NULL START Code type data</hd
><p
>This property is a reference to a DMX512 “property” in a remote device which is accessed either using START Code 0 (NULL START Code) or using a similarly mapped Alternate START Code.</p
><p
>For this behavior, an outgoing DMX512 universe is considered to address a single remote device with 512 properties of one octet each. Each referenced property therefore corresponds to one DMX512 Data Slot.</p
><p
>Treatment as a single device is used because there is no way intrinsic to DMX512 to know which receiver or receivers are receiving the data stream, or which remote device a specific slot might be used by.</p
><section
><hd
>Address Value</hd
><p
>The address is defined to be offset of the referenced data slot in the DMX512 data packet. The offset shall start at 1 and shall be expressed in slots. The START Code itself (slot zero) shall not be referenced using this behavior as it is considered to be part of the address rather than a property.</p
><p
>If the reference is a member of an array, its value represents the offset of the first element and may be subject to incrementing for subsequent elements. Any arrayInc property present shall indicate the increment in slots from one array iteration to the next.</p
></section
><section
><hd
>STARTCode</hd
><p
>If a START Code child property is present, it defines the START Code to be used to access the property. If no START Code is specified then the reference is to NULL START Code by default. With this behavior, the setting of an alternate START Code does not change the address mechanism or meaning in any way. This use should therefore only be applied to START Codes which use the same addressing mechanism (e.g. START Code 0xDD).</p
></section
></section
><section
><hd
>See also</hd
><p
>DMX512 Specification: ANSI E1.11-2004</p
></section
></behaviordef
><behaviordef
name="bindingDMXnull"
><label
key="bindingDMXnull"
set="acnbase.lset"
></label
><refines
name="xenoBinder"
set="acnbase.bset"
></refines
><section
><hd
>Binding to a Remote DMX512 Property Using NULL START Code</hd
><p
>This behavior describes a standard way to bind a local property within a device to a NULL START Code slot in DMX512.</p
><section
><hd
>Synchronization and Delivery</hd
><p
>DMX512 provides no guarantee of refresh rate or frequency except within very wide limits. It also does not provide guaranteed delivery and in certain configurations there are almost certain to be lost packets. It does however provide repetitive transmission so provided a slot maintains a value for a number of repetitions there is a very good chance that that value will be received.</p
><p
>The only requirement for synchronization imposed by this behavior is that where multiple local properties in the DDL device are changed by a single operation, the values of any DMX512 slots which are bound to those properties will all change in the same DMX512 packet.</p
><p
>In the case of DMP, properties are considered to have changed in a single operation if they are all changed by the action of a single PDU as received from the transport layer (SDT in most cases).</p
><p
>This rule prevents updates which affect multiple slots from being split across multiple packets which can cause unintended transient effects or jitter.</p
><p
>It should be noted that DMP and other protocols are quite capable of changing property values far faster than the update rate provided by DMX512 and that there is no protection against this in this behavior, it is therefore up to the controller accessing the local end of this binding to ensure that changed values are left long enough to propagate to remote DMX devices before they are overwritten.</p
></section
></section
></behaviordef
><behaviordef
name="bindingDMXalt-refresh"
><label
key="bindingDMXalt-refresh"
set="acnbase.lset"
></label
><refines
name="xenoBinder"
set="acnbase.bset"
></refines
><section
><hd
>Binding to a Remote DMX512 Property Using Alternate START Code</hd
><p
>This behavior describes a standard way to bind a local property within a device to a property accessed in a DMX512 device using an Alternate START Code, but where the Alternate START code data is expected to be repetitively refreshed in a similar way to NULL START Code data.</p
><p
>It is not applicable to Alternate START Code schemes where a single packet is sent with a specific meaning and a second identical packet could be treated as a separate message rather than simply a refresh of the same state data.</p
><p
>START Codes for which this behavior are suitable include, but are not limited to (decimal) 1, 2, 6, 10, 221 and some manufacturer specific uses under 145.</p
><section
><hd
>Synchronization and Delivery</hd
><p
>The same requirements for synchronization shall apply as for bindingDMXnull. Note though that the refresh rate for Alternate START Code data is likely to be lower than for NULL START Code.</p
></section
><section
><hd
>Interleaving of START Codes</hd
><p
>Some START Codes are expected to have specific relationships with NULL START Code packets or other Alternat START Code packets (e.g. to follow them one for one). If a device declares an addressing behavior for such a START Code (a specific refinement of DMXpropRef), then it must be assumed that it will apply the correct interleaving rules.</p
><p
>If no specialized rules are applicable for an ALternate START Code then the following shall apply.</p
><p
>For all such Alternate START Codes within a universe which have active bindings, the local device shall transmit data packets with those START Codes in a round-robin fashion.</p
><p
>If one or more local properties is bound to a NULL START Code remote property within a universe, then NULL START Code data shall be transmitted on that universe subject to the refresh rate rules of the applicable protocol (E1.11-2004, E1.31 etc.).</p
><p
>If NULL START Code data is being transmitted on a universe, then no more than one Alternate START Code packet should be sent for each NULL START Code packet.</p
></section
></section
></behaviordef
><!--Streams--><behaviordef
name="streamGroup"
><label
key="streamGroup"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Stream Processing Group</hd
><p
>Many devices may be modelled as a series of operations which are performed on a stream of material. Some examples are an audio channel where the stream of material is the audio signal, a light projector where the material is the light beam itself, some other electrical signal (including a power feed) or some more physical material such as a fluid.</p
><p
>Each point along a stream at which one or more operations takes place is a streamPoint. streamPoints are grouped together in streamGroups to represent a sequence of stream processing steps.</p
><section
><hd
>Order of Processing</hd
><p
>A streamGroup property is a group which shall contain zero or more streamPoints. At each streamPoint there may be a number of operations performed, either monitoring or modifying the stream. Within the streamGroup there is a concept of flow which implies “upstream” and “downstream”. The order of declaration of streamPoints within the streamGroup (document order) shall follow this order of flow. The first streamPoint within any streamGroup determines the nature of the stream – it may represent an origin of material, or in a system of interconnected devices, it can be a point where meterial enters the streamGroup from elsewhere. Subsequent streamPoints represent processing steps.</p
><p
>In cases where independent characteristics of the stream (e.g. the color and the beam-angle of a light source) are modified, these may be declared within the same streamPoint. However, in other cases (e.g. an audio filter followed by an amplifier), the order of processing is important (because the setting of the filter may determine whether the amplifier saturates) and they must appear within successive streamPoints.</p
><p
>Within a single streamPoint, the order of measures or controls is not significant.</p
></section
><section
><hd
>Separation of Streams</hd
><p
>Wherever, this model is used, a streamGroup represents a single stream and separates it from other streams – for example, in a lighting dimmer pack containing six dimmers, each dimmer must be contained in a separate stream.</p
></section
></section
></behaviordef
><behaviordef
name="streamPoint"
><label
key="streamPoint"
set="acnbase.lset"
></label
><refines
name="group"
set="acnbase.bset"
></refines
><section
><hd
>Stream Point</hd
><p
>This is processing point within a streamGroup. See streamGroup behavior for more detail of how streamPoints operate within the group.</p
><p
>A streamPoint is a group which may contain any number of measures which apply to the stream at this point. Measures which are not writable (even indirectly) reflect monitoring points. Measures which can be written or indirectly controlled (e.g. as driven properties) represent controls applied to modify the stream at this point. The characteristics of a streamPoint are determined primarily by the measures within it. Refinements of streamPoint or additional behaviors on the streamPoint property may provide further information or description.</p
><p
>Some aspects of a stream may require multiple measures forming a multidimensionalGroup. For example, the color of a lightBeam.</p
><p
>For streams representing things with a spatial or time-delay content (e.g. a light beam) the mechanism of ordinates and datums can be used to represent the position of the stream point relative to other points.</p
><section
><hd
>Input and Output streamPoints</hd
><p
>In addition to processing, a streamPoint may be one end of a property-reference. Two special cases of streamPoints are input and output points. These represent the (single) point at which material enters a streamGroup and the (zero or more) points at which it leaves it. These are only relevant where the same stream of material is processed in multiple streamGroups within the system and describe the routing of the stream from one group to another. This is very relevant to audio routing and patching for example.</p
></section
><section
><hd
>Stream Origins</hd
><p
>The very first streamPoint in any streamGroup determines the start of the stream within this group. If this is not a streamInput then it represents the origin of the stream. Note however that streamConverter properties originate new streams which are based on other ones.</p
></section
><section
><hd
>Parent Properties</hd
><p
>In general the logical parent of a streamPoint must be a streamGroup. However there are some special cases described in refinements. For example in the case of streamInputs which are children of a streamConverter.</p
></section
></section
></behaviordef
><behaviordef
name="streamInput"
><label
key="streamInput"
set="acnbase.lset"
></label
><refines
name="streamPoint"
set="acnbase.bset"
></refines
><section
><hd
>Stream Input</hd
><p
>This property represents the input of a stream from another place within this or another device. A streamInput property connects to a streamOutput property elsewhere to represent a routing or patching step in the system. This connection is described by a property reference (propertyRef). which identifies the other end of the link which must be a streamOutput. This propertyRef can be either from output to input, or the other way round. If the streamInput property also has a derivative of propertyRef behavior then this defines the streamOutput property to which it is connected. If it does not form a reference then it shall have no value and is simply an anchor for a propertyRef from another place.</p
><p
>If a streamInput property is present in a streamGroup, it shall be the first streamPoint within that group.</p
><p
>The propertyRef mechanism is flexible and can represent anything from hard connections between processing modules in the same device to flexible pathcing mechanisms such as used in the audio industry.</p
><section
><hd
>No Measures in Inputs and Outputs</hd
><p
>streamInput and streamOutput points shall contain no measures, they simply exist as endpoint anchors for propertyReferences describing the interconnection of streams. For inputs a following streamPoint can be used to describe any processing which occurs on input.</p
></section
><section
><hd
>Many-to-one Patching</hd
><p
>Note that in most cases it is not permitted to connect multiple outputs to the same input. In cases where the propertyRef is a part of a streamOutput property and points to aa streamInput, steps must be taken in the structure of the description to ensure that this cannot occur.</p
><p
>In the exceptional case where the equipment is truly capable of accepting an unlimited number of inputs, it is allowable to have many-to-one patching and refinement of input behavior will be required to state how multiple inputs are resolved. However, where the number of input connections is limited (even if large), they should each be expressed as a separate streamInput into a streamCoverter description.</p
></section
><section
><hd
>See also:</hd
><p
>streamPoint, streamOutput.</p
></section
></section
></behaviordef
><behaviordef
name="streamOuput"
><label
key="streamOuput"
set="acnbase.lset"
></label
><refines
name="streamPoint"
set="acnbase.bset"
></refines
><section
><hd
>Stream Output</hd
><p
>This property represents an output of a stream which is or can be routed to another place within this or another device. A streamOutput property connects to a streamInput property elsewhere to represent a routing or patching step in the system. This connection is described by a property reference (propertyRef). which identifies the other end of the link which must be a streamInput. This propertyRef can be either from output to input, or the other way round. If the streamOutput property also has a derivative of propertyRef behavior then this defines the streamInput property to which it is connected. If it does not form a reference then it shall have no value and is simply an anchor for a propertyRef from another place.</p
><p
>The propertyRef mechanism is flexible and can represent anything from hard connections between processing modules in the same device to flexible pathcing mechanisms such as used in the audio industry.</p
><section
><hd
>No Measures in Inputs and Outputs</hd
><p
>streamOutput and streamInput points shall contain no measures, they simply exist as endpoint anchors for propertyReferences describing the interconnection of streams. For outputs the preceding streamPoint(s) determine any processing which has occurred prior to output.</p
></section
><section
><hd
>See also:</hd
><p
>streamPoint, streamInput.</p
></section
></section
></behaviordef
><behaviordef
name="streamCoverter"
><label
key="streamCoverter"
set="acnbase.lset"
></label
><refines
name="streamPoint"
set="acnbase.bset"
></refines
><section
><hd
>Stream Converter</hd
><p
>A streamCoverter property takes one or more input streams and converts them into a new stream. The new stream may be of similar type to its input(s) as some simple combination, or may represent some conversion (e.g. from electrical signal to light).</p
><p
>If a streamCoverter property is present in a streamGroup, it shall be the first streamPoint within that group.</p
><p
>The input streams to the streamCoverter are represented as streamInputs: Any streamInput properties which are logical children of the streamCoverter are considered inputs to the conversion mechanism. They must be linked to streamOutputs elsewhere in order to be active. Unlike in a streamGroup a streamConverter may contain multiple streamInputs. Unless stated by refinements of this behavior, the streamInputs to a streamConverter may occur in any order.</p
><section
><hd
>Examples</hd
><p
>An audio mixer in which multiple streams (input channels) are processed individually before being connected to one or more streamCoverters which combine them to generate output channels.</p
><p
>An RGB lightsource in which three separate sources – red, green and blue - are individually controlled before being combined and further processed (e.g. by lenses or beam deflectors). In this case, each of red, green and blue may itself be the result of a streamConversion from electrical signal to colored light.</p
></section
><section
><hd
>Conversion Algorithm</hd
><p
>The nature of the conversion/combination must be specified by refinement of this behavior. In many cases of conversion, it is sufficient to define the stream generated by the streamConverter since the nature of the inputStreams will be defined by the streams they link to. For combining converters the algorithm for the combining operation must be explicitly defined.</p
></section
></section
></behaviordef
><behaviordef
name="streamRatio"
><label
key="streamRatio"
set="acnbase.lset"
></label
><refines
name="streamPoint"
set="acnbase.bset"
></refines
><refines
name="ratio"
set="acnbase.bset"
></refines
><section
><hd
>Stream Ratio</hd
><p
>This is a stream point which simply changes the value of some measure of a stream. The value of the property shall be the ratio of the measure immediately downstream of this point to that immediately upstream.</p
><p
>This property must describe the dimension and may specify non-linear behaviors in the same way as other ratio properties.</p
><p
>A wide range of amplification and attenuation processes can be described by a streamRatio property. For example in audio systems, both attenuators and amplifiers can be expressed as streamRatios with a dimension of power and a logarithmic nonLinaearity scaled to dB.</p
><p
>It must be noted that the output from a streamRatio is the product of its value and the chosen measure of the input. If its input changes, so does its output. This is different from a regulator which maintains the downstream value independently of the upstream value using feedback or feedforward methods. Thus a simple phase controlled dimmer or variac transformer is a streamRatio, but a voltage regulator or a sophisticated dimmer which modifies its output to compensate for changes in the input is not.</p
><p
>Examples of streamRatio properties include the obvious electrical amplifiers and attenuators familiar in audio, radio etc. but also include dimmers in lighting (both mechanical and electrical), and some pulse width modulation systems.</p
><section
><hd
>Use of Units and Scaling in streamMultipliers.</hd
><p
>A streamMultiplier property represents a ratio and so has no units. Scaling parameters (unitScale, fulScale etc.) within this property therefore simply define what the property units represent in terms of multiplication. For example, a phase controlled dimmer with a single byte level control would typically have a fullScale value of 1.0. A streamMultiplier with a range after scaling from 0 to 2.0 would be capable of either amplifying or attenuating.</p
><p
>Despite this it is useful to indicate in a streamMultiplier the measure of quantity which is being multiplied. For example, in an electrical signal a multiplier which linearly changes voltage will change the power by a squared factor. The multiplierMetric property is provided to give this information.</p
></section
></section
></behaviordef
><behaviordef
name="streamGovernor"
><label
key="streamGovernor"
set="acnbase.lset"
></label
><refines
name="streamRegulator"
set="acnbase.bset"
></refines
><section
><hd
>Stream Governor</hd
><p
>This is a stream regulator which uses feedback or feedforward techniques so that its output is to some extend independent of its input.</p
></section
></behaviordef
><behaviordef
name="beamSource"
><label
key="beamSource"
set="acnbase.lset"
></label
><refines
name="streamSource"
set="acnbase.bset"
></refines
><section
><hd
>Beam Source</hd
><p
>A beam is a stream with a physical size and direction, such as a beam of light or other radiation or jet of fluid.</p
><p
>A beam is defined by an axis specifying its direction and is assumed to be circularly symmetrical about this axis by default. The axis of a beam may not be straight in euclidean space, but defines the direction at any point along it (e.g. a fluid jet may curve under the effect of gravity).</p
><p
>A beamSource may carry additional streamSource behaviors – for example a property with both beamSource and streamConverter behaviors represents a function converting some other stream or streams into a beam.</p
><section
><hd
>Geometry and Frame of Reference</hd
><p
>A beam establishes a frame of reference at any point along the beam (a datum) which is the default datum for its children (and which may be referenced externally using a datumProperty behavior), in which the beam axis is the z-axis (with positive direction being in the direction of the beam), the xy-plane is orthogonal to it and consequently changes to angleZ represent rotation about the axis of the beam. The origin in the z-direction is the source of the beam (streamSource) by default.</p
><p
>In a terrestrial application and where the beam axis is not vertical, the default y-axis direction is defined by the intersection of the beam's xy-plane with a vertical plane perpendicular to it, with positive direction being upward. The x-axis is then orthogonal to both y and z with conventional right hand orientation.</p
><p
>The origin for the beam's frame of reference relative to other parts of the device may be established by specifying a datum for the beam. If no datum is specified or inherited, the beam's own frame of reference still applys to its children but nothing is known about the relationship of this to other parts of the system.</p
></section
></section
></behaviordef
><behaviordef
name="beamDiverter"
><label
key="beamDiverter"
set="acnbase.lset"
></label
><refines
name="streamFilter"
set="acnbase.bset"
></refines
><section
><hd
>Beam Diverter</hd
><p
>A beam is a stream with a physical direction (see beamSource). A beamDiverter changes the direction of the beam. Refinements include mirrors, refractors, wave-guides, pipes (for fluids) etc.</p
></section
></behaviordef
><behaviordef
name="streamFilter"
><label
key="streamFilter"
set="acnbase.lset"
></label
><refines
name="streamModifier"
set="acnbase.bset"
></refines
><section
><hd
>Stream Filters</hd
><p
>Filters are an abstract class of behaviors which in some way modify the material in a stream. For a sound channel, this can be a frequency filter or some other form of modification such as reverb. For a light beam, filters may modify the color, shape or texture of the beam. Filters may apply anywhere the equipment being controlled is manipulating a channel.</p
></section
></behaviordef
><!--Lighting streams--><behaviordef
name="lightSource"
><label
key="lightSource"
set="acnbase.lset"
></label
><refines
name="streamSource"
set="acnbase.bset"
></refines
><section
><hd
>Light Source</hd
><p
>A stream source representing a light source.</p
><p
>Refinements of this behavior define specific types of source – e.g. directional sources of various kinds, lasers etc. Light sources will usually have other streamSource behaviors in addition – e.g. streamCoverter for a source in which the power to the source is also modelled, or for a source combining other streams (RGB mixer etc.).</p
></section
></behaviordef
><behaviordef
name="colorSpec"
><label
key="colorSpec"
set="acnbase.lset"
></label
><section
><hd
>Color Specifier</hd
><p
>A property defining a color.</p
><p
>May take the form of a group of orthogonal color axes (e.g. RGB, CMYK, HSB) or another method such as a selector or PanTone number. Additional behaviors will define the method.</p
></section
></behaviordef
><behaviordef
name="colorFilter"
><label
key="colorFilter"
set="acnbase.lset"
></label
><refines
name="colorSpec"
set="acnbase.bset"
></refines
><refines
name="filter"
set="acnbase.bset"
></refines
><section
><hd
>Color Filter</hd
><p
>A property which applies a filter to a light beam (a stream) to change or dictate it's color.</p
></section
></behaviordef
><behaviordef
name="beamShape"
><label
key="beamShape"
set="acnbase.lset"
></label
><refines
name="streamModifier"
set="acnbase.bset"
></refines
><section
><hd
>Beam Shape</hd
><p
>This is a filter which may be applied to a beam to change the spatial shape of the beam. Examples are apertures, irises, stops (e.g. shutters, “barn doors”) or more complex shaping devices such as masks, mattes, templates or gobos. The beamShape property itself has no value although refinements may define one.</p
><p
>Unless otherwise defined, generic spatial parameters applied to beam shapes act in a plane orthogonal to the beam axis and with the beam axis as the default origin. For example a twist angle is an angle of rotation about the beam axis, while an orientation in two dimensions is by default within a plane orthogonal to the beam and with its origin at the beam centre.</p
></section
></behaviordef
><behaviordef
name="beamTemplate"
><label
key="beamTemplate"
set="acnbase.lset"
></label
><refines
name="beamShape"
set="acnbase.bset"
></refines
><section
><hd
>Beam Template</hd
><p
>This is an arbitrary shaping template applied to a stream such as a lightbeam to change the spatial shape of the beam. Labels or dynamically determined behaviors (see behaviorRef) may be attached to individual templates.</p
></section
></behaviordef
><behaviordef
name="opticalLens"
><label
key="opticalLens"
set="acnbase.lset"
></label
><refines
name="streamFilter"
set="acnbase.bset"
></refines
><section
><hd
>Lens</hd
><p
>This property is a lens which is typically present in a streamGroup representing a light beam.</p
><p
>Unless refinements specify otherwise (e.g. astigmatic or other aspheric lenses), the lens is assumed to approximate to a “thin lens” and the value of the property shall be the focal length of the lens (negative for concave lenses).</p
><section
><hd
>Geometry and Frame of Reference</hd
><p
>A lens' frame of reference is defined to have the z-axis coincident with the optical axis of the lens, and the xy-plane (z = 0) passing through its centre. This frame-of reference can be related to other parts of the system using datum references in the normal way. See datum behaviors.</p
><p
>Refinements of opticalLens may need to further specify the frame of reference – for example to define the astigmatic axis or the centre point in a complex lens.</p
></section
></section
></behaviordef
><behaviordef
name="simplified-specialized"
><label
key="simplified-specialized"
set="acnbase.lset"
></label
><section
><hd
>Simplified Behavior for Specialized Application</hd
><p
>This base behaviorset aims to provide a very generic and fine-grained description of the elements of control which enables building a reasonable control model for many devices from first principles. However, there are many controllers whose scope is highly specialized whilst capability to build flexible control paradigms is highly limited. The use of refinements of simplified-specialized behavior alongside the generic behaviors of this base set allow creation of sets of easily identified but more specialist and restricted behaviors to be recognized by such controllers.</p
><p
>For example, many simple lighting consoles are not concerned with generic equipment which includes audio, automation or other common entertainment technology disciplines. They also frequently make a set of fixed assumptions about what a light is and can do. More esoteric geometries, effects or features are either completely beyond them or have to be approximated for better or worse by whatever fixed model they can provide. Refinement of this behavior to create behaviors for common lighting concepts like “pan”, “tilt” or “goboselect” can allow these controllers to find the bulk of properties of interest without resorting to first principles.</p
></section
><section
><hd
>Rules for use of simplified-specialized behaviors</hd
><p
>simplified-specialized behaviors are not replacements for more generic behaviors but must be used alongside them so that more capable controllers can still build a model from first principles without needing to implement a multitude of specialist behaviors.</p
><p
>Refinements of simplified-specialized behaviors must be defined in new behaviorsets whose scope and applicability are clearly defined.</p
><p
>Each refinement of simplified-specialized behavior must clearly set out assumptions it makes about the underlying model and the scope of its applicability. It is not enough to simply state "this property controls the foobar function of a gizmo" but must indicate what counts as a foobar function and what does not (there may be other gizmos with a similar but distinct function, do these count? What if the function is non-linear or controlled by a text string instead of an integer?) and any restrictions or limitations on its use. It should also refer to base behaviors which are permitted or likely to be encountered on the same property.</p
></section
></behaviordef
></behaviorset
></DDL
>
