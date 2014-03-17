use std::io::{stdin, File, timer};
use std::io::buffered::BufferedReader;
use std::str::{from_utf8};
use std::vec::ChunkIter;
use std::vec::OwnedStr;

fn main () {
  let p = Path::new("data/example.txt");
  let contents = File::open(&p).read_to_end();
  let mut string_iter = from_utf8(contents).split_str(" ");
  let mut reader = BufferedReader::new(stdin());
  println("Input WPM: ");
  let input = reader.read_line().unwrap();
  let WPM = from_str::<uint>(input.trim()).unwrap_or(300);

  println("Input the number of words to view at a time: ");
  let input = reader.read_line().unwrap();
  let words_at_a_time = from_str::<uint>(input.trim()).unwrap_or(1);

  let sleep_time = 60000 / WPM * words_at_a_time;

  let string_vec = string_iter.to_owned_vec();

  for words in string_vec.chunks(words_at_a_time) {
    println("\x1bc".into_owned().append(words.connect(" ")));
    // println();
    std::io::timer::sleep(sleep_time.to_u64().unwrap());
  }

  // for (i, word) in string_iter.enumerate() {
	 //  print(word);
  //   if i % words_at_a_time == (words_at_a_time - 1) {
  //     println("");
  //     println("\x1bc");
  //   }
  // }
}
