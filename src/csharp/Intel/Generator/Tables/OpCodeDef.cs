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

using Generator.Enums;
using Generator.Enums.Encoder;

namespace Generator.Tables {
	enum OpCodeW : byte {
		None,
		W0,
		W1,
		WIG,
		WIG32,
	}

	enum OpCodeL : byte {
		None,
		L0,
		L1,
		LIG,
		LZ,
		L128,
		L256,
		L512,
	}

	struct OpCodeDef {
		public EncodingKind Encoding;
		public MandatoryPrefix MandatoryPrefix;
		public OpCodeW WBit;
		public OpCodeL LBit;
		public OpCodeTableKind Table;
		public uint OpCode;
		public int OpCodeLength;
		public sbyte GroupIndex;
		public sbyte RmGroupIndex;
		public CodeSize OperandSize;
		public CodeSize AddressSize;
		public ParsedOpCodeFlags Flags;

		public static OpCodeDef CreateDefault(EncodingKind encoding) =>
			new OpCodeDef {
				Encoding = encoding,
				MandatoryPrefix = MandatoryPrefix.None,
				WBit = OpCodeW.None,
				LBit = OpCodeL.None,
				Table = OpCodeTableKind.Normal,
				OpCode = 0,
				GroupIndex = -1,
				RmGroupIndex = -1,
				OperandSize = 0,
				AddressSize = 0,
				Flags = ParsedOpCodeFlags.None,
			};
	}
}
