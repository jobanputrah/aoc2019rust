use std::fs;
use std::error::Error;

#[derive(Debug)]
struct Line {
    x1: i64,
    y1: i64,
    x2: i64,
    y2: i64
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}

impl Line {
    fn intersects(&self, l2: &Self) -> bool {
        return ccw(self.x1, self.y1, l2.x1, l2.y1, l2.x2, l2.y2) != ccw(self.x2, self.y2, l2.x1, l2.y1, l2.x2, l2.y2) &&
               ccw(self.x1, self.y1, self.x2, self.y2, l2.x1, l2.y1) != ccw(self.x1, self.y1, self.x2, self.y2, l2.x2, l2.y2); 
    }

    fn intersects_at(&self, l2: &Self) -> Option<Point> {
        let a1 = self.y2 - self.y1;
        let b1 = self.x1 - self.x2;
        let c1 = a1 * self.x1 + b1 * self.y1;

        let a2 = l2.y2 - l2.y1;
        let b2 = l2.x1 - l2.x2;
        let c2 = a2 * l2.x1 + b2 * l2.y1;

        let d = a1 * b2 - a2 * b1;

        if d == 0 {
            return None;
        }

        Some(Point {
            x: (b2 * c1 - b1 * c2) / d,
            y: (a1 * c2 - a2 * c1) / d
        })
    }

    fn has_point(&self, p: &Point) -> bool {
        return man_dist(self.x1, self.y1, self.x2, self.y2) ==
            (man_dist(self.x1, self.y1, p.x, p.y) + man_dist(self.x2, self.y2, p.x, p.y));
    }
}


fn main() {
    run();
}

fn run() -> Option<()> {
    let content = fs::read_to_string("input.txt")
        .expect("Read file fail");

    let wires: Vec<&str> = content.trim().split("\n").collect();

    let l1: Vec<Line> = getlines(wires[0]).unwrap();
    let l2: Vec<Line> = getlines(wires[1]).unwrap();

    let mut inter_points: Vec<Point> = Vec::new();
    let mut min_dist: i64 = i64::max_value();

    for i in &l1 {
        for j in &l2 {
            if i.intersects(&j) {
                let p = i.intersects_at(&j).unwrap();
                let dist = man_dist(0, 0, p.x, p.y);
                if dist < min_dist {
                    min_dist = dist;
                }
                inter_points.push(p);
            }
        }
    }

    let mut min_steps: i64 = i64::max_value();

    for p in inter_points {
        let mut steps = 0;

        for i in &l1 {
            if i.has_point(&p) {
                steps += man_dist(i.x1, i.y1, p.x, p.y);
                break;
            }

            steps += man_dist(i.x1, i.y1, i.x2, i.y2);
        }

        for i in &l2 {
            if i.has_point(&p) {
                steps += man_dist(i.x1, i.y1, p.x, p.y);
                break;
            }

            steps += man_dist(i.x1, i.y1, i.x2, i.y2);
        }

        if steps < min_steps {
            min_steps = steps;
        }
    }

    println!("Distance to closest intersection: {}", min_dist);
    println!("Distance to intersection with min steps: {}", min_steps);

    Some(())
}

fn man_dist(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    let dx = x2 - x1;
    let dy = y2 - y1;

    return dx.abs() + dy.abs();
}

fn ccw(x1: i64, y1: i64, x2: i64, y2: i64, x3: i64, y3: i64) -> bool {
    let x = (y3 - y1) * (x2 - x1) > (y2 - y1) * (x3 - x1);
    return x;
}

fn getlines(dirs: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    let mut lines: Vec<Line> = Vec::new();

    let mut px = 0;
    let mut py = 0;

    for dir in dirs.split(",") {
        let d = dir.chars().next().unwrap();
        let mag: i64 = dir[1..].parse()?;

        let mut x2 = px;
        let mut y2 = py;

        match d {
            'R' => x2 += mag,
            'D' => y2 -= mag,
            'L' => x2 -= mag,
            'U' => y2 += mag,
            _ => {
                println!("Uknown direction {}", d);
            }
        };

        let line = Line {
            x1: px,
            y1: py,
            x2: x2,
            y2: y2
        };

        lines.push(line);

        px = x2;
        py = y2;
    }

    return Ok(lines);
}

