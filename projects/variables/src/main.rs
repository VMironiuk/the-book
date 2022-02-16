#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]

fn main() {
    variables_sample();
    constants_sample();
    shadowing_sample();
}

fn variables_sample() {
    // Example of an immutable variable
    let x = 5;
    // error: cannot assign twice to immutable variable
    // x = 6;

    // Example of mutable variable
    let mut y = 5;
    // Ok
    y = 6;
}

fn constants_sample() {
    // Constants are always immutable, has a program's lifetime (and because of it the type annotation must be specified)
    // Constant names are upperceased by convention
    const LOCAL_CONST: i32 = 0;
}

fn shadowing_sample() {
    // Shadowing of variable with the same type
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
    }

    // Shadowing of variable with different type
    let spaces = "    ";
    let spaces = spaces.len();

    // Shadowing of mutable variable by immutable
    let mut y = 5;
    let y = 6;

    // Shadowning of immutable variable by mutable
    let z = 5;
    let mut z = 6;
}