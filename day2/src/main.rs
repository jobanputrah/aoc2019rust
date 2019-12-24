use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Read file fail");

    let ogprogram: Vec<u32> = content.trim()
                                .split(',')
                                .map(|n| n.parse().unwrap())
                                .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = ogprogram.clone();

            program[1] = noun;
            program[2] = verb;

            for i in (0..program.len()).step_by(4) {
                let opcode = program[i];

                if opcode == 99 {
                    break;
                }

                let a: usize = program[i + 1] as usize;
                let b: usize = program[i + 2] as usize;
                let c: usize = program[i + 3] as usize;

                if opcode == 1 {
                    program[c] = program[a] + program[b];
                } else if opcode == 2 {
                    program[c] = program[a] * program[b];
                } else {
                    println!("Something went wrong");
                    break;
                }
            }

            if program[0] == 19690720 {
                println!("{:?}", (100 * noun) + verb);
                break;
            }
        }
    }

}
