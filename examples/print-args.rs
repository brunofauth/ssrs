#! /usr/bin/env -S python3 ssrs.py --

fn main() -> () {
    for arg in std::env::args() {
        println!("{}", arg);
    }
}
