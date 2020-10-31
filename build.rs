use std::process::Command;

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=Cargo.lock");

    let output = Command::new("cargo")
        .args(&["tree"])
        .output()
        .expect("failed to execute `cargo tree`");

    let output = String::from_utf8(output.stdout).expect("Output isn't valid UTF-8");

    let deps: String = output
        .lines()
        .skip(1) // skip first line because it contains user directory
        .flat_map(|s| s.chars().chain(Some('\n')))
        .collect();
    let deps = deps.trim_end();

    std::fs::write("./dependencies.txt", deps).expect("could not write to file `dependencies.txt`");
}
