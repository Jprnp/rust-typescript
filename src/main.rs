fn main() {
    let foo: Vec<u32> = vec![1, 2, 3].iter().map(| x | { x + 1 }).collect();
    println!("{:?}", foo);
}
