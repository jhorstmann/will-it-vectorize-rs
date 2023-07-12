# Will-It-Vectorize

Small code samples that show the power or weaknesses of the Rust/LLVM autovectorizer.

## SIMD (Single Instruction Multiple Data)

```
    +---+---+---+---+
    | 1 | 2 | 3 | 4 |
    +---+---+---+---+

    +---+---+---+---+
 +  | 1 | 2 | 3 | 4 |
    +---+---+---+---+

    +---+---+---+---+
 =  | 2 | 4 | 6 | 8 |
    +---+---+---+---+
```

A single `+` instruction works on all lanes of a vector register at the same time.

## Auto-vectorization

Is a technique implemented in the compiler that can transform scalar loops working
on single elements into code that uses simd instructions to process multiple elements
at the same time. This can often improve the performance by a factor equal to the number
of vector lanes.

Whether this transform worked can be verified by inspecting the resulting assembly code.
A useful tool for showing the assembly code for a function is
[`cargo-show-asm`](https://crates.io/crates/cargo-show-asm).

For example to show the assembly of the first example in this crate:

```shell
$ cargo +nightly asm --native --lib arithmetic_add

will_it_vectorize::arithmetic::arithmetic_add:

	.cfi_startproc
	vmovups ymm0, ymmword ptr [rdi]
	vaddps ymm0, ymm0, ymmword ptr [rsi]
	vmovups ymmword ptr [rdx], ymm0

	vmovups ymm0, ymmword ptr [rdi + 32]
	vaddps ymm0, ymm0, ymmword ptr [rsi + 32]
	vmovups ymmword ptr [rdx + 32], ymm0

	vmovups ymm0, ymmword ptr [rdi + 64]
	vaddps ymm0, ymm0, ymmword ptr [rsi + 64]
	vmovups ymmword ptr [rdx + 64], ymm0

	vmovups ymm0, ymmword ptr [rdi + 96]
	vaddps ymm0, ymm0, ymmword ptr [rsi + 96]
	vmovups ymmword ptr [rdx + 96], ymm0

	vzeroupper
	ret
```

We can see 4 occurrences of the `vaddps` instruction, which operates
on avx `ymm` registers, which contain 8 f32 elements each.

## Instruction sets

### x86_64

 - SSE, SSE2: guaranteed to be supported, 128 bit wide registers (~2000)
 - SSE3, SSSE3: horizontal additions, byte shuffles (since P4/Core2, ~2006)
 - SSE4.1: blend, conversions, rounding (later Core2 versions, ~2008)
 - SSE4.2: string specific functions (Core i, ~2009)
 - AVX: registers extended to 256 bits (~2011)
 - AVX2: 256 bit wide integer operations, gather (~2013)
 - AVX512: 512 bit wide operations, masks can be used for all operations (~2013 xeon phi, ~2017, skylake and later server, icelake/tigerlake client)
  
### AArch64/Arm

 - Neon: 128 bit wide operations (~2005 Cortex-A8)
 - SVE: variable vector length up to 2048 bits (~2018 fugaku supercomputer, some smart phones)

### RISC-V
 - "V" Extension: variable vector length (specification v1.0 2021, no hardware yet AFAIK)

## "Vector" vs. "Packed" model

### Vector model

 - Example: Add two arbitrary length arrays
 - Very low loop overhead
 - Good fit for simple arithmetic

 ### Packed model

 - Example: Add two arrays by processing 4/8/16/32 elements at a time
 - Reduced loop overhead, might require additional loop unrolling for best performance
 - Might require a scalar loop for remainder elements (or use masked operations)
 - Good fit if the problem requires shuffling data across vector lanes
