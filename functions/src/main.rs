fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    let x = five();
    println!("the value of x is {x}");

    let x = plus_one(x);
    println!("the value of x is {x}");

}

fn another_function(x: i32, unit_label: char) {
    println!("The measurement is : {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}