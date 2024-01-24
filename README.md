# mappers

[![License](https://img.shields.io/github/license/ScaleWeather/mappers)](https://choosealicense.com/licenses/apache-2.0/)
[![Crates.io](https://img.shields.io/crates/v/mappers)](https://crates.io/crates/mappers)
[![dependency status](https://deps.rs/repo/github/ScaleWeather/mappers/status.svg)](https://deps.rs/repo/github/ScaleWeather/mappers)
[![docs.rs](https://img.shields.io/docsrs/mappers)](https://docs.rs/mappers)

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ScaleWeather/mappers/linux.yml?branch=main&label=Build%20on%20Ubuntu)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ScaleWeather/mappers/windows.yml?branch=main&label=Build%20on%20Windows)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ScaleWeather/mappers/macos.yml?branch=main&label=Build%20on%20MacOS)

Pure Rust geographical projections library. Similar to `Proj` in basic functionality but allows for a use in concurrent contexts.

Projections' implementations closely follow algorithms and instructions provided in: [Map projections: A working manual (John P. Snyder, 1987)](https://pubs.er.usgs.gov/publication/pp1395)

**This crate in very early stages of development. If you are interested in contributing do not hesitate to contact me on Github.**

## Why this crate exists?

There is already a well-established, production-ready and battle-tested library for geographical projections and transformations - [`Proj`](https://proj.org/), which has great [high-level bindings in Rust](https://crates.io/crates/proj). And you should probably use it in most cases instead of implementing your own functions or using this crate. `Proj` is even used to test `mappers` accuracy.

However, because `Proj` is not written in Rust its usage is not straightforward in concurrent context. `Proj` also supports mostly Linux and its installation can be a real hassle on different targets (and `Proj` bindings do not support other targets anyway).

`mappers` addresses those two issues by implementing the most commonly used geographical projections in pure Rust. But it is not (yet) thoroughly tested for precision and edge-cases. `mappers` possibly also has a slightly better performance than `Proj` because it is so much less complex. But it mainly allows for use with multiple threads (processes, tasks...) and on at least Tier 1 targets (only building Ubuntu, Windows and MacOS is tested).

So `mappers` should be used only when comprehensiveness (and probably correctness) of `Proj` is less important than a need to calculate geographical projections on non-linux targets or in concurrent contexts.

## Usage example

We can project the geographical coordinates to cartographic coordinates on a map with specified projection as follows:

```rust
// First, we define the projection

// We use LCC with reference longitude centered on France
// parallels set for Europe and WGS84 ellipsoid
let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;

// Second, we define the coordinates of Mount Blanc
let (lon, lat) = (6.8651, 45.8326);

// Project the coordinates
let (x, y) = lcc.project(lon, lat)?;

// And print the result
println!("x: {}, y: {}", x, y); // x: 364836.4407792019, y: 5421073.726335758
```

We can also inversely project the cartographic coordinates to geographical coordinates:

```rust
// We again start with defining the projection
let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;

// We take the previously projected coordinates
let (x, y) = (364836.4407792019, 5421073.726335758);

// Inversely project the coordinates
let (lon, lat) = lcc.inverse_project(x, y)?;

// And print the result
println!("lon: {}, lat: {}", lon, lat); // lon: 6.8651, lat: 45.83260000001716
```

Some projections are mathematically exactly invertible, and technically geographical coordinates projected and inverse projected should be identical. However, in practice limitations of floating-point arithmetics will introduce some errors along the way, as shown in the example above.

## Multithreading

For projecting multiple coordinates at once, the crate provides `_parallel`
functions that are available in a (default) `multithreading` feature. These functions
use `rayon` crate to parallelize the projection process. They are provided
mainly for convenience, as they are not much different than calling
`.par_iter()` on a slice of coordinates and mapping the projection function over it.

```rust
let lcc = LambertConformalConic::new(2.0, 0.0, 30.0, 60.0, Ellipsoid::WGS84)?;

// Parallel functions use slices of tuples as input and output
let geographic_coordinates = vec![(6.8651, 45.8326); 10];

let map_coordinates = lcc.project_parallel(&geographic_coordinates)?;
let inversed_coordinates = lcc.inverse_project_parallel(&map_coordinates)?;
```
