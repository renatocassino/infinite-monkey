use rand::prelude::*;
use std::io::Write;
use std::thread;
use std::fs::OpenOptions;

struct Monkey {
    id: u32
}

impl Monkey {
    pub fn new(id: u32) -> Monkey {
        return Monkey{ id }
    }

    pub fn get_type_letter(&self) -> char {
        let letters = vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
            's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' '];
        let a = rand::thread_rng().gen_range(0..letters.len());

        return letters[a];
    }

    pub fn type_word(&self) -> String {
        let mut typed_word = String::new();
        loop {
            let letter = self.get_type_letter();
            if letter == ' ' {
                break;
            }
            typed_word.push(letter);
        }
        return typed_word;
    }
}

fn main() {
    let cpus = num_cpus::get();

    for i in 0..cpus {
        thread::spawn(move|| {
            let monkey = Monkey::new(i as u32);
            let path = format!("./data/monkey_text_0{}.txt", monkey.id);
            let mut file_ref = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("Unable to open file");

            loop {
                let mut words = vec![];
                for _ in 0..1000 {
                    words.push(monkey.type_word());
                }

                let text = format!("{}\n", words.join(" "));
                file_ref.write_all(text.as_bytes()).expect("write failed");
            }
        });
    }

    loop {}
}
