# Raytracer

A ray tracing-based 3d image renderer written in rust.

# Compiling

With [rust](https://rustup.rs) installed, run the following command to compile and run the raytracer:
```
cargo run --release -- <scene_file>
```

There is also a scene generator for a pyramid of spheres, which can be run with:
```
cargo run --release --bin sphere_pyramid -- <pyramid_height>
```
This will generate the file `scenes/scene_pyramid.xml`.
