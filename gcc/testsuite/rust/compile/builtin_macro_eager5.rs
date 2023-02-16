#[rustc_builtin_macro]
macro_rules! env {
  () => {{}};
}

macro_rules! test_var {
  () => {
    "NOT_DEFINED"
  };
}

fn main() {
    env!(test_var!()) // { dg-error "environment variable 'NOT_DEFINED' not defined" "" }
}
