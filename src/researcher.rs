use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread;
use std::fs::OpenOptions;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let cpus = num_cpus::get();

    for i in 0..cpus {
        thread::spawn(move|| {
            let mut words_hash_map = HashMap::new();
            if let Ok(lines) = read_lines("./br-sem-acentos.txt") {
                for line in lines {
                    if let Ok(word) = line {
                        words_hash_map.insert(word, true);
                    }
                }
            }
            let path = format!("./data/monkey_text_0{}.txt", i);

            if let Ok(lines) = read_lines(path) {
                for line in lines {
                    if let Ok(all_words) = line {
                        let words = all_words.split(" ");

                        for current_word in words {
                            match words_hash_map.get(current_word) {
                                Some(_) => {
                                    if words_hash_map.get(current_word).unwrap() == &true {
                                        let path_file = format!("./find_words/{}", current_word);
                                        OpenOptions::new()
                                            .create(true)
                                            .append(true)
                                            .open(path_file)
                                            .expect(format!("./find_words/{}", current_word).as_str());
                                    }
                                },
                                None => { words_hash_map.insert(current_word.to_string(), false);
                                }
                            };
                        }
                    }
                }
            }

        });
    }

    loop {}
}
