// ================================================================================================
// CH 07-01 PACKAGES AND CRATES
// ================================================================================================
// projects split into multiple modules (and files)
// a package can contain multiple binary crates and an optional library crate
// workspaces for very large programmes

// CRATE: smallest amount of code that the Rust compiler considers at a time
// binary crate:  programs that compile to an executable that can be run
//                must have a function called main, defines programme
// library crate: don't compile and don't have a main function
//                define functionality shared with multiple projects
// most often, a crate refers to a library crate
// crate root is a source file that the compiler starts from
// a package is a bundle of one or more crates that provides functionality
// the cargo.toml file describes how to build crates

fn main() {
    print!("Intro to crates and packages");
}
