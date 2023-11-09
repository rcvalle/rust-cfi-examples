#[link(name = "foo")]
extern "C" {
    fn hello_from_c();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        hello_from_c();
    }
}
