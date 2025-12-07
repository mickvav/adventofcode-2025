use std::fs;

#[derive(Debug)]
struct Line {
    v: Vec<i16>,
}

#[derive(Debug)]
struct Inp {
    rng: Vec<Line>,
}

impl Line {
    fn joltage(&self) -> i16 {
        let mut firstpos = 0;
        let mut firstvalue = self.v[0];
        for i in 0..self.v.len() - 1 {
            if self.v[i] > firstvalue {
                firstvalue = self.v[i];
                firstpos = i;
            };
        }
        let mut secondvalue = self.v[firstpos + 1];
        for i in firstpos + 1..self.v.len() {
            if self.v[i] > secondvalue {
                secondvalue = self.v[i];
            };
        }
        return firstvalue * 10 + secondvalue;
    }

    fn joltage2(&self) -> i64 {
        let mut pos: [usize; 12] = [0; 12];
        let mut value: [i16; 12] = [0; 12];
        let mut nextpos = 0;
        for j in 0..12 {
            for i in nextpos..self.v.len() - (11 - j) {
                if self.v[i] > value[j] {
                    value[j] = self.v[i];
                    pos[j] = i;
                    nextpos = i + 1;
                }
            }
        }
        let mut res: i64 = 0;
        let mut multiplier: i64 = 1;
        for j in 0..12 {
            res = res + multiplier * i64::from(value[11 - j]);
            multiplier = multiplier * 10;
        }
        return res;
    }
}

fn parse_line(line: &str) -> Line {
    let mut res = Line { v: Vec::new() };
    if line.len() == 0 {
        return res;
    }
    for b in line.chars() {
        res.v.push(b.to_string().parse::<i16>().unwrap());
    }
    return res;
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp { rng: Vec::new() };
    for line in lines.lines() {
        inp.rng.push(parse_line(line));
    }
    return inp;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut score: i64 = 0_i64;
        for r in self.rng.iter() {
            score = score + i64::from(r.joltage());
        }
        return score;
    }
    fn metrics2(&self) -> i64 {
        let mut score: i64 = 0_i64;
        for r in self.rng.iter() {
            score = score + r.joltage2();
        }
        return score;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 357);
        assert_eq!(read_file("test.txt").metrics2(), 3121910778619);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
