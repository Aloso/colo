use std::env;
use std::process::Command;

fn main() {
    let output = Command::new("cargo")
        .args(&["tree", "--quiet", "--color", "never"])
        .output()
        .expect("failed to execute `cargo tree`");

    let output = String::from_utf8(output.stdout).expect("Output isn't valid UTF-8");

    let deps: String = output
        .lines()
        .skip(1) // skip first line because it contains user directory
        .flat_map(|s| s.chars().chain(Some('\n')))
        .collect();
    let deps = deps.trim_end();

    let mut path = env::var("OUT_DIR").unwrap();
    path.push_str("/dependencies.txt");
    std::fs::write(&path, deps).expect("could not write to file `dependencies.txt`");
}
