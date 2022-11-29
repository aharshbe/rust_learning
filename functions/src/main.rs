
fn main() {
    // exploring functions further
    first_another_function();
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1 // becomes an expression without a semicolon, otherwise statement
    };

    println!("The value of y is: {:?}", y);


    let r = five(10);
    println!("the return value of the function, 'five' is: {r}");

    let ad = add_two(1, 2);
    println!("the return value of the function, 'add_two' is: {ad}");
}

fn first_another_function(){
    println!("this first another function should go before the other.");
}


fn another_function (x: i32) {
    let x = x+1;
    println!("{x}");
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions that return values

fn five(x: i32) -> i32 {
    x + 5
}

fn add_two(x: i32, y: i32) -> i32 {
    x + y
}