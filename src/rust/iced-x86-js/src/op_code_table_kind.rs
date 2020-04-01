/*
Copyright (C) 2018-2019 de4dot@gmail.com

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

use wasm_bindgen::prelude::*;

// GENERATOR-BEGIN: Enum
// ⚠️This was generated by GENERATOR!🦹‍♂️
/// Opcode table
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[cfg(all(feature = "encoder", feature = "op_code_info"))]
#[cfg_attr(all(not(feature = "exhaustive_enums"), has_non_exhaustive), non_exhaustive)]
pub enum OpCodeTableKind {
	/// Legacy encoding table
	Normal,
	/// `0Fxx` table (legacy, VEX, EVEX)
	T0F,
	/// `0F38xx` table (legacy, VEX, EVEX)
	T0F38,
	/// `0F3Axx` table (legacy, VEX, EVEX)
	T0F3A,
	/// `XOP8` table (XOP)
	XOP8,
	/// `XOP9` table (XOP)
	XOP9,
	/// `XOPA` table (XOP)
	XOPA,
}
// GENERATOR-END: Enum

pub(crate) fn iced_to_op_code_table_kind(value: iced_x86::OpCodeTableKind) -> OpCodeTableKind {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
