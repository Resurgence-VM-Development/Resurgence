#pragma once
#include <cstdint>

/*
    * REFERENCE
    * size - u8: the amount of registers
    * reg - u16: register
    * reg-ref - u8: determines the type of the second register
    * {
    *   0 - Const Pool
    *   1 - Global
    *   2 - Last Stack Frame
    *   3 - Local
    * }
    * jmp-options - u8: affects how jmp works
    * {
    *   0 - in current function
    *   1 - with current module
    * }
    * mem-options - u8: affects how free works
    * {
    *   0 - Call stack
    *   1 - Module stack
    * }
    * ref-type - u8: affects how a value can be read
    * {
    *   0 - nomal 
    *   1 - dereferenced
    * }
*/
enum class ByteCodeEnum : std::uint8_t
{
    /* --------------------------------- Scopes --------------------------------- */
    alloc,          //* size/index, mem-options
    free,           //* blocks, mem-options

    /* -------------------------- Register instructions ------------------------- */
    mov,            //* reg1, reg2,   ref-type1, ref-type2, reg-ref1, reg-ref2, NULL8
    cpy,            //* reg1, reg2,   ref-type1, ref-type2, reg-ref1, reg-ref2, NULL8
    ref,            //* reg1, reg2,   ref-type1, ref-type2, reg-ref1, reg-ref2, NULL8

    /* --------------------------- Array instructions --------------------------- */
    movElm,         //* reg1, reg2, index, reg-ref1, reg-ref2, NULL8
    cpyElm,         //* reg1, reg2, index, reg-ref1, reg-ref2, NULL8
    refElm,         //* reg1, reg2, index, reg-ref1, reg-ref2, NULL8

    /* ---------------------------------- jump ---------------------------------- */
    jmp,            //* index, jmp-options
    
    /* ---------------------------------- Math ---------------------------------- */
    add,            //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    sub,            //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    mul,            //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    div,            //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    pow,            //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3

    icr,            //* reg1, reg-ref1
    dcr,            //* reg1, reg-ref1

    /* -------------------------- Conditional Operators ------------------------- */
    eq,             //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    eq_not,         //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    gr,             //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    less,           //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    gr_eq,          //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
    less_eq,        //* reg1, reg2, reg3, reg-ref1, reg-ref2, reg-ref3
};