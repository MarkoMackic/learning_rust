
/* won't compile
 * cause variables are immutable by default

fn main() {
    let x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);
}
*/

// will compile because we explicitly 
// set the variable to be mutable
// fn main() {
//     let mut x = 5;
//     println!("The value of x is : {}", x);
//     x = 6;
//     println!("The value of x is : {}", x);
// }

// we define constants with const instead of let, they are always immutable so mut doesn't apply.
// and they can be not be evaluated ( e.g by a function call or something, which Java for example
// allows, their type must be annotated, persisted in scope, can be global

//const MAX_POINT: u32 = 100_000;

// shadowing
// we can redeclare variables and they'll have the value of final declaration
//
// fn main() {
//     let x = 5;
//     let x = x + 1;
//     let x = x + 2;
//
//     println!("The value of x is {}", x);
//
//     // we can also change type
//
//     let spaces = "    ";
//     let spaces = spaces.len(); // now integer yeah yeah
//
//     // not ok with mut, because we can't change variable type
//
//     // let mut spaces = "   ";
//     // spaces = spaces.len(); // will result compile time error
// }
//



