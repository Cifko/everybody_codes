pub mod binary_tree;
pub mod downloader;
pub mod math;

#[macro_export]
macro_rules! println_quest {
    ($num:expr) => {
        println!("  Quest {}:", $num)
    };
}

#[macro_export]
macro_rules! println_part {
  ($num:expr, $($arg:tt)*) => {
    println!("    Part {}: {}", $num, format!($($arg)*))
  };
}
