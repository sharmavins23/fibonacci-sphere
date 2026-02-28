# Fibonacci Sphere (Godot-Rust)

This project is a simple component inspired by a visual from
[Sebastian Lague](https://www.youtube.com/@SebastianLague)'s YouTube video
[Coding Adventure: Procedural Moons and Planets](https://youtu.be/lctXaT9pxA0?si=KbSs0yUBbpt0uBQJ).

Specifically, at the start, when discussing algorithmic generation of points on
a sphere, Sebastian discusses about (and shows a rendered example of) a
[Fibonacci Lattice](https://observablehq.com/@meetamit/fibonacci-lattices),
which is "a visually and mathematically elegant method of distributing points on
a unit square, disk, or sphere".

## Scope

(This section was written before I started the project!)

Every rendition I've seen of Fibonacci Lattices were accompanied with gorgeous
video-related visuals, and so, I had to make one. As such, the scope for this
project is simple:

- Create a renderer for a 3-dimensional (spherical) Fibonacci lattice;
- When new points are added, interpolate their positions to give the effect of
  points 'appearing' or 'bouncing' into view;
- Use the arrow keys to increase or decrease the number of points (to a clamped
  minimum/maximum!)

Brownie points for:

- The sphere being visually engaging.
- The sphere (and its points) rotating in space, with this rotation speed being
  controllable up to a point (and also clamped).

## Development

As with the prior project, to enable hot reloading:

```zsh
cargo install watchexec-cli
watchexec -c -w src -e rs "cargo build"
```
