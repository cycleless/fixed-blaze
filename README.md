<div align="center">
  <img src="static/fixed-blaze.jpg" alt="fixed-blaze logo" width="150" />
  <h1>fixed-blaze</h1>
  <p><b>Zero-cost fixed-point arithmetic with convergent rounding and SIMD‑first design</b></p>
  <p><i>A Cycleless Open Source Release</i></p>
</div>

---

## The Philosophy (Why `fixed` sucks)

Most fixed-point libraries are either **mathematically broken** or **embarrassingly slow**.

- **Broken:** They truncate towards zero, accumulating a systematic negative bias. After 10,000 multiplications your PID controller drifts off a cliff.
- **Slow:** They rely on `typenum` for scale parameters – bloated compile times, poor optimisation, and zero SIMD awareness.
- **Bloatware:** Software division loops for `/` and `sqrt` when hardware can do it in 5 cycles via Newton‑Raphson.

**`fixed-blaze`** is the antidote. It's a `no_std`, **branchless**, **const‑generic** fixed‑point engine designed for hard real‑time and embedded systems. We mapped the algebra of convergent rounding directly onto 64‑bit ALUs and SIMD registers. No bias. No dynamic dispatch. No fucking around.

---

## Core Features

- **Const Generics, Not `typenum`**  
  `Fixed<i32, 12>` instead of `FixedI32<UInt<UInt<…>>>`. Cleaner errors, faster compiles, and LLVM can constant‑fold everything.

- **Banker’s Rounding (Round‑to‑Nearest, Ties to Even)**  
  Zero systematic error – critical for long accumulation chains (filters, neural networks, physics).

- **Deterministic, Fixed‑Iteration Algorithms**  
  `div` and `sqrt` use Newton‑Raphson with a constant number of steps. No jitter. Hard real‑time safe.

- **SIMD‑First Design**  
  The core `mul_blaze` and `add_unchecked` are written to auto‑vectorise. On NEON / AVX2 you process 4–8 values at once – the `fixed` crate can’t do that.

- **Dual‑Path API**  
  - `checked_*` methods for safety.  
  - `*_unchecked` methods for maximum speed (no overflow checks).  

- **`#![no_std]` & Zero‑Alloc**  
  Runs on bare metal from Cortex‑M0 to RISC‑V. No heap, no panic handlers unless you opt in.

---

## How We Cheat the Benchmark

| Feature | `fixed` crate | `fixed-blaze` |
|:--------|:--------------|:---------------|
| Scale parameter | `typenum` (runtime overhead) | `const FRAC: u32` (zero‑cost) |
| Rounding | Truncation (negative bias) | Banker’s rounding (unbiased) |
| Division | Software bit‑loop (O(N²) cycles) | Newton‑Raphson (fixed, fast) |
| SIMD | None (scalar only) | Auto‑vectorised + NEON/AVX2 intrinsics |
| Real‑time | Jitter due to variable‑length loops | Hard deterministic, fixed iterations |

---

## Frictionless Quickstart

```rust
#![no_std]

use fixed_blaze::Fixed;

// Q16.16 format (16 integer, 16 fractional bits)
let a = Fixed::<i32, 16>::from_bits(1 << 16); // 1.0
let b = Fixed::<i32, 16>::from_bits(2 << 16); // 2.0

let c = a * b;        // 2.0, using convergent rounding
let d = c / a;        // 2.0, via deterministic Newton‑Raphson

assert_eq!(d.to_bits(), 2 << 16);

// SIMD‑friendly batch processing (auto‑vectorised)
let mut arr = [a, b, c];
arr.iter_mut().for_each(|x| *x = x.mul_blaze_unchecked(d));
```

---

## License

**GNU AGPL v3.**  
*Deterministic math belongs to everyone.*

<div align="center">
  <img src="static/cycleless_100.png" alt="Cycleless Logo" width="60" />
  <p><b>CYCLELESS RnD</b><br><i>Zero Cycles. Absolute Precision.</i></p>
</div>
