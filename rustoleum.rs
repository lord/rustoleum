use std::io::{stdin, File};
use std::io::buffered::BufferedReader;
use std::str::{from_utf8};

fn main () {
  let p = Path::new("data/example.txt");
  let contents = File::open(&p).read_to_end();
  let string_iter = from_utf8(contents).split_str(" ");
  let mut reader = BufferedReader::new(stdin());
  println("Input WPM: ");
  let input = reader.read_line().unwrap();
  let WPM = from_str::<int>(input.trim()).unwrap_or(300);
  println(WPM.to_str());
}
