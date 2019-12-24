use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Read file fail");

    let mut total: i32 = 0;

    for s in content.split("\n") {
        if s.len() == 0 {
            continue;
        }

        let mut mass: i32 = s.parse()
            .expect("Parse fail");

        let mut fuel = get_fuel(mass);
        while fuel > 0 {
            total += fuel;
            fuel = get_fuel(fuel);
        }
    }

    println!("Total {}", total);
}

fn get_fuel(mass: i32) -> i32 {
    ((mass as f32 / 3.0).floor() as i32) - 2
}
