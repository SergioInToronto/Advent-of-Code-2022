use std::fs;
use std::collections::HashMap;


fn main() {
    let instructions = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut sizes: HashMap<String, isize> = HashMap::new();
    sizes.insert(String::from("/"), -1); // directories are -1 until we compute their size

    sizes = parse_instructions(&instructions, sizes);
    compute_directory_sizes(sizes);
    
    println!("part1 - todo");
}


fn parse_instructions(instructions: &String, mut sizes: HashMap<String, isize>) -> HashMap<String, isize> {
    let mut current_path: Vec<&str> = Vec::new();
    current_path.push("");
    
    for instruction in instructions.trim_end().split("\n") {
        // println!("Handling: {}", &instruction);
        if ["$ ls", "$ cd /"].contains(&instruction) {
            // No useful information here. Continue.
        } else if instruction == "$ cd .." {
            current_path.pop();
        } else if &instruction[0..5] == "$ cd " {
            let dir_name = &instruction[6..];
            current_path.push(dir_name);
            continue;
        } else {
            let mut parts = instruction.split(" ");
            let size_or_dir = parts.next().unwrap();
            let size: isize = if size_or_dir == "dir" { -1 } else { size_or_dir.parse::<isize>().unwrap() };
            let filename = parts.next().unwrap();
            let path = current_path.join("/") + "/" + filename;
            sizes.insert(path.to_string(), size);
        }
    }
    return sizes;
}


fn compute_directory_sizes(mut sizes: HashMap<String, isize>) {
    let sizes_copy = &sizes.clone(); // We'll read this one while writing to the other
    let mut total: isize;
    // This is O(n^2). If performance mattered we'd revisit this code first
    for (d_path, d_size) in sizes_copy {
        if *d_size != -1 {
            continue;
        }
        println!("Computing directory size for {}...", d_path);
        total = 0;
        for (path, size) in &sizes {
            // TODO: YOU ARE HERE
        }
    }
}
