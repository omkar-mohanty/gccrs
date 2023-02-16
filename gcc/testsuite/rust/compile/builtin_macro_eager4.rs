#[rustc_builtin_macro]
macro_rules! include{
  () => {{}};
}

macro_rules! file1 {
    () => {
        "builtin_macro_include_str.rs"
    };
}

fn main () {
  include!(file1!()); // ok
}
