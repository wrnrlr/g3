# g3

Rust crate for plane-based projective geometric-algebra for 3D aka the Clifford Algebra with signature P(R*<sub>3,0,1</sub>).


## API

* Invariants:
  * `Point`
  * `Line`, `Branch` and `IdealLine`
  * `Plane`
* Variants:
  * `Direction`
  * `Translator`
  * `Rotor`
  * `Motor`

### Meet Operation `^`

### Join Operator `&`

### Inner Product `|`

### Geometric Product `*`

`a*b = a|b + a^b`

### Sandwich Product `a(b)`

### Dual Operator `!`

## Get Started

This software uses some of Rust experimental feautures like `std_simd` and `fn_traits` so make sure to compile using the nightly release.

```bash
rustup update -- nightly
```

```bash
cargo +nightly build
```


## Run example


```
cargo +nightly run --example elements --features="mirror"
```

## Awesome Links

* [Bivector](https://bivector.net/), community site with more info on geometric algebra including videos, software and a discrord server.
* Videos:
  * [Siggraph2019 Geometric Algebra](https://www.youtube.com/watch?v=tX4H_ctggYo), Talk explaining the why and a bit of the how of GA.
* Software:
  * [Klein](https://www.jeremyong.com/klein/) is a similar project in C++ as was the inspiration for g3.
  * [ganja.js](https://github.com/enkimute/ganja.js), JavaScript library for geometric algebra of arbitrary signature.
* Articles:
  * [Geometric Algebra - Duality and the Cross Product](https://www.youtube.com/watch?v=RAcyVrMNV5s)
  * [Geometric Algebra Tutorial](https://geometricalgebratutorial.com)
  * https://mattferraro.dev/posts/geometric-algebra
* [Cheat Sheet](https://enki.ws/ganja.js/examples/coffeeshop.html#V3k2baxG2&fullscreen)

## Status

This software is very much still a work in progress.

* Todo
  * Integrate with the baryon graphics library
    * WebGPU shaders to draw basic elements: // WebGPU tutorial: https://github.com/jack1232/webgpu11
  * Rename `reverse` methods to `conjugate` for Clifford Conjugate, see [section 1.5](https://observablehq.com/@enkimute/glu-lookat-in-3d-pga)
