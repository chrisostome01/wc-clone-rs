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
    let args: Vec<String> = env::args().collect();
    let file_path = &args[args.len() - 1];
    let content = fs::read_to_string(file_path).expect("Unable to read the file.");

    let stats = compute_stats(content);
    println!(
        "{} {} {} {}",
        stats.total_lines, stats.total_words, stats.total_bytes, stats.total_chars
    );
}
