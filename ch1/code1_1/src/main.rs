// use std::fs;

// fn main() {
//     let paths = fs::read_dir("./").unwrap();
   
//     // paths.for_each(|a| {
//     //     println!("{:#?}", a.unwrap().path());
//     // });
//         println!("{}\n{}","./","../");
//     for path in paths {
//         println!("{}", path.unwrap().path().display())
//     }
// }


// use std::backtrace::Backtrace;

// fn main() {
//     println!("Custom backtrace: {}", Backtrace::capture());

//     // ... or forcibly capture the backtrace regardless of environment variable configuration
//     println!("Custom backtrace: {}", Backtrace::force_capture());
// }

use std::{thread, time::Duration};

fn main() {
    thread::sleep(Duration::from_millis(4000));
}