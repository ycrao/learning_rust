fn main() {
    print_sum(5, 6);
    add_one(1);
}

fn print_sum(x: i32, y : i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}
