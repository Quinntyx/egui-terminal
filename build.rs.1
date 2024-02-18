#![feature(iter_intersperse)]
#![allow(unused_imports)]

use std::fs::{File, OpenOptions, self};
use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;

fn main () {
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .open("Cargo.toml")
        .unwrap();

    let mut other_file = OpenOptions::new()
        .read(true)
        .open("git_deps.toml")
        .unwrap();

    let mut git_deps = String::default();

    other_file.read_to_string(&mut git_deps).unwrap();

    writeln!(file, "{}", git_deps).unwrap();

    // let processed: String = BufReader::new(file).lines()
    //     .map(|line| line.unwrap().trim_start_matches("#!").trim_start_matches(" ").to_owned())
    //     .intersperse_with(|| String::from("\n"))
    //     .collect();

    // if Path::new("Cargo.toml.old").exists() {
    //     println!("cargo:warning=Cargo.toml exists, not overriding it");
    // } else {
    //     fs::copy("Cargo.toml", "Cargo.toml.old").unwrap();
    //     println!("cargo:warn=Moving original Cargo.toml to Cargo.toml.old; Before publishing crate, make sure to restore it!");
    // }

    // fs::remove_file("Cargo.toml").unwrap();

    // let mut new_file = File::create("Cargo.toml").unwrap();

    // write!(new_file, "{processed}").unwrap();
}
