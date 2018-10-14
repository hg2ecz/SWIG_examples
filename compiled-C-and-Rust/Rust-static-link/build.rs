extern crate cc;

fn main() {
    cc::Build::new()
	.file("src/c_example.c")
	.flag("-Ofast")
	.flag("-march=native")
	.include("src")
	.compile("libexample.a");
}
