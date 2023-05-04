fn main() {
    read_file_example();
}

/**
* This function reads the lines.txt file in the root of the project and prints it.
* Toying around with it is a good practice for Iterators in Rust.
**/
fn read_file_example() -> () {
    if let Ok(file) = std::fs::read_to_string("lines.txt") {
        file
            .lines()
            .enumerate()
            .filter(| (idx, _) | idx % 2 == 0)
            .skip(2)
            .take(2)
            .for_each(| (_, x) | println!("{}", x));
    }
}
