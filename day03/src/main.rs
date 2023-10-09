use std::fs;


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed to read file");
    for row in contents.trim_end().split("\n") {
        // TODO
    }
}
