use cargo_proc_macro_print::hello;

fn main() {
    let _test = Test{};
    println!("Hello, world!");
}

#[hello]
struct Test {}