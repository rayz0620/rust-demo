fn main() {
    // Comment
    /*
        Block comment
    */
    println!("This month has {} days", 31);

    println!("{1}, this is {0}. {0}, this is {1}", "Alice", "Bob");

    println!("{subject} {verb} {object}", subject="I", verb="have", object="control");

    println!("{number:>width$}", number=0, width=6);

    println!("{number:>width$}", number=100, width=6);
}