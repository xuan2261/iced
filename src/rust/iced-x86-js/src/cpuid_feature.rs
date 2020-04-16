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
/// `CPUID` feature flags
#[wasm_bindgen]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum CpuidFeature {
	/// 8086 or later
	INTEL8086 = 0,
	/// 8086 only
	INTEL8086_ONLY = 1,
	/// 80186 or later
	INTEL186 = 2,
	/// 80286 or later
	INTEL286 = 3,
	/// 80286 only
	INTEL286_ONLY = 4,
	/// 80386 or later
	INTEL386 = 5,
	/// 80386 only
	INTEL386_ONLY = 6,
	/// 80386 A0-B0 stepping only (`XBTS`, `IBTS` instructions)
	INTEL386_A0_ONLY = 7,
	/// Intel486 or later
	INTEL486 = 8,
	/// Intel486 A stepping only (`CMPXCHG`)
	INTEL486_A_ONLY = 9,
	/// 80386 and Intel486 only
	INTEL386_486_ONLY = 10,
	/// IA-64
	IA64 = 11,
	/// CPUID.80000001H:EDX.LM\[bit 29\]
	X64 = 12,
	/// CPUID.(EAX=07H, ECX=0H):EBX.ADX\[bit 19\]
	ADX = 13,
	/// CPUID.01H:ECX.AES\[bit 25\]
	AES = 14,
	/// CPUID.01H:ECX.AVX\[bit 28\]
	AVX = 15,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX2\[bit 5\]
	AVX2 = 16,
	/// CPUID.(EAX=07H, ECX=0H):EDX.AVX512_4FMAPS\[bit 3\]
	AVX512_4FMAPS = 17,
	/// CPUID.(EAX=07H, ECX=0H):EDX.AVX512_4VNNIW\[bit 2\]
	AVX512_4VNNIW = 18,
	/// CPUID.(EAX=07H, ECX=1H):EAX\[bit 5\]
	AVX512_BF16 = 19,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_BITALG\[bit 12\]
	AVX512_BITALG = 20,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512_IFMA\[bit 21\]
	AVX512_IFMA = 21,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VBMI\[bit 1\]
	AVX512_VBMI = 22,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VBMI2\[bit 6\]
	AVX512_VBMI2 = 23,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VNNI\[bit 11\]
	AVX512_VNNI = 24,
	/// CPUID.(EAX=07H, ECX=0H):EDX\[bit 08\]
	AVX512_VP2INTERSECT = 25,
	/// CPUID.(EAX=07H, ECX=0H):ECX.AVX512_VPOPCNTDQ\[bit 14\]
	AVX512_VPOPCNTDQ = 26,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512BW\[bit 30\]
	AVX512BW = 27,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512CD\[bit 28\]
	AVX512CD = 28,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512DQ\[bit 17\]
	AVX512DQ = 29,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512ER\[bit 27\]
	AVX512ER = 30,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512F\[bit 16\]
	AVX512F = 31,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512PF\[bit 26\]
	AVX512PF = 32,
	/// CPUID.(EAX=07H, ECX=0H):EBX.AVX512VL\[bit 31\]
	AVX512VL = 33,
	/// CPUID.(EAX=07H, ECX=0H):EBX.BMI1\[bit 3\]
	BMI1 = 34,
	/// CPUID.(EAX=07H, ECX=0H):EBX.BMI2\[bit 8\]
	BMI2 = 35,
	/// CPUID.(EAX=07H, ECX=0H):EDX.CET_IBT\[bit 20\]
	CET_IBT = 36,
	/// CPUID.(EAX=07H, ECX=0H):ECX.CET_SS\[bit 7\]
	CET_SS = 37,
	/// `CL1INVMB` instruction (Intel SCC = Single-Chip Computer)
	CL1INVMB = 38,
	/// CPUID.(EAX=07H, ECX=0H):ECX.CLDEMOTE\[bit 25\]
	CLDEMOTE = 39,
	/// CPUID.(EAX=07H, ECX=0H):EBX.CLFLUSHOPT\[bit 23\]
	CLFLUSHOPT = 40,
	/// CPUID.01H:EDX.CLFSH\[bit 19\]
	CLFSH = 41,
	/// CPUID.(EAX=07H, ECX=0H):EBX.CLWB\[bit 24\]
	CLWB = 42,
	/// CPUID.80000008H:EBX.CLZERO\[bit 0\]
	CLZERO = 43,
	/// CPUID.01H:EDX.CMOV\[bit 15\]
	CMOV = 44,
	/// CPUID.01H:ECX.CMPXCHG16B\[bit 13\]
	CMPXCHG16B = 45,
	/// `RFLAGS.ID` can be toggled
	CPUID = 46,
	/// CPUID.01H:EDX.CX8\[bit 8\]
	CX8 = 47,
	/// CPUID.80000001H:EDX.3DNOW\[bit 31\]
	D3NOW = 48,
	/// CPUID.80000001H:EDX.3DNOWEXT\[bit 30\]
	D3NOWEXT = 49,
	/// CPUID.(EAX=12H, ECX=0H):EAX.OSS\[bit 5\]
	ENCLV = 50,
	/// CPUID.(EAX=07H, ECX=0H):ECX\[bit 29\]
	ENQCMD = 51,
	/// CPUID.01H:ECX.F16C\[bit 29\]
	F16C = 52,
	/// CPUID.01H:ECX.FMA\[bit 12\]
	FMA = 53,
	/// CPUID.80000001H:ECX.FMA4\[bit 16\]
	FMA4 = 54,
	/// 8087 or later (CPUID.01H:EDX.FPU\[bit 0\])
	FPU = 55,
	/// 80287 or later
	FPU287 = 56,
	/// 80287XL only
	FPU287XL_ONLY = 57,
	/// 80387 or later
	FPU387 = 58,
	/// 80387SL only
	FPU387SL_ONLY = 59,
	/// CPUID.(EAX=07H, ECX=0H):EBX.FSGSBASE\[bit 0\]
	FSGSBASE = 60,
	/// CPUID.01H:EDX.FXSR\[bit 24\]
	FXSR = 61,
	/// AMD Geode LX/GX CPU
	GEODE = 62,
	/// CPUID.(EAX=07H, ECX=0H):ECX.GFNI\[bit 8\]
	GFNI = 63,
	/// CPUID.(EAX=07H, ECX=0H):EBX.HLE\[bit 4\]
	HLE = 64,
	/// [`HLE`] or [`RTM`]
	///
	/// [`HLE`]: enum.CpuidFeature.html#variant.HLE
	/// [`RTM`]: enum.CpuidFeature.html#variant.RTM
	HLE_or_RTM = 65,
	/// [`VMX`] and IA32_VMX_EPT_VPID_CAP\[bit 20\]
	///
	/// [`VMX`]: enum.CpuidFeature.html#variant.VMX
	INVEPT = 66,
	/// CPUID.(EAX=07H, ECX=0H):EBX.INVPCID\[bit 10\]
	INVPCID = 67,
	/// [`VMX`] and IA32_VMX_EPT_VPID_CAP\[bit 32\]
	///
	/// [`VMX`]: enum.CpuidFeature.html#variant.VMX
	INVVPID = 68,
	/// CPUID.80000001H:ECX.LWP\[bit 15\]
	LWP = 69,
	/// CPUID.80000001H:ECX.LZCNT\[bit 5\]
	LZCNT = 70,
	/// CPUID.80000008H:EBX.MCOMMIT\[bit 8\]
	MCOMMIT = 71,
	/// CPUID.01H:EDX.MMX\[bit 23\]
	MMX = 72,
	/// CPUID.01H:ECX.MONITOR\[bit 3\]
	MONITOR = 73,
	/// CPUID.80000001H:ECX.MONITORX\[bit 29\]
	MONITORX = 74,
	/// CPUID.01H:ECX.MOVBE\[bit 22\]
	MOVBE = 75,
	/// CPUID.(EAX=07H, ECX=0H):ECX.MOVDIR64B\[bit 28\]
	MOVDIR64B = 76,
	/// CPUID.(EAX=07H, ECX=0H):ECX.MOVDIRI\[bit 27\]
	MOVDIRI = 77,
	/// CPUID.(EAX=07H, ECX=0H):EBX.MPX\[bit 14\]
	MPX = 78,
	/// CPUID.01H:EDX.MSR\[bit 5\]
	MSR = 79,
	/// Multi-byte nops (`0F1F /0`): CPUID.01H.EAX\[Bits 11:8\] = 0110B or 1111B
	MULTIBYTENOP = 80,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.ACE\[Bits 7:6\] = 11B (\[6\] = exists, \[7\] = enabled)
	PADLOCK_ACE = 81,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.PHE\[Bits 11:10\] = 11B (\[10\] = exists, \[11\] = enabled)
	PADLOCK_PHE = 82,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.PMM\[Bits 13:12\] = 11B (\[12\] = exists, \[13\] = enabled)
	PADLOCK_PMM = 83,
	/// CPUID.0C0000000H:EAX >= 0C0000001H AND CPUID.0C0000001H:EDX.RNG\[Bits 3:2\] = 11B (\[2\] = exists, \[3\] = enabled)
	PADLOCK_RNG = 84,
	/// `PAUSE` instruction (Pentium 4 or later)
	PAUSE = 85,
	/// CPUID.01H:ECX.PCLMULQDQ\[bit 1\]
	PCLMULQDQ = 86,
	/// CPUID.(EAX=07H, ECX=0H):EBX.PCOMMIT\[bit 22\]
	PCOMMIT = 87,
	/// CPUID.(EAX=07H, ECX=0H):EDX.PCONFIG\[bit 18\]
	PCONFIG = 88,
	/// CPUID.(EAX=07H, ECX=0H):ECX.PKU\[bit 3\]
	PKU = 89,
	/// CPUID.01H:ECX.POPCNT\[bit 23\]
	POPCNT = 90,
	/// CPUID.80000001H:ECX.PREFETCHW\[bit 8\]
	PREFETCHW = 91,
	/// CPUID.(EAX=07H, ECX=0H):ECX.PREFETCHWT1\[bit 0\]
	PREFETCHWT1 = 92,
	/// CPUID.(EAX=14H, ECX=0H):EBX.PTWRITE\[bit 4\]
	PTWRITE = 93,
	/// CPUID.(EAX=07H, ECX=0H):ECX.RDPID\[bit 22\]
	RDPID = 94,
	/// `RDPMC` instruction (Pentium MMX or later, or Pentium Pro or later)
	RDPMC = 95,
	/// CPUID.80000008H:EBX.RDPRU\[bit 4\]
	RDPRU = 96,
	/// CPUID.01H:ECX.RDRAND\[bit 30\]
	RDRAND = 97,
	/// CPUID.(EAX=07H, ECX=0H):EBX.RDSEED\[bit 18\]
	RDSEED = 98,
	/// CPUID.80000001H:EDX.RDTSCP\[bit 27\]
	RDTSCP = 99,
	/// CPUID.(EAX=07H, ECX=0H):EBX.RTM\[bit 11\]
	RTM = 100,
	/// CPUID.01H:EDX.SEP\[bit 11\]
	SEP = 101,
	/// CPUID.(EAX=12H, ECX=0H):EAX.SGX1\[bit 0\]
	SGX1 = 102,
	/// CPUID.(EAX=07H, ECX=0H):EBX.SHA\[bit 29\]
	SHA = 103,
	/// CPUID.80000001H:ECX.SKINIT\[bit 12\]
	SKINIT = 104,
	/// [`SKINIT`] or [`SVML`]
	///
	/// [`SKINIT`]: enum.CpuidFeature.html#variant.SKINIT
	/// [`SVML`]: enum.CpuidFeature.html#variant.SVML
	SKINIT_or_SVML = 105,
	/// CPUID.(EAX=07H, ECX=0H):EBX.SMAP\[bit 20\]
	SMAP = 106,
	/// CPUID.01H:ECX.SMX\[bit 6\]
	SMX = 107,
	/// CPUID.01H:EDX.SSE\[bit 25\]
	SSE = 108,
	/// CPUID.01H:EDX.SSE2\[bit 26\]
	SSE2 = 109,
	/// CPUID.01H:ECX.SSE3\[bit 0\]
	SSE3 = 110,
	/// CPUID.01H:ECX.SSE4_1\[bit 19\]
	SSE4_1 = 111,
	/// CPUID.01H:ECX.SSE4_2\[bit 20\]
	SSE4_2 = 112,
	/// CPUID.80000001H:ECX.SSE4A\[bit 6\]
	SSE4A = 113,
	/// CPUID.01H:ECX.SSSE3\[bit 9\]
	SSSE3 = 114,
	/// CPUID.80000001H:ECX.SVM\[bit 2\]
	SVM = 115,
	/// CPUID.8000000AH:EDX.SVML\[bit 2\]
	SVML = 116,
	/// CPUID.80000001H:EDX.SYSCALL\[bit 11\]
	SYSCALL = 117,
	/// CPUID.80000001H:ECX.TBM\[bit 21\]
	TBM = 118,
	/// CPUID.01H:EDX.TSC\[bit 4\]
	TSC = 119,
	/// CPUID.(EAX=07H, ECX=0H):ECX.VAES\[bit 9\]
	VAES = 120,
	/// CPUID.01H:ECX.VMX\[bit 5\]
	VMX = 121,
	/// CPUID.(EAX=07H, ECX=0H):ECX.VPCLMULQDQ\[bit 10\]
	VPCLMULQDQ = 122,
	/// CPUID.(EAX=07H, ECX=0H):ECX.WAITPKG\[bit 5\]
	WAITPKG = 123,
	/// CPUID.(EAX=80000008H, ECX=0H):EBX.WBNOINVD\[bit 9\]
	WBNOINVD = 124,
	/// CPUID.80000001H:ECX.XOP\[bit 11\]
	XOP = 125,
	/// CPUID.01H:ECX.XSAVE\[bit 26\]
	XSAVE = 126,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVEC\[bit 1\]
	XSAVEC = 127,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVEOPT\[bit 0\]
	XSAVEOPT = 128,
	/// CPUID.(EAX=0DH, ECX=1H):EAX.XSAVES\[bit 3\]
	XSAVES = 129,
	/// CPUID.8000001FH:EAX.SNP\[bit 4\]
	SNP = 130,
	/// CPUID.(EAX=07H, ECX=0H):EDX.SERIALIZE\[bit 14\]
	SERIALIZE = 131,
	/// CPUID.(EAX=07H, ECX=0H):EDX.TSXLDTRK\[bit 16\]
	TSXLDTRK = 132,
}
// GENERATOR-END: Enum
