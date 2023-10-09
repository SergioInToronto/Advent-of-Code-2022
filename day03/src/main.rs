use std::fs;


fn find_common_item(items1: &str, items2: &str) -> char {
    for x in items1.chars() {
        if items2.contains(x) {
            // println!("Common item: {}", &x);
            return x.clone();
        }
    }
    return '_';
}


fn main() {
    const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let contents = fs::read_to_string("input.txt").expect("Failed to read file");

    let mut total_priorities = 0;
    for row in contents.trim_end().split("\n") {
        let midpoint: usize = &row.len() / 2;
        // println!("Midpoint: {} Items: {}", midpoint, &row.len());
        let items1 = &row[..midpoint];
        let items2 = &row[midpoint..];
        
        let common_item = find_common_item(&items1, &items2);
        let item_priority = LETTERS.find(common_item).unwrap() + 1;
        // println!("Priority: {}", &item_priority);
        total_priorities = total_priorities + item_priority;
        
        // println!("Items1: {}, items2: {}", &items1, &items2);
    }

    println!("Total priorities: {}", total_priorities);
}
