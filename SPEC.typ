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
- Instructions
  - Instruction set
  - Instruction behavior
  - Representation in binary files
- Memory layout
  - Global registers
  - Stack
  - Call stack
  - Stackframes
- Function API
  - ResurgenceState

This spec defines all of these. The reference implementation of Resurgence can be found on *#link("https://github.com/Resurgence-VM-Development/Resurgence", "GitHub")*, although it should be known that the reference implementation also adds a C FFI, forward facing API, and code generation API, all of whihc are not defined in the spec and thus should be considered implementation unique.

= Instructions
The following defines the spec regardding Resurgence instructions, their behaviors, and their representation in binary.

== Instructions and Behaviors
For later

== Representation of Instructions in Binary
For later
