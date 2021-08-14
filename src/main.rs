use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

fn main() {
    let elements_to_keep = 10;
    let mut reservoir = Vec::new();

    let filename = "/home/rukien/Téléchargements/SampleCSVFile_53000kb.csv";

    let mut i = 0;
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                if i <= elements_to_keep - 1 {
                    reservoir.push(ip);
                } else {
                    let mut rng = rand::thread_rng();
                    let rnd = rng.gen_range(1..i);
                    if rnd <= elements_to_keep - 1 {
                        let _ = std::mem::replace(&mut reservoir[rnd], ip);
                    }
                }
                i = i + 1;
            }
        }

        let mut j = 0;
        for kept in reservoir.iter() {
            j = j + 1;
            println!("#{}: {}", j, kept);

        }


    }

    println!("Done.");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
