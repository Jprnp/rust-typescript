fn main() {
    println!("{:?}", practice(vec![1, 2, 3], 1));
    println!("{:?}", practice(vec![1, 2, 3], 3));
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

enum Color {
    Red,
    Green,
    Blue
}

fn print_color(color: Color) {
    match color {
        Color::Blue => println!("blue"),
        Color::Red => println!("red"),
        Color::Green => println!("green")
    };
}

fn multiply(value: Option<usize>) -> Option<usize> {
    // return value.unwrap_or(0) * 5;
    // if let Some(x) = value {
    //     return x * 5;
    // }
    // 0;

    // return value.map(|x| x * 5);
    // if let Some(x) = value {
    //     return Some(x * 5);
    // }
    // return None;
    return Some(value? * 5);
}

fn practice(nums: Vec<usize>, index: usize) -> usize {
    return nums.get(index).unwrap_or(&index) * 5;
}
