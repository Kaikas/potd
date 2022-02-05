mod quote;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut pastlines = Vec::<String>::new();
    let mut quotes = Vec::<String>::new();
    println!("mod quote {{");
    println!("pub fn get_random_quote() {{");
    println!("let quotes = Vec::<String>::new()");
    if let Ok(lines) = read_lines("./pqf") {
        let mut count = 0;
        for line in lines {
            if let Ok(l) = line {
                if l == "" {
                    count += 1;
                } else {
                    count = 0;
                    pastlines.push(l);
                }
                if count == 2 {
                    quotes.push(pastlines.join("\n"));
                    for x in &quotes {
                        println!("quotes.push(\"{}\");", x.replace("\"", "''"));
                    }
                    pastlines.clear();
                }
            }
        }
    }
    println!("quotes.choose(&mut rand::thread:rng());");
    println!("}}\n}}")
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
