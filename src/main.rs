use std::env;

fn add(v: Vec<i32>) {
    println!("Result for add: {}", v.iter().sum::<i32>()); //iterates and sums all elements in array
}
fn divide(v: Vec<i32>) {
    let mut result = 1;
    for value in v {
        result /= value
    }
    println!("Result for division: {}", result)
}
fn multiply(v: Vec<i32>) {
    let mut result = 1;
    for value in v {
        result *= value
    }
    println!("Result for multiplication: {}", result)
}
fn minus(v: Vec<i32>) {
    let mut result = 0; //used for making the calculation happen
    for value in v {
        result -= value
    }
    println!("Result for minus: {}", result);
}
fn get_mode() -> String {
    let mode = std::env::args().nth(1).expect("No mode given!\nGive /a, /d, /mi or /mu as first arg."); //gets the index for the mode.
    return mode;
}
fn arg_vector() -> Vec<i32> {
    let mut args: Vec<String> = std::env::args().collect();
    args.drain(0..2); 
    let mut arg_list: Vec<i32> = Vec::new();
    for arg in args {
        let mut i = 0;
        arg_list.insert(i, arg.trim().parse::<i32>().unwrap()); //trims and parses to be returned in i32
        i += 1;
    }
    return arg_list;
}

fn main() {
    let mode = get_mode();
    if mode.eq("/a") {
        add(arg_vector());
    }
    else if mode.eq("/d") {
        divide(arg_vector());
    }
    else if mode.eq("/mu") {
        multiply(arg_vector());
    }
    else if mode.eq("/mi") {
        minus(arg_vector());
    }
}