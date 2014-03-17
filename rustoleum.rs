use std::io::{File, io_error, Open, Read};
use std::str;

fn main () {
  println("hello world!");
  let p = Path::new("data/example.txt");
  let contents = File::open(&p).read_to_end();
  println(std::str::from_utf8(contents));
}