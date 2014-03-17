use std::io::{stdin, File, timer};
use std::io::buffered::BufferedReader;
use std::str::{from_utf8};

fn main () {
  let p = Path::new("data/example.txt");
  let contents = File::open(&p).read_to_end();
  let mut string_iter = from_utf8(contents).split_str(" ");
  let mut reader = BufferedReader::new(stdin());
  println("Input WPM: ");
  let input = reader.read_line().unwrap();
  let WPM = from_str::<int>(input.trim()).unwrap_or(300);
  let sleep_time = 60000 / WPM;
  for word in string_iter {
	  print("\x1bc");
	  println(word);
	  std::io::timer::sleep(sleep_time.to_u64().unwrap());
  }
}
