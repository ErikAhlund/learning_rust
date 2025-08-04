fn main() {
    // We can mutate using mut
    let mut x = 1;
    println!("The value of x is: {x}");
    x = 2;
    println!("The value of x is: {x}");

    // We can shadow immutable variables
    let x = x;
    let x = x + 1;
    {
        let x = x*2;
        println!("The value of x is in the inner loop is {x}");
    }
    println!("The value of x is {x}");

    // We can shadow x into a mutbale version of itself
    let mut x = x;
    x = x + 1;
    println!("The value of x is {x}");

    // We can declare the "type" of a number in the rvalue
    let u8_num = 58u8;
    let u16_num = 12345u16;
    let i8_num = -58i8;

    // But it's cleaner to set it manually
    let u8_num: u8 = 58;
    let u16_num: u16 = 12345;
    let i8_num: i8 = -58;

    let float: f32 = 0.4;
    let double: f64 = 0.4;
    let default_double = 0.4;

    let tup = (12u8, 'c', false);

    // Destructuring
    let (x, y, z) = tup;
    let twelve = tup.0;

    // unit is an empty tuple, which is essentially void
    let unit = ();

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    let five_threes = [3; 5];

    let zero = array[2] - five_threes[0];
}