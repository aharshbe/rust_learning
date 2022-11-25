fn main() {

    println!("Printing numbers");
    
    // one can indicate numeric types as a suffix
    let x = 5_000_000_000u128;
    println!("0 The value of x is {x}");
    
    // or as a prefix
    let x: u128 = x + 1_000;
    println!("1 The value of x is {x}");
    {
        let x = x * 2;
        println!("2 The value of x in the inner scope is {x}");
    } 
    println!("3 The value of x is {x}");
    
    // calling the types function from below
    types();
}

// Creating a new function to practice types and functions
fn types() {
    println!("Starting types function");

    let fnumber: f64 = 98668833.97; // f64 is default
    println!("{fnumber}");

    let bcheck: bool = true;
    println!("{bcheck}");

    let emoji: char = '😻';
    println!("{emoji}");

    // tuples
    let types: (i32, f64, char, bool) = (500, 6.4, emoji, bcheck);
    let (x, y, z, f) = types;
    println!("{:?}",types);
    println!("The value of y is {y}");
    println!("4 Another way to access a tuple value {}", types.2);

    //arrays
    let a: [i64; 5] = [1, 2, 3, 4, 5];
    let f = [1, 3, 4];
    println!("{:?}", a);
    println!("{}, {}",a[0],a[3]);
    println!("{:?}, {}",f, f[2]);

    let t2: () = ();
    println!("{:?}", t2);

}
