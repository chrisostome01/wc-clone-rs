use std::{
    env, fs,
    io::{self, Read},
};

struct Stats {
    words: usize,
    lines: usize,
    bytes: usize,
    chars: usize,
}

fn compute_stats(content: &str) -> Stats {
    Stats {
        words: content.split_whitespace().count(),
        lines: content.lines().count(),
        bytes: content.len(),
        chars: content.chars().count(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let flags: Vec<&String> = args.iter().filter(|a| a.starts_with('-')).collect();
    let file_path = args.iter().skip(2).find(|a| !a.starts_with('-'));

    // Read content from File or Stdin
    let content = match file_path {
        Some(path) => fs::read_to_string(path).expect("Unable to read file"),
        None => {
            let mut buffer = String::new();
            io::stdin()
                .read_to_string(&mut buffer)
                .expect("Unable to read stdin");
            buffer
        }
    };

    let stats = compute_stats(&content);

    // Determine what to display
    let show_l = flags.contains(&&"-l".to_string());
    let show_w = flags.contains(&&"-w".to_string());
    let show_c = flags.contains(&&"-c".to_string());
    let show_m = flags.contains(&&"-m".to_string());

    // Default is -l -w -c
    let default = !show_l && !show_w && !show_c && !show_m;

    let mut output = Vec::new();

    if show_l || default {
        output.push(stats.lines.to_string());
    }
    if show_w || default {
        output.push(stats.words.to_string());
    }
    if show_c || default {
        output.push(stats.bytes.to_string());
    }
    if show_m {
        output.push(stats.chars.to_string());
    }

    // Add path at the end
    // if let Some(path) = file_path {
    //     output.push(path.clone());
    // }

    match file_path {
        Some(path) => output.push(path.to_string()),
        None => {}
    };

    println!("{}", output.join(" "));
}
