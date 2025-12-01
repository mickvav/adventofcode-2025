use std::fs;

#[derive(Debug)]
struct Rot {
    v: i32,
}

#[derive(Debug)]
struct Inp {
    rot: Vec<Rot>,
}

fn parse_line(line: &str) -> Rot {
    if line.len() == 0 {
        return Rot { v: 0 };
    }
    if line[0..1] == *"R" {
        let p = &line[1..];
        return Rot {
            v: p.to_string().parse::<i32>().unwrap(),
        };
    }
    let p = &line[1..];
    return Rot {
        v: -p.to_string().parse::<i32>().unwrap(),
    };
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp { rot: Vec::new() };
    for line in lines.lines() {
        inp.rot.push(parse_line(line))
    }
    return inp;
}

impl Inp {
    fn metrics(&self) -> i32 {
        let mut pos = 50;
        let mut score = 0;
        for r in self.rot.iter() {
            println!("s: {:?} pos: {:?}", score, pos);
            pos = pos + r.v;
            if pos < 0 {
                pos = (pos + 100000) % 100
            }
            if pos > 99 {
                pos = pos % 100
            }
            if pos == 0 {
                score = score + 1;
            }
        }
        return score;
    }
    fn metrics2(&self) -> i32 {
        let mut pos = 50;
        let mut score = 0;
        for r in self.rot.iter() {
            println!("s: {:?} pos: {:?} v:{:?}", score, pos, r.v);
            let ppos = pos;
            pos = pos + r.v;
            if pos == 0 {
                score = score + 1;
            }
            if pos < 0 {
                println!("== pos: {:?} m: {:?}", pos, -pos/100);
                score += -pos /100 + 1;
                pos = (pos + 100000) % 100;
                if ppos == 0 {
                    score = score -1;
                }
            }
            if pos > 99 {
                score += pos / 100;
                pos = pos % 100
            }
        }
        return score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 3);
        assert_eq!(read_file("test.txt").metrics2(), 6);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
