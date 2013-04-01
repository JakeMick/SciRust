#[link(name = "SciRust",
       vers = "0.1",
       url  = "https://github.com/eholk/SciRust")];

#[comment = "A Scientific Computing Library for Rust"];
#[crate_type = "lib"];

extern mod std;

#[path="matrix/matrix.rs"]
pub mod matrix;


