# g3

This is a neat library for 3D computer graphics based on geometric algebra for the Rust programing language.
Specifically it implements the plane-based projective geometric-algebra aka. the Clifford Algebra with signature P(R*<sub>3,0,1</sub>).
At first this may sound like a strange and esoteric idea to use in
contrast to the linear algebra you might already be familiar with.
However, one will find it to be a more intuitive and powerful formalism to describe operations for three-dimensional euclidean/flat space.


Let's break down what we mean by plane-based, projective and geometric-algebra in reverse order.

### Geometric algebra
Of the history of mathematics it is often said that it where the Arabs or Indians that discovered the number zero.
It might be hard for today's people with all their modern technology to fully appreciate the difficulties of solving
quadratic equations using roman numerals. Harder still to fathom that 
like the mathematicians of old, without the number zero, one is missing some numbers that can make life easier,
and one is to take this quite literally.

Most know about Complex numbers as the non-real solution of equation *x = √-1*.
Less known is the non-real solution for *x = √1* called the Hyperbolic numbers
and the non-real solution for *x = √0* called the Dual numbers.

Together the complex numbers *p*, the hyperbolic numbers *q*
and the dual numbers *r* describe a space *R<sub>p,q,r</sub>*.

### (Hyper)planes


### Projections
The first thing to realise is that to represent all possible transformations of 3D space one needs an extra 4th dimension.


## Elements

This library exports the following basic elements:
* Plane: the basis vector from which all other elements are build
* Line: the intersection of two planes \
  There are two special cases for lines:
  * Branch, a line through the origin
  * Horizon, a line infinitely far away
* Point: the intersection of three planes
* Rotor: a rotation
* Translator: a translation
* Motor: a combination of a Rotor and a Translator

## Geometric Operations

### Meet Operation `^`

### Join Operator `&`

### Inner Product `|`

### Geometric Product `*`

`a*b = a|b + a^b`

### Sandwich Product `a(b)`

`a(b) = a*b*a⁻¹`

### Dual Operator `!`

## Get Started

TODO

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
* [Rational trigonometry via projective geometric algebra](https://arxiv.org/pdf/1401.2371.pdf)
* [Alan Macdonald videos](https://www.youtube.com/playlist?list=PLLvlxwbzkr7igd6bL7959WWE7XInCCevt)
* [Geometric Calculus Talk](https://www.youtube.com/watch?v=ItGlUbFBFfc)
* [Lecture playlist](https://www.youtube.com/playlist?list=PLv6uM2DOOtPY28m4RE_oGxyrf6w-Erq5b)
* [TensorFlow-based framework for Geometric Algebra](https://tfgap.warlock.ai/#/)

https://enki.ws/ganja.js/examples/coffeeshop.html#ydDtaGu0a
