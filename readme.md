# g3

Rust crate for plane-based projective geometric-algebra for 3D aka the Clifford Algebra with signature P(R*<sub>3,0,1</sub>).


## API

## Plane

## Point

## Line

## 

### Meet Operation `^`

### Join Operator `&`

### Inner Product `|`

### Geometric Product `*`

`a*b = a|b + a^b`

### Sandwich Product `a(b)`

### Dual Operator `!`

## Get Started

This software uses some of Rust experimental feautures like `fn_traits` so make sure to compile using the nightly release.

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
* [May the forque be with you: Rigid body dynamics in PGA](https://enki.ws/ganja.js/examples/pga_dyn.html)
* [The Geometry of 3DPGA products](https://enki.ws/ganja.js/examples/coffeeshop.html#ydDtaGu0a&fullscreen)
* [PGA Easy](https://enki.ws/PGAEasy/PGAEasy%20GAppendix.html)
* [Automatic differentiation](https://discourse.bivector.net/t/automatic-differentiation/289)
* [Paper with CGA](https://www.researchgate.net/profile/Leo-Dorst/publication/266149530_Total_Least_Squares_Fitting_of_k-Spheres_in_n-D_Euclidean_Space_Using_an_n2-D_Isometric_Representation/links/561431ce08ae4ce3cc6391ac/Total-Least-Squares-Fitting-of-k-Spheres-in-n-D-Euclidean-Space-Using-an-n-2-D-Isometric-Representation.pdf)
* [New Developments in Projective Geometric Algebra](http://terathon.com/gdc21_lengyel.pdf)
* [Flectors](https://projectivegeometricalgebra.org/wiki/index.php?title=Flector)
* [Space & Antispace](https://projectivegeometricalgebra.org/Lengyel-SpaceAntispace.pdf)
