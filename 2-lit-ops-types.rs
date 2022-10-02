/*
* Underscores can be inserted in numeric literals to improve readability, e.g. 1_000 is the same as 1000, 
and 0.000_001 is the same as 0.000001.
* The operators available and their precedence in Rust are similar to other C-like languages.
*/

fn lit_types() {
    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);
}

// Tuples in rust
fn tuples(){
    // A tuple with a bunch of different types
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    // Values can be extracted from the tuple using tuple indexing. Doesn't support []
    println!("long tuple first value: {}", long_tuple.0);

    // printable.. !! But tuples of more than 12 elements can't be printed
    println!("tuple contains --  {:?}", long_tuple);

    // pair 
    let pair = (1, true);
    fn reverse_pair(pair: (i32, bool))-> (bool, i32){
        (pair.1, pair.0)
    }
    println!("reversed pair - {:?}", reverse_pair(pair));

    // destruction of tuples :):) lol
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);
}

fn main(){
    lit_types();
    tuples()
}