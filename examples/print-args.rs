#! /usr/bin/env ssrs

fn main() -> () {
    for arg in std::env::args() {
        println!("{}", arg);
    }
}
