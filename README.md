# mazegen -- Rust Maze Generation Tools

This is meant to be a fun project, playing with the algorithms from the book
_Mazes for Programmers_ by Jamis Buck.  It is also an opportunity to experiment
with using the [Molt](https://github.com/wduquette/molt) interpreter to wrap Rust APIs as
a convenience and development aid, and to look for rough spots in Molt's Rust API.

The Rust API provides:

* A Grid type for rectangular mazes
* Several maze algorithms (more to come)
* Rendering of mazes into text and PNG

The Molt API provides:

* A binding to the Grid type sufficient for writing basic maze algorithms
* A binding to the Rust `thread_rng()` random number generator
* The ability to create and render mazes
* The smallest beginnings of a binding to the [**image**](https://crates.io/crates/image) crate.
