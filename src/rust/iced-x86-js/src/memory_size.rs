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
/// Size of a memory reference
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum MemorySize {
	/// Unknown size or the instruction doesn't reference any memory (eg. `LEA`)
	Unknown,
	/// Memory location contains a `u8`
	UInt8,
	/// Memory location contains a `u16`
	UInt16,
	/// Memory location contains a `u32`
	UInt32,
	/// Memory location contains a `u52`
	UInt52,
	/// Memory location contains a `u64`
	UInt64,
	/// Memory location contains a `u128`
	UInt128,
	/// Memory location contains a `u256`
	UInt256,
	/// Memory location contains a `u512`
	UInt512,
	/// Memory location contains a `i8`
	Int8,
	/// Memory location contains a `i16`
	Int16,
	/// Memory location contains a `i32`
	Int32,
	/// Memory location contains a `i64`
	Int64,
	/// Memory location contains a `i128`
	Int128,
	/// Memory location contains a `i256`
	Int256,
	/// Memory location contains a `i512`
	Int512,
	/// Memory location contains a seg:ptr pair, `u16` (offset) + `u16` (segment/selector)
	SegPtr16,
	/// Memory location contains a seg:ptr pair, `u32` (offset) + `u16` (segment/selector)
	SegPtr32,
	/// Memory location contains a seg:ptr pair, `u64` (offset) + `u16` (segment/selector)
	SegPtr64,
	/// Memory location contains a 16-bit offset (`JMP/CALL WORD PTR [mem]`)
	WordOffset,
	/// Memory location contains a 32-bit offset (`JMP/CALL DWORD PTR [mem]`)
	DwordOffset,
	/// Memory location contains a 64-bit offset (`JMP/CALL QWORD PTR [mem]`)
	QwordOffset,
	/// Memory location contains two `u16`s (16-bit `BOUND`)
	Bound16_WordWord,
	/// Memory location contains two `u32`s (32-bit `BOUND`)
	Bound32_DwordDword,
	/// 32-bit `BNDMOV`, 2 x `u32`
	Bnd32,
	/// 64-bit `BNDMOV`, 2 x `u64`
	Bnd64,
	/// Memory location contains a 16-bit limit and a 32-bit address (eg. `LGDTW`, `LGDTD`)
	Fword6,
	/// Memory location contains a 16-bit limit and a 64-bit address (eg. `LGDTQ`)
	Fword10,
	/// Memory location contains a `f16`
	Float16,
	/// Memory location contains a `f32`
	Float32,
	/// Memory location contains a `f64`
	Float64,
	/// Memory location contains a `f80`
	Float80,
	/// Memory location contains a `f128`
	Float128,
	/// Memory location contains a `bfloat16`
	BFloat16,
	/// Memory location contains a 14-byte FPU environment (16-bit `FLDENV`/`FSTENV`)
	FpuEnv14,
	/// Memory location contains a 28-byte FPU environment (32/64-bit `FLDENV`/`FSTENV`)
	FpuEnv28,
	/// Memory location contains a 94-byte FPU environment (16-bit `FSAVE`/`FRSTOR`)
	FpuState94,
	/// Memory location contains a 108-byte FPU environment (32/64-bit `FSAVE`/`FRSTOR`)
	FpuState108,
	/// Memory location contains 512-bytes of `FXSAVE`/`FXRSTOR` data
	Fxsave_512Byte,
	/// Memory location contains 512-bytes of `FXSAVE64`/`FXRSTOR64` data
	Fxsave64_512Byte,
	/// 32-bit `XSAVE` area
	Xsave,
	/// 64-bit `XSAVE` area
	Xsave64,
	/// Memory location contains a 10-byte `bcd` value (`FBLD`/`FBSTP`)
	Bcd,
	/// 16 bit location: 2 x `u8`
	Packed16_UInt8,
	/// 16 bit location: 2 x `i8`
	Packed16_Int8,
	/// 32 bit location: 4 x `u8`
	Packed32_UInt8,
	/// 32 bit location: 4 x `i8`
	Packed32_Int8,
	/// 32 bit location: 2 x `u16`
	Packed32_UInt16,
	/// 32 bit location: 2 x `i16`
	Packed32_Int16,
	/// 32 bit location: 2 x `bfloat16`
	Packed32_BFloat16,
	/// 64-bit location: 8 x `u8`
	Packed64_UInt8,
	/// 64-bit location: 8 x `i8`
	Packed64_Int8,
	/// 64-bit location: 4 x `u16`
	Packed64_UInt16,
	/// 64-bit location: 4 x `i16`
	Packed64_Int16,
	/// 64-bit location: 2 x `u32`
	Packed64_UInt32,
	/// 64-bit location: 2 x `i32`
	Packed64_Int32,
	/// 64-bit location: 4 x `f16`
	Packed64_Float16,
	/// 64-bit location: 2 x `f32`
	Packed64_Float32,
	/// 128 bit location: 16 x `u8`
	Packed128_UInt8,
	/// 128 bit location: 16 x `i8`
	Packed128_Int8,
	/// 128 bit location: 8 x `u16`
	Packed128_UInt16,
	/// 128 bit location: 8 x `i16`
	Packed128_Int16,
	/// 128 bit location: 4 x `u32`
	Packed128_UInt32,
	/// 128 bit location: 4 x `i32`
	Packed128_Int32,
	/// 128 bit location: 2 x `u52`
	Packed128_UInt52,
	/// 128 bit location: 2 x `u64`
	Packed128_UInt64,
	/// 128 bit location: 2 x `i64`
	Packed128_Int64,
	/// 128 bit location: 8 x `f16`
	Packed128_Float16,
	/// 128 bit location: 4 x `f32`
	Packed128_Float32,
	/// 128 bit location: 2 x `f64`
	Packed128_Float64,
	/// 128 bit location: 4 x (2 x `bfloat16`)
	Packed128_2xBFloat16,
	/// 256 bit location: 32 x `u8`
	Packed256_UInt8,
	/// 256 bit location: 32 x `i8`
	Packed256_Int8,
	/// 256 bit location: 16 x `u16`
	Packed256_UInt16,
	/// 256 bit location: 16 x `i16`
	Packed256_Int16,
	/// 256 bit location: 8 x `u32`
	Packed256_UInt32,
	/// 256 bit location: 8 x `i32`
	Packed256_Int32,
	/// 256 bit location: 4 x `u52`
	Packed256_UInt52,
	/// 256 bit location: 4 x `u64`
	Packed256_UInt64,
	/// 256 bit location: 4 x `i64`
	Packed256_Int64,
	/// 256 bit location: 2 x `u128`
	Packed256_UInt128,
	/// 256 bit location: 2 x `i128`
	Packed256_Int128,
	/// 256 bit location: 16 x `f16`
	Packed256_Float16,
	/// 256 bit location: 8 x `f32`
	Packed256_Float32,
	/// 256 bit location: 4 x `f64`
	Packed256_Float64,
	/// 256 bit location: 2 x `f128`
	Packed256_Float128,
	/// 256 bit location: 8 x (2 x `bfloat16`)
	Packed256_2xBFloat16,
	/// 512 bit location: 64 x `u8`
	Packed512_UInt8,
	/// 512 bit location: 64 x `i8`
	Packed512_Int8,
	/// 512 bit location: 32 x `u16`
	Packed512_UInt16,
	/// 512 bit location: 32 x `i16`
	Packed512_Int16,
	/// 512 bit location: 16 x `u32`
	Packed512_UInt32,
	/// 512 bit location: 16 x `i32`
	Packed512_Int32,
	/// 512 bit location: 8 x `u52`
	Packed512_UInt52,
	/// 512 bit location: 8 x `u64`
	Packed512_UInt64,
	/// 512 bit location: 8 x `i64`
	Packed512_Int64,
	/// 256 bit location: 4 x `u128`
	Packed512_UInt128,
	/// 512 bit location: 16 x `f32`
	Packed512_Float32,
	/// 512 bit location: 8 x `f64`
	Packed512_Float64,
	/// 512 bit location: 16 x (2 x `bfloat16`)
	Packed512_2xBFloat16,
	/// Broadcast `u32` to 64 bits
	Broadcast64_UInt32,
	/// Broadcast `i32` to 64 bits
	Broadcast64_Int32,
	/// Broadcast `f32` to 64 bits
	Broadcast64_Float32,
	/// Broadcast `u32` to 128 bits
	Broadcast128_UInt32,
	/// Broadcast `i32` to 128 bits
	Broadcast128_Int32,
	/// Broadcast `u52` to 128 bits
	Broadcast128_UInt52,
	/// Broadcast `u64` to 128 bits
	Broadcast128_UInt64,
	/// Broadcast `i64` to 128 bits
	Broadcast128_Int64,
	/// Broadcast `f32` to 128 bits
	Broadcast128_Float32,
	/// Broadcast `f64` to 128 bits
	Broadcast128_Float64,
	/// Broadcast `u32` to 256 bits
	Broadcast256_UInt32,
	/// Broadcast `i32` to 256 bits
	Broadcast256_Int32,
	/// Broadcast `u52` to 256 bits
	Broadcast256_UInt52,
	/// Broadcast `u64` to 256 bits
	Broadcast256_UInt64,
	/// Broadcast `i64` to 256 bits
	Broadcast256_Int64,
	/// Broadcast `f32` to 256 bits
	Broadcast256_Float32,
	/// Broadcast `f64` to 256 bits
	Broadcast256_Float64,
	/// Broadcast `u32` to 512 bits
	Broadcast512_UInt32,
	/// Broadcast `i32` to 512 bits
	Broadcast512_Int32,
	/// Broadcast `u52` to 512 bits
	Broadcast512_UInt52,
	/// Broadcast `u64` to 512 bits
	Broadcast512_UInt64,
	/// Broadcast `i64` to 512 bits
	Broadcast512_Int64,
	/// Broadcast `f32` to 512 bits
	Broadcast512_Float32,
	/// Broadcast `f64` to 512 bits
	Broadcast512_Float64,
	/// Broadcast 2 x `i16` to 128 bits
	Broadcast128_2xInt16,
	/// Broadcast 2 x `i16` to 256 bits
	Broadcast256_2xInt16,
	/// Broadcast 2 x `i16` to 512 bits
	Broadcast512_2xInt16,
	/// Broadcast 2 x `u32` to 128 bits
	Broadcast128_2xUInt32,
	/// Broadcast 2 x `u32` to 256 bits
	Broadcast256_2xUInt32,
	/// Broadcast 2 x `u32` to 512 bits
	Broadcast512_2xUInt32,
	/// Broadcast 2 x `i32` to 128 bits
	Broadcast128_2xInt32,
	/// Broadcast 2 x `i32` to 256 bits
	Broadcast256_2xInt32,
	/// Broadcast 2 x `i32` to 512 bits
	Broadcast512_2xInt32,
	/// Broadcast 2 x `bfloat16` to 128 bits
	Broadcast128_2xBFloat16,
	/// Broadcast 2 x `bfloat16` to 256 bits
	Broadcast256_2xBFloat16,
	/// Broadcast 2 x `bfloat16` to 512 bits
	Broadcast512_2xBFloat16,
}
// GENERATOR-END: Enum

pub(crate) fn iced_to_memory_size(value: iced_x86::MemorySize) -> MemorySize {
	// Safe, the enums are exactly identical
	unsafe { std::mem::transmute(value as u8) }
}
