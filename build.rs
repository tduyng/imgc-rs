use cargo_metadata::{MetadataCommand};
use std::fs::File;
use std::io::Write;
use std::path::Path;

// workaround from: https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
#[allow(unused_macros)]
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    // Run `cargo metadata` to gather project metadata
    let metadata = MetadataCommand::new()
        .exec()
        .expect("Failed to execute cargo metadata");

    // Create or overwrite the versions.rs file
    let out_dir = std::env::var("OUT_DIR")
        .expect("OUT_DIR not set by Cargo");
    let dest_path = Path::new(&out_dir).join("versions.rs");
    let mut file = File::create(dest_path)
        .expect("Failed to create versions.rs");

    // Write the header for the generated file
    writeln!(file, "// Automatically generated file. Do not edit.\n")
        .expect("Failed to write to versions.rs");
    writeln!(file, "/// Dependency version information generated via build.rs")
        .expect("Failed to write to versions.rs");

    // Generate a constant table with dependency names and versions
    writeln!(file, "pub const DEPENDENCIES: &[(&str, &str)] = &[")
        .expect("Failed to write to versions.rs");
    for package in metadata.packages {
        //p!("{:?}", package);
        writeln!(file, "    (\"{}\", \"{}\"),", package.name, package.version)
            .expect("Failed to write to versions.rs");
    }
    writeln!(file, "];").expect("Failed to write to versions.rs");

    println!("cargo:rerun-if-changed=build.rs");
}