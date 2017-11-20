fn main() {
    println!("Demo of ownership in Rust");
    // cloning();
    function_call();
}

fn cloning() {
    let str_1 = String::from("Azalea");
    let str_2 = str_1.clone();

    println!("str_2: {}", str_2);

    // You will get a compile error on this line
    println!("str_1: {}", str_1);
}

fn function_call() {
    let str_1 = String::from("Azalea");
    takes_ownership(str_1);

    // println!("{}", str_1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}