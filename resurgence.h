/*

resurgence.h - Resurgence C Library Interface

This interface provides a way for non-Rust projects to incorporate a
Resurgence Virtual Machine. The C interface is fairly similar in usage to the
Rust interface, but has been modified to be easier to use in non-Rust projects.

Caveats of the Resurgence C interface:
- Instances of Interpreters and CodeHolders MUST NOT be used on multiple
  threads / processes simultaneously. Otherwise, undefined behavior will occur.
- If a function's comment says it "consumes" an input, it means that the
  ownership of that value is transferred and MUST NOT be used again by the
  calling / host application, or else undefined behavior will occur. If a value
  has been consumed, do not attempt to free it, because it will be freed
  automatically when it is no longer used.

---

Copyright (c) dynafide 2022, All Rights Reserved.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

 */

#ifndef __RESURGENCE_H
#define __RESURGENCE_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct RVMInterpreter;
struct RVMCodeHolder;
struct RVMState;

/**
 * Creates an instance of an Interpreter. If successful, returns a pointer to an
 * Interpreter instance. If this fails, it returns a null pointer. Consumes the
 * CodeHolder.
 */
struct RVMInterpreter* rvm_interpreter_new(struct RVMCodeHolder* ch);

/**
 * Registers a function to be used for ExtCalls. Must be called before invoking
 * rvm_interpreter_resolve_imports. If successful, returns 0; If this fails, it
 * returns 1.
 */
uint8_t rvm_interpreter_register_function(
  struct RVMInterpreter* inter,
  uint8_t (*callback)(struct RVMState*),
  const char* name_char
);

/**
 * Attempts to resolve all imports requested by the CodeHolder. If this
 * succeeds, returns 0; If this fails, it returns 1.
 */
uint8_t rvm_interpreter_resolve_imports(struct RVMInterpreter* inter);

/**
 * Execute a function in the virtual machine interpreter. If successful, returns
 * 0. If this fails, it returns 1.
 */
uint8_t rvm_interpreter_execute_function(
  struct RVMInterpreter* inter,
  const char* name_char
);

/**
 * Free and destroy an Interpreter instance. Consumes the Interpreter.
 */
void rvm_interpreter_destroy(struct RVMInterpreter* inter);

/**
 * Creates an empty CodeHolder instance.
 */
struct RVMCodeHolder* rvm_codeholder_new();

/**
 * Free and destroy a CodeHolder instance. Consumes the CodeHolder.
 */
void rvm_codeholder_destroy(struct RVMCodeHolder* ch);

/**
 * Reads a bytecode file and creates a CodeHolder instance.
 */
struct RVMCodeHolder* rvm_read_bytecode_file(const char* path_char);

/**
 * Retrieve an integer from an RVMState.
 */
uint8_t rvm_state_get_integer(struct RVMState* state, int64_t* out_value);

/**
 * Retrieve a floating point number from an RVMState.
 */
uint8_t rvm_state_get_float(struct RVMState* state, double* out_value);

/**
 * Retrieve a string from an RVMState.
 * NOTE: The string output by this function is allocated by Resurgence and MUST
 * NOT be freed directly by the calling application. Free this string using
 * rvm_string_free!
 */
uint8_t rvm_state_get_string(struct RVMState* state, char** out_value);

/**
 * Retrieve a boolean value from an RVMState. Outputs the value as a uint8_t.
 * False = 0, True = 1.
 * Please note, this does not return the actual value, but the success state.
 * If this function fails, it returns 1. Otherwise, it always returns 0.
 */
uint8_t rvm_state_get_bool(struct RVMState* state, uint8_t* out_value);

/**
 * Push an integer value onto the stack of an RVMState instance.
 * Returns 0 if successful. Returns 1 if it fails.
 */
uint8_t rvm_state_push_integer(struct RVMState* state, int64_t value);

/**
 * Push a floating point value onto the stack of an RVMState instance.
 * Returns 0 if successful. Returns 1 if it fails.
 */
uint8_t rvm_state_push_float(struct RVMState* state, double value);

/**
 * Push a string value onto the stack of an RVMState instance.
 * The string is not consumed and must be manually freed through normal means.
 * Returns 0 if successful. Returns 1 if it fails.
 */
uint8_t rvm_state_push_string(struct RVMState* state, const char* value);

/**
 * Push a boolean value onto the stack of an RVMState instance.
 * Returns 0 if successful. Returns 1 if it fails.
 */
uint8_t rvm_state_push_bool(struct RVMState* state, uint8_t value);

/**
 * Frees a string (char*) that was allocated by another Resurgence function.
 */
void rvm_string_free(char* str);

#ifdef __cplusplus
} // extern "C" {
#endif

#endif // #ifndef __RESURGENCE_H
