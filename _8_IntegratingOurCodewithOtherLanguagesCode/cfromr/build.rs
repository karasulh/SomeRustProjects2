//If there is build.rs in any Rust project, Rust will assume that before trying to call build, this needs to run.

use cmake::Config;

fn main(){
    let dst = Config::new("libbadmath").build();
    
    println!("cargo:rustc-link-search=native={}",dst.display());
    println!("cargo:rustc-link-lib=static=badmath");
}