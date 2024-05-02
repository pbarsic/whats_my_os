use std::env::consts::{OS, FAMILY, ARCH};

fn print_all_keys() {
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_CFG_") {
            println!("{}: {}", key, value);
        } };
}

fn main() {
    println!("OS = {}", OS);
    println!("FAMILY = {}", FAMILY);
    println!("ARCH = {}", ARCH);

    print_all_keys();

    if cfg!(target_os = "linux") {
        println!("target_os = linux");
    } else if cfg!(target_os = "macos") {
        println!("target_os = macos");
    } else if cfg!(target_os = "windows") {
        println!("target_os = windows. I am so sorry for you.");
    } else {
        panic!("unsupported OS");
    }

}
