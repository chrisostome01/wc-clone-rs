use std::{env, fs};

/**

-c: Byte count
-l: Line count
-w: Word count
-m: Character count (multibyte safe)
Default: Equivalent to -l -w -c
Stdin: Reads from standard input if no filename is provided

*/

struct Stats {
    total_words: usize,
    total_lines: usize,
    total_bytes: usize,
    total_chars: usize,
}

fn compute_stats(content: String) -> Stats {
    let total_words = content.split_whitespace().count();
    let total_lines = content.lines().count();
    let total_bytes = content.len();
    let total_chars = content.chars().count();

    Stats {
        total_words,
        total_lines,
        total_bytes,
        total_chars,
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let file_path = args.pop().expect("Unable to get file :)");
    let lines_count_flag = String::from("-l");
    let byte_count_flag = String::from("-c");
    let word_count_flag = String::from("-w");
    let character_count_flag = String::from("-m");
    let print_default = false;

    let mut show_total_bytes = false;
    let mut show_total_lines = false;
    let mut show_total_chars = false;
    let mut show_total_words = false;

    let mut output = String::from("");

    if args.len() >= 2 {
        args.drain(0..2);
    }

    if args.len() == 0 {
        show_total_lines = true;
        show_total_words = true;
        show_total_chars = true;
    }

    println!("{:?}", args);

    let content = fs::read_to_string(file_path).expect("Unable to read the file.");
    let stats = compute_stats(content);

    if (args.contains(&lines_count_flag)) {}
    // if (args.contains("-l")) {}
    // if (args.contains("-l")) {}

    println!(
        "{:}, {:}, {:}, {:}",
        stats.total_words, stats.total_lines, stats.total_bytes, stats.total_chars
    )
}
