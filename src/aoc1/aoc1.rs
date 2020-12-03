use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_config(&args);
    let contents = fs::read_to_string(filename)
        .expect("Couldn't read file.");
    let query = query.trim().parse::<i32>().unwrap();
    let integers: Vec<usize> = contents.split_whitespace().filter_map(|w| w.parse().ok()).collect();
    if query == 2 {
        two(&integers);
    } else if query == 3 {
        three(&integers);
    } else {
        std::process::exit(1);
    }

}
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

fn two(ints: &Vec<usize>) {
    for i in ints {
        for j in ints {
            if i == j {
                continue;
            } else if i + j == 2020 {
                println!("i is {} \nj is {}\ni * j is {}",i, j, i*j);
                    std::process::exit(1);
            }
        }
    }
}

fn three(ints: &Vec<usize>) {
    for i in ints {
        for j in ints {
            for k in ints {
                if k == i || k == j {
                    continue;
                } else if (i + j + k) == 2020 {
                    println!("i is {} \nj is {}\nk is {}\ni * j * k is {}",i, j, k, i*j*k);
                    std::process::exit(1);
                }
            } 
        }
    }
}