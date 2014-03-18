use std::io::{stdin, File};
use std::io::buffered::BufferedReader;
use std::str::{from_utf8};
use std::os;

fn print_usage () {
	println("A tool to keep your reading skills bright and shiny.");
	println("Usage: rustoleum [arguments] path/to/input/file");
	println("-i (I)nteractive mode");
	println("-wpm N Display N words per (m)inute (default: 300)");
	println("-wpl N Display N words per (l)ine (default: 1)");
	println("-h Display this (h)elp");
}

fn get_args_interactive () -> (uint, uint, Path) {
	let mut reader = BufferedReader::new(stdin());

	println("Input path to file to read: ");
	let input = reader.read_line().unwrap_or(~"data/example.txt");
	let p = Path::new(input.trim());

	println("Input WPM: ");
	let input = reader.read_line().unwrap();
	let WPM = from_str::<uint>(input.trim()).unwrap_or(300);

	println("Input the number of words to view at a time: ");
	let input = reader.read_line().unwrap();
	let WPL = from_str::<uint>(input.trim()).unwrap_or(1);

	(WPM, WPL, p)
}

fn get_vals (args: ~[~str]) -> (uint, uint, Path){
	if args.contains(&~"-i"){
		get_args_interactive()
	}  else {
		let fallbackfile = &~"data/example.txt";
		let path = args.tail().last_opt().unwrap_or(fallbackfile);
		let p = Path::new(path.as_slice());
		let WPM = from_str::<uint>(args[1 + args.position_elem(&~"-wpm").unwrap_or(-1)]).unwrap_or(300);
		let WPL = from_str::<uint>(args[1 + args.position_elem(&~"-wpl").unwrap_or(-1)]).unwrap_or(1);
		(WPM,WPL,p)
	}
}

fn print_file(WPM: uint, WPL: uint, p: Path){
	if p.exists(){
		let contents = File::open(&p).read_to_end();
		let mut string_iter = from_utf8(contents).words();
		let string_vec = string_iter.to_owned_vec();
		let sleep_time = 60000 / WPM * WPL;
		for words in string_vec.chunks(WPL) {
			println("\x1bc".into_owned().append(words.connect(" ")));
			std::io::timer::sleep(sleep_time.to_u64().unwrap());
		}
	} else {
		println("The path specified does not exist. Please specify a valid path, or run rustoleum -h for help.");
	}
}
fn main () {
	if os::args().contains(&~"-h"){
		print_usage();
	} else {
		let (WPM, WPL, p) = get_vals(os::args());
		print_file(WPM, WPL, p);
	}

}
