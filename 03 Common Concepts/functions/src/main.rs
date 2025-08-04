fn function_before() {
    println!("This function appears before main");
}

fn main() {
    function_before();
    println!("Hello, world!");

    // value "changes" to u16 because it's used in a function that has a u16 parameter
    let value = 5;
    function_after(value);

    let four = {
        let x = 3;
        x + 1 // by not having a semicolon, this returns the value
    };

    let void = {
        let x = 3;
        x + 1; // now this returns a unit (no value). Essentially it does nothing
    };
    /*
        Multi line comment
        Just like in C++
    */

    let five = five();
    if five == also_five() { // Doesn't need parenthesis but it does need brackets!
        println!("The value is five!");
    } else {
        println!("The value is NOT five!");
    }

    let wants_four = true;
    let number = if wants_four {4} else {5}; // Basically a ternary operator

    let mut count = 0;
    let result = loop {
        count += 1;
        if count >= 10 {
            break count*2 // semicolon not needed??
        }
    }; // This needs a semicolon because we break with a value

    println!("The loop returned {result}");

    'outer_loop: loop {
        'inner_loop: loop {
            break 'outer_loop; // breaks the outer loop
        } // Again, semicolon is not needed
    } // We don't need a semicolon because we don't set a value

    while true { // Warning: Denote infinite loops with `loop { ... }`
        break;
    }

    let elements = [1, 2, 3, 4, 5];
    for element in elements {
        println!("Element is {element}");
    }

    for number in 1..6 {
        println!("The number is {number}");
    }
}

fn function_after(x: u16) {
    println!("The value is {x}");
}

fn five () -> i8 { // The 5 gets turned to an i8
    5 // no semicolon, so it returns 5
}

fn also_five () -> i8 {
    return 5; // Rust tells you return is unnecessary...
}