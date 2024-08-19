// fn main() {
//     panic!("crash and burn");
//     }


// thread 'main' panicked at src/main.rs:2:5:
// crash and burn
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// fn main() {
//     let v = vec![1, 2, 3];
//     v[99];
//     }
        
//In this situation, Rust will panic

//thread 'main' panicked at src/main.rs:12:6:
// index out of bounds: the len is 3 but the index is 99
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


// use std::fs::File;
// fn main() {
// let f = File::open("hello.txt");
// let f = match f {
// Ok(file) => file,
// Err(error) => {
// panic!("There was a problem opening the file: {:?}", error)
// },
// };
// }

// use std::fs::File;
// use std::io::ErrorKind;
//     fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => file,
//         Err(ref error) if error.kind() == ErrorKind::NotFound => {
//             match File::create("hello.txt") {
//             Ok(fc) => fc,
//                 Err(e) => {
//                     panic!(
//                     "Tried to create file but there was a problem: {:?}",
//                     e
//                 )
//             },
//         }
//     },
//     Err(error) => {
//         panic!(
//         "There was a problem opening the file: {:?}",
//         error
//         )
//         },
//     };
// }

// use std::fs::File;
// use std::io::ErrorKind;

// use std::fs::File;
// fn main() {
// let f = File::open("hello.txt").unwrap();
// }

// use std::fs::File;
// fn main() {
// let f = File::open("hello.txt").expect("Failed to open hello.txt");
// }

use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? after a result. 
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? after a result. 
    Ok(s)
}