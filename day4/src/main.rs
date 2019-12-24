use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input.txt");

    let range: Vec<&str> = input.trim().split("-").collect();
    let from: i64 = range[0].parse().unwrap();
    let to: i64 = range[1].parse().unwrap();

    let mut possibilities = 0;

    let mut pass = from;
    while pass != to {
        if is_valid(pass) {
            possibilities += 1;
        }

        pass += 1;
    }

    println!("Possible passwords: {}", possibilities);
}

fn is_valid(pass: i64) -> bool {
    let mut pass_t = pass;

    let mut has_inc = true;
    let mut has_adj = false;

    let mut pd = pass_t % 10;
    pass_t /= 10;

    let mut rep_no = pd;
    let mut rep_count = 1;

    while pass_t >= 1 {
        let d = pass_t % 10;

        if pd < d {
            has_inc = false;
            break;
        }

        if !has_adj {
            if rep_no == d {
                rep_count += 1;
            } else {
                if rep_count == 2 {
                    has_adj = true;
                } else {
                    rep_count = 1;
                    rep_no = d;
                }
            }
        }

        pd = d;
        pass_t /= 10;
    }

    if rep_count == 2 {
        has_adj = true;
    }

    return has_inc && has_adj;
}
