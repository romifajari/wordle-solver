mod words;

use std::io::{self, Write};
use std::sync::LazyLock;
use words::{Filter, ValidWords};

static VALID_WORDS: LazyLock<ValidWords> = LazyLock::new(ValidWords::new);

fn parse_index_char_pairs(arg: &str) -> Vec<(usize, char)> {
    arg.split(',')
        .filter_map(|pair| {
            let mut parts = pair.split('-');
            match (parts.next(), parts.next()) {
                (Some(index), Some(ch)) => {
                    if let (Ok(i), Some(c)) = (index.parse::<usize>(), ch.chars().next()) {
                        if i < 5 {
                            return Some((i, c));
                        }
                    }
                    None
                }
                _ => None,
            }
        })
        .collect()
}

fn main() {
    let mut filter = Filter::new();

    println!("Wordle Solver REPL");
    println!("Commands:");
    println!("  green <index-char>[,more]   - Set green letters, e.g. green 0-s,1-t");
    println!("  yellow <index-char>[,more]  - Set yellow letters, e.g. yellow 2-a,3-e");
    println!("  gray <index-char>[,more]    - Set gray letters, e.g. gray 1-e,4-f");
    println!("  run                         - Show matching words");
    println!("  reset                       - Clear all filters");
    println!("  exit                        - Quit");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Error reading input");
            continue;
        }

        let parts: Vec<_> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "green" if parts.len() == 2 => {
                for (index, c) in parse_index_char_pairs(parts[1]) {
                    filter = filter.add_green(index, c);
                    println!("Added green: {} at {}", c, index);
                }
            }

            "yellow" if parts.len() == 2 => {
                for (index, c) in parse_index_char_pairs(parts[1]) {
                    filter = filter.add_yellow(index, c);
                    println!("Added yellow: {} not at {}", c, index);
                }
            }

            "gray" if parts.len() == 2 => {
                for (index, c) in parse_index_char_pairs(parts[1]) {
                    filter = filter.add_gray(index, c);
                    println!("Added gray: {} not at {}", c, index);
                }
            }

            "run" => {
                let matches = VALID_WORDS.filter_and_format(&filter);
                println!("Matches ({}):", matches.len());
                for word in matches {
                    println!("{}", word);
                }
            }

            "reset" => {
                filter = Filter::new();
                println!("Filter reset.");
            }

            "exit" => {
                break;
            }

            _ => {
                println!("Unknown or malformed command");
            }
        }
    }
}
