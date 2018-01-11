Game of Life 3D
===============

A 3D version of the well-known Conway's Game of Life, written in Rust.

![Screenshot](screenshot.png)


Status
------

* The game take place in a cube
* Each cube represents a cell
* The cell color represents its age
* The user can turn around the cube
* The user can select the game speed
* The user can select the game size


Future goals
------------

I will probably use this project to test some libraries or aspects of the Rust
language, so it might become overbloat, but nobody should care about it since
it does nothing useful yet anyway.

A few ideas:

- [ ] A way to reconfigure the game in real time (such as a Lua shell)
- [ ] Change cells colors accordings to the sound card output
- [ ] Compile to WebAssembly to render in a browser
- [ ] Add some visual effects/edit some GLSL
- [ ] Use tokio in some ways (why not?)
