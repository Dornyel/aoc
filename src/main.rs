use std::{env,fs};

pub const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Copy, Clone, Debug)]
struct PassportValid {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool
}
impl Default for PassportValid {
    fn default() -> PassportValid {
        PassportValid {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false
            
        }
    }
}
impl PassportValid {
    pub fn check_fields(mut self, c: &&str, l: &str) -> PassportValid {
        if *c == "byr" {
            if l[4..8].trim().parse::<i32>().unwrap() >= 1920 && 
            l[4..8].trim().parse::<i32>().unwrap() <= 2002 {
                self.byr = true;
            } else {self.byr = false;}

        } else if *c == "iyr" {
            if l[4..8].trim().parse::<i32>().unwrap() >= 2010 && 
            l[4..8].trim().parse::<i32>().unwrap() <= 2020 {
                self.iyr = true;
            } else {self.iyr = false;}

        } else if *c == "eyr" {
            if l[4..8].trim().parse::<i32>().unwrap() >= 2020 && 
            l[4..8].trim().parse::<i32>().unwrap() <= 2030 {
                self.eyr = true;
            } else {self.eyr = false;}

        } else if *c == "hgt" {
                    if l.len() == 8 {
                if (String::from(&l[5..])).contains("in") {
                    if String::from(&l[4..6]).trim().parse::<i32>().unwrap() >= 59 && 
                    String::from(&l[4..6]).trim().parse::<i32>().unwrap() <= 76 {
                        self.hgt = true;
                    } else {self.hgt = false;}
                }
            } else if l.len() == 9 {
                if (String::from(&l[6..])).contains("cm") {                    
                    if String::from(&l[4..7]).trim().parse::<i32>().unwrap() >= 150 && 
                    String::from(&l[4..7]).trim().parse::<i32>().unwrap() <= 193 {
                        self.hgt = true;
                    } else {self.hgt = false;}
                } else {self.hgt = false;}
            }
             else {
                return self
            }

        } else if *c == "hcl" {
            let hcl = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
            let mut correct = 0;
            if l.len() == 11 {
                if l.chars().nth(4) == Some('#') {
                    for i in 5..12 {
                        for j in 0..hcl.len() {
                            if l.chars().nth(i) == Some(hcl[j]) {
                                correct += 1;
                            }
                        }
                    }
                }
            }
            if correct == 6 {

                self.hcl = true;
            } else {
                self.hcl = false;
            }

        } else if *c == "ecl" {
            if l.len() == 7 {
                for i in &ECL {
                    if *i == String::from(&l[4..]) {
                        self.ecl = true;
                    }
                }
            } else {
                self.ecl = false;
            }
            
        } else if *c == "pid" {
            let chars: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            let mut counter = 0;
            let num2 = String::from(&l[4..]);
            if num2.len() == 9{
                for i in num2.chars() {
                    for j in &chars {
                        if i == *j {
                            counter += 1;
                        }
                    }
            }
        }
            if counter == 9 {
                self.pid = true;
            } else {
                self.pid = false;
            }
        }
        return self;
    }
}

    fn is_valid(p: &PassportValid) -> bool {

        if p.byr == true &&
            p.iyr == true &&
            p.eyr == true &&
            p.hgt == true &&
            p.hcl == true &&
            p.ecl == true &&
            p.pid == true {
                return true;
            } else {
                return false;
            }
    }

fn main() {

    const FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let args: Vec<String> = env::args().collect();

    let filename = &args.as_slice()[1];

    let file = fs::read_to_string(filename)
        .expect("could not read filename");

    let lines: Vec<&str> = file.split("\n").collect();
    let mut lines2 = Vec::new();
    for i in 0..lines.len() {
        let temp: Vec<&str> = lines[i].split(" ")
        .collect::<Vec<_>>();
        lines2.push(temp);
    }
    let mut counter = 0;
    let mut correct = 0;
    let mut passport = PassportValid::default();

    for i in 0..lines2.len() {
        for j in &FIELDS {
            for k in 0..lines2[i].len() {
                if lines2[i][k] == "" {
                    if counter == FIELDS.len() {
                        if is_valid(&passport) == true {
                            correct += 1;
                            passport = PassportValid::default();
                        } else { 
                            passport = PassportValid::default();
                        }
                    }
                    counter = 0;
                } else if *j == &lines2[i][k][0..3] {
                    counter += 1;
                    passport = passport.check_fields(j, lines2[i][k]);
                }
            }
        }
    }
    println!("correct: {}", correct);
}
