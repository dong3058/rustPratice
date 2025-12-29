mod control;
mod collection;
mod iteration;
mod string;
mod structest;
mod enumtest;
mod traittest;
fn main() {

    control::control_method();
    collection::collections();
    string::str_test();
    iteration::iteraion_test();
    structest::struct_practice();
    enumtest::enum_test();
    traittest::trait_test();
    println!("Hello, world!");
}
