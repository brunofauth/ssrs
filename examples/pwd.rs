#! /usr/bin/env ssrs

fn main() -> () {
    println!(
        "{}",
        std::env::current_dir()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    );
}
