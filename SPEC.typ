#set text(size: 14pt, font: "IBM Plex Serif")
#set page(numbering: "1")

#align(center, text(25pt)[
  *Resurgence Virtual Machine Spec for 0.2.0*
])

#align(center, text(12pt)[
  Last edited: May 22rd, 2023
])

#grid(
  columns: (1fr, 1fr),
  align(center)[
    StandingPad
  ],
  align(center)[
    Dynafide
  ],
)

= Table of Contents
#outline(title: none, indent: true)

= Resurgence Virtual Machine Overview
There are 3 big parts to a _Resurgence Virtual Machine_:
- Memory layout
  - Global registers
  - Stack
  - Call Stack
  - Stackframes
- Instructions
  - Instruction set
  - Instruction behavior
  - Representation in binary files
- Function API
  - ResurgenceState

This spec defines all of these. The reference implementation of Resurgence can be found on *#link("https://github.com/Resurgence-VM-Development/Resurgence", "GitHub")*, although it should be known that the reference implementation also adds a C FFI, forward facing API, and code generation API, all of whihc are not defined in the spec and thus should be considered implementation unique.

#set heading(numbering: "1.")
= Memory Layout
For later

= Instructions
The following defines the spec regardding Resurgence instructions, their behaviors, and their representation in binary.

== Instructions and Behaviors
Resurgence has 25 instructions. This part of the spec defines those instructions.

Some terminology:
- Register: A location in memory represented by a Loc (see below) and an unsigned 32-bit integer
  - Const: Constant Pool
  - Global: Global Registers
  - Accu: Accumulator
  - Local: Top StackFrame on the Call Stack
- RegReference: Whether to fully qualify Addresses or not
- Instruction Pointer: a unsigned 64-bit integer representing the current index in the execution loop, starting from 0

=== Alloc
```
alloc <u32>
```
Creates a new Stackframe on the Call Stack with $n$ amount of registers, $n$ being an unsigned 32-bit integer.
=== FrameAlloc
```
frame_alloc <u32>, <loc>
```
Adds $n$ additional registers to $"loc"$, where $n$ is an unsigned 32-bit integer and $"loc"$ is either `Global` or `Local`.

=== Free
```
free <u32>
```
Removes $n$ amount of Stackframes from the Call Stack, where $n$ is an unsigned 32-bit integer. This is in contrast to `Alloc`, where $n$ is the amount of registers.

=== FrameFree
```
frame_free <u32>, <loc>
```
Removes $n$ amount of registers from $"loc"$, where $n$ is an unsigned 32-bit integer and $"loc"$ is either `Global` or `Local`.

=== Jump
```
jump <i64>
```
Adds $n$ to the Instuction Pointer, where $n$ is a signed 64-bit integer. If $n$ is negative, then `jump` effectively goes backwards.

It is undefined behavior if $n$ makes the Instruction Pointer go beyond bounds, or if $n$ makes the Instruction Pointer become negative. For instance, if the instruction pointer is at 9, then `jump -11` is undefined behavior.

=== Call
```
call <u64>
```
Stashes the current value of the Instruction Pointer and sets it to $n$, where $n$ is an unsigned 64-bit integer.

It is undefined behavior if $n$ goes beyond bounds. For instance, if the code size is 10 instructions (0 to 9), then `call 10` is undefined behavior.

In the reference implementation of Resurgence, stashing the value of the Instruction Pointer is done with recursion.
#footnote[#link("https://github.com/Resurgence-VM-Development/Resurgence/blob/1e3c330ad2878c1cb9d3bef49f599a02df31a787/src/internal/interpreter/execution_engine.rs#L113", "Resurgence Implementation of Call on GitHub (link)")]
How stashing is implemented is merely an implementation detail, it is perfectly valid to also create a non-recursive setup with a seperate data structure so long as the code can not access said data structure.
=== ExtCall
=== Ret
=== Mov
=== Cpy
=== Ref
=== StackPush
=== StackPop
=== StackMov
=== Add
=== Sub
=== Mul
=== Div
=== Mod
=== Equal
=== NotEqual
=== Greater
=== Less
=== GreaterEqual
=== LessEqual

== Representation of Instructions in Binary
For later
