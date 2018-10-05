extern crate gcc;

fn main() {
    gcc::Config::new()
                .file("src/c_example.c")
                .include("src")
                .compile("libexample.a");
}
