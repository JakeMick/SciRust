#[link(name = "SciRust",
       vers = "0.1",
       url  = "https://github.com/eholk/SciRust")];

#[comment = "A Scientific Computing Library for Rust"];
#[crate_type = "lib"];

#[path="matrix/matrix.rs"]
pub mod matrix;

extern mod std;