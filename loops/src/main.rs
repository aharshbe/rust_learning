fn main() {
    iterate_loop();

    let list = ["Austin", "Frank", "Janice", "Marge", "Francis", "Locki"];
    let size: usize = list.len();
    iterate_list(list, size);
    interate_list_2(list);
    range_iterate();
}

// basic iterate through a loop using loop
fn iterate_loop() {
    let mut i = 0;
    loop {
        if i < 10 {
            i += 1;
        } else {
            break
        }
        println!("{i}");
    }
    println!("the final value of i is: {i}");
}

// iterate through a list of strings with a defined number of 5
// &str indicates an array of strings -> 6 is the number of elements
// not sure how to indicate an arb number of elements, when searching, looked like enums or structs which are a later chapter
fn iterate_list(list: [&str; 6], size: usize) {
    let mut i = 0;
    let result = loop {
        if i < size {
            println!("{i}: {:?}", list[i]);
            i += 1;
        } else {
            break i
        }
    };
    println!("the result is {result}");
}

// iterate through a list option two
fn interate_list_2(list: [&str; 6]) {
    // like in python
    for i in list {
        println!("the value is: {i}");
    }
}

// traverse a list using a range
fn range_iterate() {
    let start = 1;
    let end = 4;
    // use .rev() to reverse a list
    for number in (start..end).rev() {
        println!("{number}");
    }
    println!("liftoff!");
}
