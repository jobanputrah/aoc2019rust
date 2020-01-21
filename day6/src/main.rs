use std::fs;
use std::collections::HashMap;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input");

    let orbits: Vec<&str> = input.trim().split("\n").collect();

    let mut parents: HashMap<&str, &str> = HashMap::new();

    for orbit_str in orbits {
        let orbit: Vec<&str> = orbit_str
            .split(")")
            .collect();

        parents.insert(orbit[1], orbit[0]);
    }

    part1(&parents);
    part2(&parents);
}

fn part2(parents: &HashMap<&str, &str>) {
    let mut san_dist: Vec<(&str, i32)> = Vec::new();
    let mut you_dist: Vec<(&str, i32)> = Vec::new();

    let mut curr = "SAN";
    let mut dist = -1;
    loop {
        curr = parents.get(curr).unwrap();
        dist += 1;

        san_dist.push((curr, dist));

        if curr == "COM" {
            break;
        }
    }

    curr = "YOU";
    dist = -1;
    loop {
        curr = parents.get(curr).unwrap();
        dist += 1;

        you_dist.push((curr, dist));

        if curr == "COM" {
            break;
        }
    }

    let mut found = false;
    for y in &you_dist {
        for s in &san_dist {
            if y.0 == s.0 {

                println!("{}", y.1 + s.1);
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }
}

fn part1(parents: &HashMap<&str, &str>) {
    let mut total_dist = 0;

    for &v in parents.keys() {
        if v == "COM" {
            continue;
        }

        let mut dist = 0;
        let mut curr = v;
        loop {
            curr = parents.get(curr).unwrap();
            dist += 1;

            if curr == "COM" {
                break;
            }
        }

        total_dist += dist;
    }

    println!("{}", total_dist);
}

