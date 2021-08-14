use std::fs::File;
use std::fs::metadata;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    keep: usize,
}

fn main() {
    let args = Cli::from_args();

    if metadata(&args.path).is_err() {
        eprintln!("Error: Input file does not appear to exist");
        std::process::exit(2);
    }

    let elements_to_keep = args.keep;
    let mut reservoir = Vec::new();

    let input_file = args.path;

    let mut i = 0;
    if let Ok(lines) = read_lines(input_file) {
        for line in lines {
            if let Ok(ip) = line {
                if i < elements_to_keep {
                    reservoir.push(ip);
                } else {
                    let mut rng = rand::thread_rng();
                    let rnd = rng.gen_range(0..i);
                    if rnd < elements_to_keep {
                        let _ = std::mem::replace(&mut reservoir[rnd], ip);
                    }
                }
                i = i + 1;
            }
        }

        for kept in reservoir.iter() {
            println!("{}", kept);
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
