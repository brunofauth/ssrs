#! /usr/bin/env -S python3 ssrs.py --

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
