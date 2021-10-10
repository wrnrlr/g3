# g3

Rust crate for plane-based projective geometric-algebra for 3D aka the Clifford Algebra with signature ![formula](https://render.githubusercontent.com/render/math?math=P%28\mathbb{R}^{*}_{3,0,1}%29).

## Get Started

This software uses some of Rust experimental feautures like `std_simd` and `fn_traits` so make sure to compile using the nightly release.

```bash
rustup update -- nightly
```

```bash
cargo +nightly build
```

## Awesome Links

* [Siggraph2019 Geometric Algebra](https://www.youtube.com/watch?v=tX4H_ctggYo), Talk explaining the why and a bit of the how of GA.
* [Bivector](https://bivector.net/), community site with more info on geometric algebra including videos, software and a discrord server.
* [Klein](https://www.jeremyong.com/klein/), simular project in C++, the `g3` library models PGA3D in the same way as `Klein.
* [ganja.js](https://github.com/enkimute/ganja.js), JavaScript library for geometric algebra of arbitrary signature.
* [Geometric Algebra - Duality and the Cross Product](https://www.youtube.com/watch?v=RAcyVrMNV5s)

## Status

* Todo 
  * Rename `reverse` methods to `conjugate` for Clifford Conjugate, see [section 1.5](https://observablehq.com/@enkimute/glu-lookat-in-3d-pga)
