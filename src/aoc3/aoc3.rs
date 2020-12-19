use std:: {io, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if let [_, filename, ..] = args.as_slice() { 
    let contents = fs::read_to_string(filename)
        .expect("could not read file");
       let lines: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
       //Input amount of steps right and down, separated by spaces.
       println!("How many down?");
       let mut down = String::new();
       io::stdin().read_line(&mut down).unwrap();
       let down: Vec<usize> = down.split_whitespace().filter_map(|w| w.parse().ok()).collect();
       println!("How many right?");
       let mut right = String::new();
       io::stdin().read_line(&mut right).unwrap();
       let right: Vec<usize> = right.split_whitespace().filter_map(|w| w.parse().ok()).collect();
       //let step = (chars * down) + right;
       //println!("{:?}", step);

       let mut trees = 0;
       let mut line = 0;
       let mut spot = 0;
       let mut product: i64 = 1;

       if right.len() == down.len() {
           for i in 0..right.len() {
                while line < lines.len() {
                    if lines[line].chars().nth(spot) == Some('#') {
                        trees += 1;
                    }
                    line += down[i];
                    if spot + right[i] < 31 {
                        spot += right[i];
                    } else {
                        spot = (spot + right[i]) - 31
                    }

                }
                product = product * trees;
                trees = 0;
                line = 0;
                spot = 0;
           }
       }
       println!("{}", product);
    }
}