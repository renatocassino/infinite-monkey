use redis::Commands;
use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufRead};
use std::num::NonZeroUsize;
use std::path::Path;
use std::thread;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let cpus = num_cpus::get();

    for i in 0..cpus {
        thread::spawn(move || {
            let client = redis::Client::open("redis://127.0.0.1/").unwrap();
            let mut con = client.get_connection().unwrap();

            println!("Thread {} started", i);
            let mut words_hash_map = HashMap::new();
            if let Ok(lines) = read_lines("./br-sem-acentos.txt") {
                for line in lines {
                    if let Ok(word) = line {
                        words_hash_map.insert(word, true);
                    }
                }
            }

            let queue_name = format!("word:{}", i);

            loop {
                let words: Vec<String> = con
                    .lpop(&queue_name, Some(NonZeroUsize::new(50).unwrap()))
                    .unwrap();

                if words.len() == 0 {
                    thread::sleep(std::time::Duration::from_millis(100));
                    continue;
                }

                for current_word in words {
                    match words_hash_map.get(&current_word) {
                        Some(_) => {
                            if words_hash_map.get(&current_word).unwrap() == &true {
                                let path_file = format!("./find_words/{}", current_word);
                                OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(path_file)
                                    .expect(format!("./find_words/{}", current_word).as_str());
                            }
                        }
                        None => {}
                    };
                }
            }
        });
    }

    loop {}
}
