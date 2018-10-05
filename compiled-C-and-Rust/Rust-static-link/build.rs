extern crate gcc;

fn main() {
    gcc::Build::new()
	.file("src/c_example.c")
	.flag("-Ofast")
	.flag("-march=native")
	.include("src")
	.compile("libexample.a");
}
