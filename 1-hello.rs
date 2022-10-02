/*
Covers:
1. Main function syntax 
2. Comments
3. Printing
*/

// This is a sinlge line comment
/// Library docs (covered later)
/// Prints hello world on the console
///
/// # Arguments
///    None
///
///  # Examples
/// 
/// 

fn main(){

    println!("hello world");
    format();

}

fn format(){
    // {} will be replaced by any arguments
    print!("Dude its {} degrees outside!", 36);
    // println! changes line
    println!(" this is will printed on the same line");
    println!("This won't");

    // Positional arguments can be used. Specifying an integer inside `{}`
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As named arguments
    println!("{subject} {verb} {object}",
            object="the lazy dog",
            subject="the quick brown fox",
            verb="jumps over");
}