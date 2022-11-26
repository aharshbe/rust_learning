
fn main() {
    // exploring functions further
    first_another_function();
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');

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