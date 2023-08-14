# Elysian

A DSL, IR, transpiler, interpreter, and toolkit for building signed distance fields and converting them into concrete assets.

## Distance Fields 101

* A signed distance field is a function that takes a position, and produces the distance to a surface.
* For instance, the SDF for a point is simply eucliean length, a.k.a. vector length
* These can be combined in various ways and used to create a model.
* Field functions aren't just limited to distance; they can also produce arbitrary supplemental data,
  such as gradient, normal, UV, and color.
* This makes SDFs very powerful, but they require art, math, and programming skill to use effectively.
* Generally relegated to secondary use cases in computer graphics
  * Ambient Occlusion
  * Soft Shadows
  * 3D texture generation
* "Full SDF" approaches are mostly limited to Shadertoy and the demoscene

## Project History

Elysian has been quietly in development for some time now:

### GL analytical raytracer

Adjacent; not SDF, but seeded an interest in analytical geometry and spacewarping

### Godot prototype

* Built as a side project during the development of Qodot
* Shader-based virtual machine that read bytecode via texture and interpreted it in-shader
* Poor performance
* Too limited
* Too engine-specific
* Established base idea: Extensible, composable SDFs

### Bevy prototype

* Generated textual WGSL from an AST of Rust types
* Worked, but abstractions not powerful enough to make textual intermediary robust
* Bevy's WGSL subset is also too domain-specific

### Imperative rust-gpu prototype

Worked, but language support too limited to realize the necessary abstractions

### Type-Level Functional Programming prototype

* Built a huge type-level FP library called t-funk to power it
* Solidified the computational model using category theory
* Worked, but not idiomatic to Rust, took forever to compile, and broke rust-gpu

### Functional Programming rust-gpu prototype

* Similar ideas to previous, but restricted to Fn trait objects
* Worked, but language support still too limited to realize the necessary abstractions
* Too domain-specific; forces users to use rust-gpu / SPIR-V, which lags behind GLSL / WGSL in support

### Elysian

* Nuclear solution: Step above domain-specifity and build a compiler
* Write tooling in idiomatic dynamic rust for fast compile times and performance
* Use value-level abstractions to model functional concepts
  * More work, since functionality previously modeled by the Rust type system needs to be implemented manually,
    but checks the boxes for compile time, abstraction strength, and runtime performance
* Output simple code that can compile on lowest-common-denominator targets

## Elysian Is A...

* Domain-Specific Language
  * For analytically defining implicit geometry
  * Using signed distance fields, and other arbitrary field functions
* Intermediate representation
  * Which can be transpiled into arbitrary output formats
* Set of backends that turn IR into arbitrary concrete output
* Rust library encompassing all of the above
* Set of tool programs exposing the above for general use

## Elysian Is Not A...

* Renderer
* Image editor
* Modeling package
* Shading language

It has some crossover with these domains, but is a higher-order solution for generating the associated data instead of creating it directly.
To put it another way, Elysian is the foundation you might build any of the above on top of.

## Elysian is applicable in...

* Graphics
    * Images
    * Meshes
  * Rendering
    * Raymarching

* Collision Detection
  * Look up various properties of the nearest point of a surface
    * Distance, normal, tangent, etc.

* Field function analysis
  * Bounding error
  * Decompose fields into lower dimensions to visualize and analyze their behaviour
    * i.e. Bounding behaviour under raymarching
    * MRI-style slicing of 3D fields

## Features

### Declarative

* Describe what your geometry is, not how it should be constructed
* Write expressions to wire relations between field data and parameters
  * Parametrize over time to create arbitrary animations

### Procedural

* All primitives compile down to functions,
  which can be invoked directly or used to drive an abstraction
  * ex. elysian-image takes a field, evaluates it internally, and produces an image
* Powerful composability
  * ex. Using a voronoi diagram as a space-partitioning acceleration structure,
    using distance-from-camera to pick field LODs based on a heuristic, etc.

### Analytical

* Infinite resolution
  * Define procedural recursive shapes that continue to produce detail at the micro level
* No need for mesh sculpting or other similar abstractions over discrete data

### Lossless

* Information is retained in the transition from artist imagination to data

### General

* Supports multiple vector spaces and field functions per primitive
  * Promotes reusability
  * Reduces cognitive overhead in cases where one abstraction can comfortably cover multiple domains
    * i.e. Ball vs Circle2D and Sphere3D

### Higher-order

* Build scenes using a single consistent language, and transpile them into final assets
* Reduces integration cost by encapsulating domains as backends
  * i.e. Rendering abstractions
    * Leave conversion to scene / acceleration structures / meshes / textures to a backend integration
* Interpret the same data in different ways
  * Scalability
    * Raymarch on powerful GPUs
    * Discretize to meshes and textures on weaker hardware
    * Freely mix and match based on your own heuristics
  * Shareability
    * Convert the same data into different domains that can work in concert
    * ex. Transpiling a scene to a shader for rendering,
      and corresponding statically-compiled Rust code for collision detection

### Extensible

* Define your own primitives and backends via rust library

## Common Concerns

### You have to hand-code your entire scene when using SDFs, like Shadertoy

* Elysian's purpose is to solve this problem through abstraction
* It even has a Shadertoy backend that can convert Elysian AST into Shadertoy-compatible GLSL!

### Calculating extra data like normals is expensive

* This is only true if using an iterative solution like local differencing
* Supplementary fields like normals, tangents, UVs and so forth can also be generated
  using simple analytical functions
* Elysian solves for this by allowing each primitive to output
  an arbitrary set of analytical field data (comparable to position / normal / tangent / uv / uv2 / color channels in a mesh),
  with iterative solutions made available as modifiers for cases where analytical data is unavailable.

### 3D SDFs are slow because you have to raymarch them

* This is only partly true; iterative raymarching is expensive, but is not the only way to render SDFs.
* SDFs can be converted into GPU-friendly meshes and textures for better performance scaling,
  either statically (i.e. at the cost of per-fragment dynamism), or dynamically via compute shader.
* Raymarching performance can also be ameliorated via domain-specific optimizations,
  such as splitting an SDF scene into separately-rasterized primitives to improve GPU parallelism
* In addition, specific SDFs could be accelerated on a case-by-case basis by providing an
  analytical ray-shape intersection function.

### Generated meshes are prone to artifacts

* Traditional marching cube / marching tetrahedron approaches are artifact-prone,
  but more modern solutions like contour fitting reduce this problem significantly.
 
### There's no way to rig or animate an SDF-based model

* Partly true: There's no popular tooling or standardization around rigged SDF animation.
* However, since SDF models are just compositions of functions, which can themselves be parametric,
  the structure is there to build animation systems that are significantly more powerful than the
  weighted bone deformation abstractions of triangle-based modeling.
* At a basic level, you have things like the bend modifier - useful for simple smooth deformation
* For more advanced cases, traditional skeletal rigging data could be imported and used to drive SDF-native deformation,
  with an analytical 'weight field' replacing per-vertex bone weights.

## Relevant Projects

### Media Molecule's Dreams

* The precedent-setter for SDFs as a first-class building block
* Uses fully compute-driven software rendering with no rasterizer

### Adobe 3D modeler

* SDF-based traditional modeling package
* Discretizes into a mesh for rendering
* Uses a hybrid editing approach that can modify either
  the underlying SDF or its generated triangle representation,
  then fold it back into the SDF after the computation is complete
* Able to import meshes and convert them into SDFs
  * Likely via some wrapper that preserves the mesh and
    evaluates a distance-to-nearest-point-on-triangle,
    or a conversion pass that converts mesh triangles / lines / points into
    a composition of the equivalent SDFs

### VL.Fuse

* Visual live-programming library for use with the vvvv environment

### MagicaCSG

* Standalone SDF editor and renderer

### MudBun

* Unity plugin for SDF modeling in-editor
* Supports raymarching, compute-based mesh generation, custom shapes, automatic rigging

### msdfgen

* Multi-channel SDF generator
* Generates 2D valve-style pseudo-SDF textures for sharp glyph rendering
* (Pseudo-SDF textures store one field per color channel, then combine them in-shader to eliminate undesired isosurface rounding)

### diagrams

* Declarative Haskell DSL for composing vector geometry
* Similar goal, different scope and scale
