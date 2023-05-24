#set text(size: 14pt, font: "IBM Plex Serif")
#set page(numbering: "1")

#align(center, text(25pt)[
  *Resurgence Virtual Machine Specification, Version 7.0*
])

#align(center, text(12pt)[
  Last edited: May 24th, 2023
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

Implementations that do not implement all "MUST" behavior or implement "MUST NOT" behaivor is considered out of compliance with this specification. Implementations MAY implement other features not included in this specification on an as-needed basis, so long as they do not affect compliance with this specification.

== Versioning
TODO

#set heading(numbering: "1.")
= Memory Layout
TODO

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

= Portable Bytecode Format
Resurgence uses a binary data format that is designed to be fast, flexible, and portable.

== Data Types

=== Booleans
Booleans are expressed as a single 8-bit value that is either True or False. True SHOULD be represented as a value of 0x01, and False MUST be represented as a value of 0x00. Implementations MUST treat all non-zero values of boolean fields as True.

=== Integers
All integers are represented using big endianness. This means that for multi-byte integers, the most significant byte is first, and then in descending order of significance. Signed integers use the most significant bit to store integer polarity, and Unsigned integers do not.

The format uses the following integer formats:
- `u32` - Unsigned 32-bit integer
- `u64` - Unsigned 64-bit integer
- `i64` - Signed 64-bit integer

=== Floating Point Numbers
All floating point numbers used in the Bytecode format are 64-bit values represented using the "binary64" format from the IEEE 754-2008 standard. Implementations MUST use the IEEE 754-2008 specification when processing floating point numbers with the portable format. However, implementations MAY convert to/from other formats for internal use.

Floating point numbers are represented using big endianness.

=== Strings
Strings are UTF-8 text whose length is described by a leading `u64` value, followed by the raw bytes of the string. Implementations MUST NOT insert or rely on null terminators (0x00) for delimiting the end of strings when representing them in the portable format.

=== Registers
TODO

=== Register References
TODO

=== Register Location
TODO

