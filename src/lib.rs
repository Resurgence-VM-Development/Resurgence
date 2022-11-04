/*!
# Resurgence
A VM backend library that makes developing interpreters easy. Can be used either as an entire backend, or to create a backend
*/

/*
    Copyright (c) 2022 StandingPad
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
    AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
    OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
    SOFTWARE.
*/
pub(crate) mod objects;
pub use objects::codeholder::CodeHolder;
pub(crate) use objects::constant;

pub(crate) mod internal;
pub use internal::execution_engine::ExecutionEngine;
pub use internal::interpreter::Interpreter;

pub(crate) mod ext_func;
pub use ext_func::resurgence_state::ResurgenceState;

pub mod bytecode;

pub mod codegen;

pub mod ffi;
