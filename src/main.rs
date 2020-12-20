use std::{env,fs};

fn main() {
    const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let args: Vec<String> = env::args().collect();

    let filename = &args.as_slice()[1];
    //println!("{:?}", filename);

    let file = fs::read_to_string(filename)
        .expect("could not read filename");

    let lines: Vec<&str> = file.split("\n").collect();
    let mut lines2 = Vec::new();
    for i in 0..lines.len() {
        let temp: Vec<&str> = lines[i].split_whitespace()
        .collect::<Vec<_>>();
        lines2.push(temp);
    }
    //println!("{:?}", lines2[2]);
    let mut counter = 0;
    let mut correct = 0;

    for i in &FIELDS {
        for j in 0..lines2.len() {
            for k in 0..lines2[j].len() {
                //println!("{:?}", lines2[j][k]);
                println!("{}", counter);
                println!("{:?}", lines2[j][k][0..2]);
                if lines2[j][k].is_empty() {
                    println!("{}", counter);
                    if counter == FIELDS.len() {
                        correct += 1;

                    }
                    counter = 0;
                } else if **i == lines2[j][k][0..2] {
                    counter += 1;                    
                }
            }
        }
    }
    println!("correct: {}", correct);




   // println!("{:?}", lines);
   // println!("");
   // println!("{:?}", lines2);
}