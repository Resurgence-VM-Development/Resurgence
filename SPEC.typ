#set text(size: 14pt, font: "IBM Plex Serif")
#set page(numbering: "1")

#align(center, text(25pt)[
  *Resurgence Virtual Machine Specification, Version 7.0*
])

#align(center, text(12pt)[
  DRAFT -- Last edited: June 5th, 2023
])

#grid(
  columns: (1fr, 1fr),
  align(center)[
    StandingPad (Mahid Sheikh)
  ],
  align(center)[
    Dynafide (Chris Morgan)
  ],
)

= Table of Contents
#outline(title: none, indent: true)

#pagebreak(weak: true)

= Overview
There are 3 big parts to a _Resurgence Virtual Machine_:
- Memory layout
  - Global registers
  - Stack
  - Call Stack
  - Stackframes
  - Constants
- Instructions
  - Instruction set
  - Instruction behavior
  - Representation in binary files
- Function API
  - Calls Table
  - ResurgenceState

This specification defines all of these. The reference implementation of Resurgence can be found on *#link("https://github.com/Resurgence-VM-Development/Resurgence", "GitHub")*, although it should be known that the reference implementation also adds a C FFI, forward facing API, and code generation API, all of which are not defined in the specification and thus should be considered implementation unique.

== Requirements
The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED",  "MAY", and "OPTIONAL" in this document are to be interpreted as described in *#link("https://www.rfc-editor.org/rfc/rfc2119", "RFC 2119")*.

Implementations that do not implement all "MUST" behavior or implement "MUST NOT" behavior is considered out of compliance with this specification. Implementations MAY implement other features not included in this specification on an as-needed basis, so long as they do not affect compliance with this specification.

== Versioning <versioning>
The specification is versioned such that non-breaking changes are backwards compatible with previous versions. This is achieved by use of a `MAJOR.MINOR` versioning scheme.

The `MAJOR` version is incremented whenever breaking changes are made to the specification which prevent an implementation from reading older versions.

The `MINOR` version is incremented whenever a change is made that breaks implementations of previous versions, but is backwards compatible with older Bytecode.

For example: The reference implementation of Resurgence using version 5.2 would be able to decode Bytecode generated with version 5.2, 5.1, and 5.0, but would not be able to handle versions 5.3, 6.0 or 4.0.

An implementation MUST validate that it is capable of processing Bytecode of the version(s) it implements. Implementations MUST perform checks to reject any Bytecode generated using newer versions of this specification that it does not explicitly support.

An implementation MAY choose to implement backwards compatibility features to handle multiple specification versions without issue.

#pagebreak(weak: true)

#set heading(numbering: "1.")
= Interpreter Design
== Execution Loop
An _execution loop_ SHALL be defined as the main part of a _Resurgence Virtual Machine_ that executes instructions. An execution loop MUST have an _Instruction Pointer_ (represented by an unsigned 64-bit integer), which SHALL point to the intex of the current instruction. All programs SHALL begin at index 0. The _Instruction Pointer_ MUST NOT be accessible to the program, but certain instructions will change the value of the _Instruction Pointer_.

An implementation MAY choose to only have one execution loop, or have multiple _execution loops_ for multithreading reasons. As of the 0.2.0 specification, there are no instructions for threading, so multithreading instructions SHOULD be considered non-standard for now.

== Memory Layout
Resurgence defines 2 parts to the memory layout:
- _Global_
- _Local_

_Global_ SHALL be defined as all parts of interpreter shared by all execution loop, whereas _Local_ SHALL be defined as all parts of the interpreter unique to each execution loop.

The following are defined as _Global_:
- Global Registers
- Functions declared through the Function API

The following meanwhile are defined as _Local_:
- Stack
- Call Stack
- Accumulator

=== Internal Types
Resurgence defines the following types:
- Int: a signed, 64-bit integer
- Float/Double: a signed, 64-bit float
- String: a UTF-8 string with no null terminator
- Bool: a simple _true_ or _false_

There SHALL be no integer promotion.


=== Global Registers
Global registers are represented by an array of values. Global registers MUST be accessible to the entire program and all threads of applicible.

=== Function API
Functions can be defined and registered for use in the Resurgence runtime. All functions registered with the runtime MUST be usable to the entire program.

Functions are to take an object called _ResurgenceState_, which represents a gate to the internal memory. This object SHALL have functions associated with it that would allow taking values off of the stack.

Example:
```py
def foo(r: ResurgenceState):
  bar: int = r.get_i64()
  x = bar + 1
  r.push_int(x)
```

==== ResurgenceState
  _ResurgenceState_ SHALL define the following functions to retrive passed arguments:
- `get_i64() -> i64`
- `get_f64() -> f64`
- `get_string() -> UTF_8_String`
- `get_bool() -> bool`
- `get_value_as_string() -> UTF_8_String`
Arguments to a function shall be passed through the Stack and retrived in a LIFO fashion. In other words, the first argument shall be the last argument.

In addition

In addition, _ResurgenceState_ shall also define the following functions to push values on the stack:
- `push_i64(i64)`
- `push_f64(f64)`
- `push_string(UTF_8_String)`
- `push_bool(bool)`

All values passed through these functions SHALL populate the stack in a LIFO fashion. In other words, the first value pushed on the stack shall be the last value taken by the program.

==== Stack
The Stack SHALL be a stack of values. Each _execution loop_ MUST have their own stack. The Stack is used for the following:
- Passing arguments to external, registered functions
- Retriving return values from external, registered functions

In addition, the Stack may also be used by the program for anything that may benefit or be simpler to implement with it. However, operations MUST NOT deal with the Stack directly, only registers. The `stack_mov` operation exists to move values from the top of the stack to a register.

If multithreaded, the Stack MUST be unique to each _execution loop_.

==== Call Stack
The Call Stack SHALL be a stack that holds _Stackframes_. A _Stackframe_ SHALL be a set of registers created with the `alloc` instruction.

`LOCAL` SHALL reference the top most _Stackframe_.

If multithreaded, the Call Stack MUST be unique to each _execution loop_.

==== Accumulator
The Accumulator SHALL be a 64-bit floating point that can be used in any register argument. The location of the Accumulator in instructions is to be defined as `ACCU`.

#pagebreak(weak: true)

= Instructions
Resurgence has 25 instructions. This part of the specification defines those instructions. The reference implementation also declares, but does not define, 5 more instructions.
#footnote[#link("https://github.com/Resurgence-VM-Development/Resurgence/blob/8bfe13f9205b28fcea04e0a527bd05fe451d5a9f/src/objects/instruction.rs#L189", "Additional instructions in the Resurgence reference implementation (link)")]
Since these are not defined nor formalized, they will not be included in this version of the spec. Developers should simply ignore these instructions.

Some terminology:
- `REG`: A location in memory represented by a `LOC` (see below) and an unsigned 32-bit integer
  - `CONST`: Constant Pool
  - `GLOBAL`: Global Registers
  - `ACCU`: Accumulator
  - `LOCAL`: Top StackFrame on the Call Stack
- `REG_REF`: Whether to fully qualify Addresses or not
  - `DEREFERENCE`: Fully qualify addresses
  - `AS_IS`: Ignore the address a register holds
- Instruction Pointer: a unsigned 64-bit integer representing the current index in the execution loop, starting from 0

All instructions in this specification are written as follows:
```
instruction argument_name <type>,...
```

In addition, all integer overflows for instructions SHALL always be undefined behavior.

== Memory Management

=== Alloc
```
alloc n <u32>
```
Creates a new Stackframe on the Call Stack with $n$ amount of registers, $n$ being an unsigned 32-bit integer.

=== FrameAlloc
```
frame_alloc n <u32>, loc <LOC>
```
Adds $n$ additional registers to `loc`, where $n$ is an unsigned 32-bit integer and `loc` is either `GLOBAL` or `LOCAL`.

=== Free
```
free n <u32>
```
Removes $n$ amount of Stackframes from the Call Stack, where $n$ is an unsigned 32-bit integer. This is in contrast to `Alloc`, where $n$ is the amount of registers.

It SHALL be undefined behavior for $n$ to be greater than the amount of Stackframes on the Call Stack. For example, if there are 3 Stackframes, then `free 4` is undefined behavior.

=== FrameFree
```
frame_free n <u32>, loc <LOC>
```
Removes $n$ amount of registers from `loc`, where $n$ is an unsigned 32-bit integer and `loc` is either `GLOBAL` or `LOCAL`.

It SHALL be undefined behavior for $n$ to be greater than the amount of registers in `loc`. For example, if there are 2 GLOBAL registers, then `frame_free 3, GLOBAL`

== Execution Control

=== Jump
```
jump n <i64>
```
Adds $n$ to the Instuction Pointer, where $n$ is a signed 64-bit integer. If $n$ is negative, then `jump` effectively goes backwards.

It SHALL be undefined behavior if $n$ makes the Instruction Pointer go beyond bounds, or if $n$ makes the Instruction Pointer become negative. For instance, if the instruction pointer is at 9, then `jump -11` is undefined behavior.

=== Call
```
call n <u64>
```
Stashes the current value of the Instruction Pointer and sets it to $n$, where $n$ is an unsigned 64-bit integer.

It SHALL be undefined behavior if $n$ goes beyond bounds. For instance, if the code size is 10 instructions (0 to 9), then `call 10` is undefined behavior.

In the reference implementation of Resurgence, stashing the value of the Instruction Pointer is done with recursion.
#footnote[#link("https://github.com/Resurgence-VM-Development/Resurgence/blob/1e3c330ad2878c1cb9d3bef49f599a02df31a787/src/internal/interpreter/execution_engine.rs#L113", "Resurgence Implementation of Call (link)")]
How stashing is implemented is merely an implementation detail. While `call` implies calling a function, it is perfectly valid to also create a non-recursive setup with a seperate data structure so long as the code can not access said data structure.

=== ExtCall
```
ext_call ID <u64>
```
Calls an external function, where `ID` is the unique ID number of the function the program wishes to call.

It SHALL be undefined behavior for `ID` to be a value that is not assigned to an external function.

In the reference implementation of Resurgence, external functions are either defined in Rust or C, and assigned using register functions.
#footnote[#link("https://github.com/Resurgence-VM-Development/Resurgence/blob/8bfe13f9205b28fcea04e0a527bd05fe451d5a9f/src/internal/interpreter/imports.rs#L15", "Resurgence Implementation of register functions (link)")]
This is merely an implementation detail. When it comes to external functions, all that matters is the Function API.

=== Ret
```
ret
```
Resets the Instruction Pointer to a previous value if possible; otherwise, the program exits.

In the reference implementation of Resurgence, this is done by returning from recursive calls done by `call`.
#footnote[#link("https://github.com/Resurgence-VM-Development/Resurgence/blob/8bfe13f9205b28fcea04e0a527bd05fe451d5a9f/src/internal/interpreter/execution_engine.rs#L134", "Resurgence Implementaion of Ret (link)")]
This is merely an implementation detail. While `ret` implies returning, all it really does is set the Instruction Pointer to a previous value when possible, and exits the program otherwise.

== Memory Manipulation

=== Mov
```
mov dst <REG>, dst_ref <REG_REF>, src <REG>, src_ref <REG_REF>
```
Moves a value from `src` to `dst`. 
- If `dst` holds an address, then the address will be fully resolved if `dst_ref` is set to `DEREFERENCE`.
- If `src` holds an address, then the address will be fully resolved if `src_ref` is set to `DEREFERENCE`.

It SHALL be undefined behavior for the following:
- Either `dst` or `src` to be beyond bounds.
- To set `dst_ref`/`src_ref` to `DEREFERENCE` if `dst`/`src` do not hold addresses.
- To move a value from/to a register in `CONST`.
- To move a non-double type to `ACCU`.

=== Cpy
```
cpy dst <REG>, dst_ref <REG_REF>, src <REG>, src_ref <REG_REF>
```
Copies a value from `src` to `dst`. 
- If `dst` holds an address, then the address will be fully resolved if `dst_ref` is set to `DEREFERENCE`.
- If `src` holds an address, then the address will be fully resolved if `src_ref` is set to `DEREFERENCE`.

It SHALL be undefined behavior for the following:
- Either `dst` or `src` to be beyond bounds.
- To set `dst_ref`/`src_ref` to `DEREFERENCE` if `dst`/`src` do not hold addresses.
- To copy a value to a register in `CONST`.
- To copy a non-double type to `ACCU`.

=== Ref
```
ref dst <REG>, dst_ref <REG_REF>, src <REG>, src_ref <REG_REF>
```
Takes the address of `src` and sets `dst` to hold said address.
- If `dst` holds an address, then the address will be fully resolved if `dst_ref` is set to `DEREFERENCE`.
- If `src` holds an address, then the address will be fully resolved if `src_ref` is set to `DEREFERENCE`.

It SHALL be undefined behavior for the following:
- Either `dst` or `src` to be beyond bounds.
- To set `dst_ref`/`src_ref` to `DEREFERENCE` if `dst`/`src` do not hold addresses.
- To set `src` to a register in `CONST` or to set `src` to `ACCU`.

=== StackPush
```
stack_push src <REG>, src_ref <REG_REF>
```
Moves the value stored in `src` to the top of the Stack. If `src` holds an adress, then the address will be fully resolved if `src_ref` is set to `DEREFERENCE`.

It SHALL be undefined behavior for the following:
- `src` to be beyond bounds.
- To set `src_ref` to `DEREFERENCE` if `src` does not hold an address.

=== StackPop
```
stack_pop
```
Pops the top element off of the stack.

In the future, `stack_pop` may be merged with `stack_mov`.
=== StackMov
```
stack_mov dst <REG>, dst_ref <REG_REF>
```
Moves the top element from the stack to `dst`. If `dst` holds an adress, then the address will be fully resolved if `dst_ref` is set to `DEREFERENCE`.

It SHALL be undefined behavior for the following:
- `dst` to be beyond bounds
- To set `dst_ref` to `DEREFERENCE` if `dst` does not hold an address.

In the future, `stack_mov` may be merged with `stack_pop`.

== Operators

=== Add
```
add dst <REG>, src_1 <REG>, src_2 <REG>
```
Adds `src_1` and `src_2`, storing the result in `dst`. If `src_1` holds an address and `src_2` holds an integer, then pointer arithmethic can be performed.

The type of the result MUST be:
- A. The type of `src_1` if both `src_1` and `src_2` are integers
- B. Address if one of the source registers is an address and the other an integer type.
  - The resulting address shall point to `original address + value of other source register`
- C. A 64-bit float if either `src_1` or `src_2` are also a 64-bit float
  - The size of the source register that holds the float (if only one source register holds a float)
  - The largest float size if both source registers hold floats

It SHALL be undefined behavior for the following:
- For `src_1` and `src_2` to both hold addresses
- For `src_1` and/or `src_2` to hold a non-numeric type

=== Sub
```
sub dst <REG>, src_1 <REG>, src_2 <REG>
```
Subtracts `src_1` and `src_2`, storing the result in `dst`. If `src_1` holds an address and `src_2` holds an integer, then pointer arithmethic can be performed.

The type of the result MUST be:
- A. The type of `src_1` if both `src_1` and `src_2` are integers
- B. Address if one of the source registers is an address and the other an integer type.
  - The resulting address shall point to `original address - value of other source register`
- C. A 64-bit float if either `src_1` or `src_2` are also a 64-bit float
  - The size of the source register that holds the float (if only one source register holds a float)
  - The largest float size if both source registers hold floats

It SHALL be undefined behavior for the following:
- For `src_1` and `src_2` to both hold addresses
- For `src_1` and/or `src_2` to hold a non-numeric type

=== Mul
```
mul dst <REG>, src_1 <REG>, src_2 <REG>
```
Multiplies `src_1` and `src_2`, storing the result in `dst`.

The type of the result MUST be:
- A. The type of `src_1` if both `src_1` and `src_2` are integers
- B. A 64-bit float if either `src_1` or `src_2` are also a 64-bit float
  - The size of the source register that holds the float (if only one source register holds a float)
  - The largest float size if both source registers hold floats

It SHALL be undefined behavior for the following:
- For `src_1` and `src_2` to both hold addresses
- For `src_1` and/or `src_2` to hold a non-numeric type

=== Div
```
divides dst <REG>, src_1 <REG>, src_2 <REG>
```
Multiplies `src_1` by `src_2`, storing the result in `dst`.

The type of the result MUST be:
- A. The type of `src_1` if both `src_1` and `src_2` are integers
- B. A float if either `src_1` or `src_2` are also a float. The size SHALL be decided by either:
  - The size of the source register that holds the float (if only one source register holds a float)
  - The largest float size if both source registers hold floats

It SHALL be undefined behavior for the following:
- For `src_1` and `src_2` to both hold addresses
- For `src_1` and/or `src_2` to hold a non-numeric type
- To have `src_1` and/or `src_2` hold addresses.
- For `src_2` to be 0.

=== Mod
```
divides dst <REG>, src_1 <REG>, src_2 <REG>
```
Multiplies `src_1` by `src_2`, storing the remainder in `dst`.

The type of the result MUST be:
- A. The type of `src_1` if both `src_1` and `src_2` are integers
- B. A float if either `src_1` or `src_2` are also a float. The size SHALL be decided by either:
  - The size of the source register that holds the float (if only one source register holds a float)
  - The largest float size if both source registers hold floats

It SHALL be undefined behavior for the following:
- For `src_1` and `src_2` to both hold addresses
- For `src_1` and/or `src_2` to hold a non-numeric type
- To have `src_1` and/or `src_2` hold addresses.
- For `src_2` to be 0.

== Comparison
=== Equal
```
equal src_1 <REG>, src_2 <REG>
```

Compares `src_1` and `src_2` for equality. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be compatible with each other in terms of equality. If both `src_1` and `src_2` hold the same type, then they can be compared. Otherwise, the comparison MUST be one of the following:
- Integer to Float (the integer SHALL be interpreted as a float for the comparison)

In addition, Address to Bool comparison MAY be allowed. In that case, an Address is true if it exists and false if it doesn't.

It SHALL be undefined behavior to compare 2 types that are not compatible for comparison.

=== NotEqual
```
not_equal src_1 <REG>, src_2 <REG>
```

Compares `src_1` and `src_2` for inequality. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be compatible with each other in terms of equality. If both `src_1` and `src_2` hold the same type, then they can be compared. Otherwise, the comparison MUST be one of the following:
- Integer to Float (the integer SHALL be interpreted as a float for the comparison)

In addition, Address to Bool comparison MAY be allowed. In that case, an Address is true if it exists and false if it doesn't.

It SHALL be undefined behavior to compare 2 types that are not compatible for comparison.

=== Greater
```
greater src_1 <REG>, src_2 <REG>
```

Compares checks if `src_1` is greater than `src_2`. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be numeric types. It SHALL be undefined behavior otherwise. In addition, if one of the source registers is a float and the other an int, then the int SHALL be interpreted as a float.

=== Less
```
less src_1 <REG>, src_2 <REG>
```

Compares checks if `src_1` is less than `src_2`. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be numeric types. It SHALL be undefined behavior otherwise. In addition, if one of the source registers is a float and the other an int, then the int SHALL be interpreted as a float.

=== GreaterEqual
```
greater_equal src_1 <REG>, src_2 <REG>
```

Compares checks if `src_1` is greater than or equal to `src_2`. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be numeric types. It SHALL be undefined behavior otherwise. In addition, if one of the source registers is a float and the other an int, then the int SHALL be interpreted as a float.

=== LessEqual
```
less_equal src_1 <REG>, src_2 <REG>
```

Compares checks if `src_1` is less than or equal to `src_2`. If the result is true, then the instruction pointer is incremented by one and the following instruction is skipped.

Both `src_1` and `src_2` MUST be numeric types. It SHALL be undefined behavior otherwise. In addition, if one of the source registers is a float and the other an int, then the int SHALL be interpreted as a float.

#pagebreak(weak: true)

= Portable Bytecode Format
Resurgence uses a special binary data format, known as Bytecode, to represent instructions and other information required at runtime. The Bytecode format is designed with the following goals:
- Fast to encode and decode
- Easy to understand, helping developers write compilers and implementations
- Flexible design for many use cases
- Portable across operating systems and architectures
- Efficient usage of available storage / bandwidth

== Data Types

=== Booleans
Booleans are expressed as a single 8-bit value that is either True or False. True SHOULD be represented as a value of 0x01, and False MUST be represented as a value of 0x00. Implementations MUST treat all non-zero values of boolean fields as True.

=== Integers
All integers are represented using big endianness. This means that for multi-byte integers, the most significant byte is first, and then in descending order of significance. Signed integers use the most significant bit to store integer polarity, and Unsigned integers do not.

The format uses the following integer formats:
- `u8` - Unsigned 8-bit integer
- `u16` - Unsigned 16-bit integer
- `u32` - Unsigned 32-bit integer
- `u64` - Unsigned 64-bit integer
- `i64` - Signed 64-bit integer

=== Floating Point Numbers
All floating point numbers used in the Bytecode format are 64-bit values represented using the "binary64" format from the IEEE 754-2008 standard. Implementations MUST use the IEEE 754-2008 specification when processing floating point numbers with the portable format. However, implementations MAY convert to/from other formats for internal use.

Floating point numbers are represented using big endianness.

=== Strings
Strings are UTF-8 text whose length is described by a leading `u64` value, followed by the raw bytes of the string. Implementations MUST NOT insert or rely on null terminators (0x00) for delimiting the end of strings when representing them in the portable format.

=== Registers <registers>
Registers are an enumeration type that describes a location in the Virtual Machine's memory. Registers are described by a `u8` "location" field which describes what section of the memory it is stored in, followed by a `u32` "position" field which describes the position of that value in that section of the Virtual Machine's memory. The "Location" field MUST use the following values to describe memory locations:
#table(
  columns: 2,
  [*Location*], [*Value*],
  [Constant], [01],
  [Accumulator], [02],
  [Global Register], [03],
  [Local Register], [04],
)

The exact meaning of the "Position" field varies, depending on the Location value. The following table describes the relationship:
#table(
  columns: 2,
  [*Location*], [*Meaning of Position Field*],
  [Constant], [Corresponds to the the index number of a constant in the bytecode constants table. See #link(<constants_table>,"Constants Table section") for details.],
  [Accumulator], [Not used. Position field MUST be set to zero.],
  [Global Register], [Corresponds to a value in the global registers table, created at runtime.],
  [Local Register], [Corresponds to a value in the local registers table, created at runtime.],
)


=== Register References
Register Reference values are used to indicate whether values should be used as-is or de-referenced before use. These values are useful for replicating the functionality of pointers. They consist of a `u8` value which MUST either be `01` for as-is / verbatim use or `02` to enable de-referencing.

=== Register Locations
Register Locations are a simple type that describes the location of a section in the Virtual Machine's memory. The meaning of this type MUST be exactly equivalent to the "Location" field in the Register type, but MUST NOT have the corresponding the "position" field. See the #link(<registers>, "Registers section") for details on the exact meaning of the register location field.

=== Constants
Constants are an enumeration type that describes a hard-coded value in the bytecode's constants section. Constants are described by a `u8` "type" field, followed by its contents, as described in previous sections of this specification. Constants MUST support Integers, Floating Point Numbers, Strings, Booleans, and Registers as the format of its contents. The following table lists the values (formatted as hexadecimals) that MUST be used to describe the supported types:
#table(
  columns: 2,
  [*Type*], [*Value*],
  [Integer], [01],
  [Float], [02],
  [String], [03],
  [Boolean], [04],
  [Register], [05],
)

To describe a Constant containing a String, for example, the constant type field would be set to `03` and would be followed by the String's `u64` length field and its textual contents.

== Bytecode Header
The Bytecode contains a header before instructions are listed. This header contains the following:
- A magic number, to identify that this is valid Bytecode information
- Major and minor specification version that the Bytecode is compliant with
- Constants table, defining all constant values used in the program
- Imports table, listing runtime features required by the program
- Exports table, listing functions that the program implements, as well as the position in the instructions that the function starts at

=== Magic Number
The magic number is a fixed 32-bit value at the beginning of all Bytecode instances. Implementations MUST use the following value: 0x52564D88 (hexadecimal). To ensure stability, implementations MUST check to ensure  that a bytecode instance begins with this value before beginning processing or execution.

=== Version Information
`major_version <u16>, minor_version <u16>`

The version information is expressed as a `u16` major version, followed by a `u16` minor version. This version number is explained in #link(<versioning>,"the Versioning section"). To ensure stability, implementations MUST also check this value to ensure runtime compatibility and prevent undefined behavior from occurring.

=== Constants Table <constants_table>
`length <u32>, constant <Constant>, ...`

The constants table is expressed as `u32` length value indicating the number of entries in the constants table, followed by Constant objects representing the constant values.

Implementations MUST preserve the order of these values, as the index numbers are used to refer to these constant values in the instructions.

=== Imports Table
`length <u64>, func_name <String>, ...`

The imports table is expressed as a `u64` length value indicating the number of entries in the imports table, followed by String objects which each contain the names of each function which MUST be loaded by the implementation when preparing to execute the code.

Implementations MUST preserve the order of these values, as the index numbers are used in the `ExtCall` instruction.

=== Exports Table
`length <u64>, (func_name <String>, index <u64>), ...`

The exports table is expressed as a `u64` length value indicating the number of entries in the exports table, followed by the following for each entry:
- A string, representing the name of the exported function
- A `u64` value, representing the index of the first instruction of the function. 

*NOTE:* Implementations MUST use the index number in entries as the instruction number, NOT as the position of the instruction in bytes in the Bytecode.

== Instructions Section
`(inst_type <u8>, args)...`

The instructions section comprises the body of the Bytecode. Each entry consists of the instruction's type, expressed as a `u8`, followed by its parameters.

The instructions type values are listed in the following table. Values are expressed as hexadecimal numbers.
#table(
  columns: 2,
  [*Instruction*], [*Value*],
  [Alloc], [01],
  [FrameAlloc], [15],
  [Free], [02],
  [FrameFree], [16],
  [Jump], [03],
  [Call], [04],
  [ExtCall], [05],
  [Ret], [19],
  [Mov], [06],
  [Cpy], [07],
  [Ref], [08],
  [StackPush], [09],
  [StackPop], [0A],
  [StackMov], [17],
  [Add], [0B],
  [Sub], [0C],
  [Mul], [0D],
  [Div], [0E],
  [Mod], [18],
  [Equal], [0F],
  [NotEqual], [10],
  [Greater], [11],
  [Less], [12],
  [GreaterEqual], [13],
  [LessEqual], [14],
)


*NOTE:* Unlike other sections of the bytecode, this section does NOT specify a length field. Implementations MUST read instructions until the read cursor reaches the end of the bytecode. If a given bytecode instance does not have the appropriate length given its instructions, implementations MUST indicate failure in some way.
