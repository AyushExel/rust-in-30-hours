/*
Refresher kids - arrays are stored in contiguous memory. Hence, the super fast speed. Most DS can be/are some sort of 
array derivative. Most. Not all
*/

use std::mem;

/// slices don't need to specify size at compile time
fn analyse_slice(sl: &[i32]){
 println!("{len} is the length of slice and {elem} is its first element", len=sl.len(), elem=sl[0]);
}

fn arrays(){
 // signature - [T; size]
 let xs: [i32; 5] = [1,2,3,4,5];

 // initialize all values to 0
 let ys: [i32; 500] = [0; 500];

 // check memory occupied
 println!("ys occupy {} bytes of contiguous memory", mem::size_of_val(&ys));

 // Slicing. Borrow the whole array
 analyse_slice(&xs);
 
 // slice array [starting index .. one after the last index]
 analyse_slice(&xs[1..4]); // slice index 1 to 4 (excluding)

 // get-some-None pattern
 for i in 0..xs.len() + 1 { //one element too far
    match xs.get(i) {
        Some(xval) => println!("{}: {}", i, xval),
        None => println!("Slow down! {} is too far!", i),
    }
}
}

fn main(){
    arrays();
}