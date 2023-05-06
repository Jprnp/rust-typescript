fn main() {
    print_file_from_args();
}

/**
* This function reads the lines.txt file in the root of the project and prints it.
* Toying around with it is a good practice for Iterators in Rust.
**/
fn read_file_example() {
    if let Ok(file) = std::fs::read_to_string("lines.txt") {
        file
            .lines()
            .enumerate()
            .filter(|(idx, _)| idx % 2 == 0)
            .skip(2)
            .take(2)
            .for_each(|(_, x)| println!("{}", x));
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

fn print_file_from_args() {
    if std::env::args().nth(1).and_then(print_file_by_name).is_none() {
        println!("File not found!");
    }
}

fn print_file_by_name(file_name: String) -> Option<()> {
    match std::fs::read_to_string(file_name) {
        Ok(file) => { 
            file.lines().for_each(|line| {
                if let Ok(value) = line.parse::<usize>() {
                    println!("{}", value);
                } else {
                    println!("Line not a number");
                }
            }); 
            return Some(()) 
        },
        Err(_) => { return None },
    }
}
