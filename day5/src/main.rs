use std::fs;
use std::io::{self, Write};

fn load(file: &str) -> Vec<i32> {
    let content = fs::read_to_string(file)
        .expect("Read file fail");

    content.trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect()
}

fn set(prog: &mut Vec<i32>, loc: i32, val: i32) {
    prog[loc as usize] = val;
}

fn get(prog: &Vec<i32>, loc: i32, isref: bool) -> i32 {
    if isref {
        prog[loc as usize]
    } else {
        loc
    }
}

fn main() {
    let mut p = load("input.txt");

    let mut i: usize = 0;
    while i < p.len() {
        let mut op = p[i];
        let opcode = lastdigit(&mut op, 2);

        match opcode {
            1 => {
                let a = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let b = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);
                let c = p[i + 3];

                set(&mut p, c, a + b);

                i += 4;
            },
            2 => {
                let a = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let b = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);
                let c = p[i + 3];

                set(&mut p, c, a * b);

                i += 4;
            },
            3 => {
                print!("> ");
                io::stdout().flush().ok().expect("Could not flush stdout");

                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read input");
                let input: i32 = input.trim().parse().expect("Invalid input");

                let a = p[i + 1];
                set(&mut p, a, input);

                i += 2;
            },
            4 => {
                let output = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                println!("{}", output);

                i += 2;
            },
            5 => {
                let cond = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let jump = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);

                if cond != 0{
                    i = jump as usize;
                } else {
                    i += 3;
                }
            },
            6 => {
                let cond = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let jump = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);

                if cond == 0{
                    i = jump as usize;
                } else {
                    i += 3;
                }
            },
            7 => {
                let a = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let b = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);
                let c = p[i + 3];

                let mut val: i32 = 0;
                if a < b {
                    val = 1;
                }

                set(&mut p, c, val);

                i += 4;
            },
            8 => {
                let a = get(&p, p[i + 1], lastdigit(&mut op, 1) == 0);
                let b = get(&p, p[i + 2], lastdigit(&mut op, 1) == 0);
                let c = p[i + 3];

                let mut val: i32 = 0;
                if a == b {
                    val = 1;
                }

                set(&mut p, c, val);

                i += 4;
            },
            99 => {
                break;
            },
            _ => {
                println!("Unkown opcode: {}", opcode);
            },
        }
    }
}

fn lastdigit(n: &mut i32, i: i32) -> i32 {
    if *n == 0 {
        return 0;
    }

    let p = 10i32.pow(i as u32);
    let d = *n % p;

    *n /= p;

    return d;
}

