use std:: {env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];
        let contents = fs::read_to_string(filename)
            .expect("could not read file");
        let lines: Vec<&str> = contents.split("\n").collect();
        let mut varname = Vec::new();

        for i in 0..lines.len() {
            let temp: Vec<&str> = lines[i].split_whitespace().collect::<Vec<_>>();
            varname.push(temp);
        }
        if query == "p1" {
            println!("amount correct: {}", part_one(varname));
        } else if query == "p2" {
            println!("amount correct: {}", part_two(varname));
        }
    }
}

fn part_one(input: Vec<Vec<&str>>) -> usize {
    let mut amount_correct = 0usize;
    for i in 0..input.len() {
        let mut counter = 0;
        let compare = input[i][1].chars().nth(0);

        for k in input[i][2].chars() {
            if Some(k) == compare {
                counter += 1;
            } else {
                continue;
            }
        }      
        let ints: Vec<i32> = input[i][0].split("-").filter_map(|w| w.parse().ok()).collect::<Vec<i32>>();
        if counter >= ints[0] && counter <= ints[1] {
            amount_correct += 1;
        }
    }
    return amount_correct;
}

fn part_two(input: Vec<Vec<&str>>) -> usize {
    let mut amount_correct = 0usize;
    for i in 0..input.len() {
        let mut ok = 0;
        let compare = input[i][1].chars().nth(0);
        let ints: Vec<i32> = input[i][0].split("-").filter_map(|w| w.parse().ok()).collect::<Vec<i32>>();
       
        if input[i][2].chars().nth(ints[0] as usize - 1) == compare {
            ok += 1;
        }
        if input[i][2].chars().nth(ints[1] as usize - 1) == compare {
            ok += 1;
        } 
        if ok == 1 {
            amount_correct += 1;
        }
    }
    return amount_correct;
}
