// Yo, let's go into data types
//
// scalar ( integer, fp nums, bools, chars ) and compund ( which is logical ) 
//
// statically typed language with type inference
//
// we have to add type annotation if needed .. 
// let guess: u32 = "42".parse().expect("Not a number!");
//
// SCALARS: 
//
// integers:
//
// defaults to i32 ( fastest impl, even on 64 bit system ) 
//
// i -> signed, u -> unsigned
// 8 to 64 + arch (isize, usize) 
//
// literals allow type suffix and _ as visual separator
// 
// fp's : 
//
// defaults to f64 ( slower on 32 bit systems, but can be used )
//
// fn main()
// {
//     let x = 2.0; // f64
//     let y: f32 = 3.0; //f32
// }
//
// standard : IEEE-754 
//
// standard math ops + - / * %
//
// bools:
//
// true/ false ( heh ) 
//
// char type:
// 
// unicode support
//
//
// COMPOUND TYPES:
//
// basic compund types: tuples and array
// 
// TUPLES:
//
// fn main() {
//     // type annotation is optional if can be inferred
//     // but without annotations this would be i32, f64, i32 
//     let tup: (i32, f64, u8 ) = ( 500, 6.4, 1 );
// }
//
// supports destructuring 
//
// fn main() {
//     let tup = (500, 100, 1);
//     let (x, y, z) = tup;
//     println!("The value of y is {}", y);
// } 
//
// // supports indexing with . ( 0 indexed ) 
//
// fn main() {
//     let tup = (500, 100, 1);
//     let five_hundred = tup.0;
//     let one_hundred = tup.1;
//     println!("Yo, 500 = {}", five_hundred);
// }
//
// ARRAYS:
// 
// fixed size, can't grow or shrink, type of elems must be the same ( I think it's standard ) 
//
// fn main()
// {
//     let a = [1,2,3,4,5,6];
// }
//
// not same as vectors ( but that can be mentally inferred ) 
//
// data held on stack 
// 
// panics if idx is out of bounds ( why didn't they compile time check it where they could ? ) 
//
// accessing is 0 index based like 
//
// a[idx] 

