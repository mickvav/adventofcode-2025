use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
struct Line {
    v: Vec<char>,
}

#[derive(Debug)]
struct Inp {
    r: Vec<Line>,
}

#[derive(Debug)]
struct TlBeams {
    b: HashMap<usize, i64>,
}

impl TlBeams {
    fn add(&mut self, pos: usize, u: i64) {
        if self.b.contains_key(&pos) {
            let v = self.b[&pos];
            self.b.insert(pos, v + u);
        } else {
            self.b.insert(pos, u);
        };
    }
    fn count(&self) -> i64 {
        let mut res = 0;
        for (_k, v) in self.b.iter() {
            res = res + v;
        }
        return res;
    }
}

impl Line {
    fn split(&self, beams: Vec<usize>) -> (Vec<usize>, i64) {
        let mut res: Vec<usize> = Vec::new();
        let mut ops: i64 = 0;
        let mut set: HashSet<usize> = HashSet::new();
        for b in beams {
            if self.v[b] == '.' {
                set.insert(b);
            } else {
                ops = ops + 1;
                set.insert(b - 1);
                set.insert(b + 1);
            };
        }
        for b in set.iter() {
            res.push(*b);
        }
        return (res, ops);
    }
    fn splittimeline(&self, beams: TlBeams) -> TlBeams {
        let mut res = TlBeams { b: HashMap::new() };
        for (b, tls) in beams.b.iter() {
            if self.v[*b] == '.' {
                res.add(*b, *tls);
            } else {
                res.add(b - 1, *tls);
                res.add(b + 1, *tls);
            };
        }
        return res;
    }
    fn start(&self) -> Vec<usize> {
        let mut res: Vec<usize> = Vec::new();
        for i in 0..self.v.len() {
            if self.v[i] == 'S' {
                res.push(i);
                return res;
            }
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
        res.v.push(b);
    }
    return res;
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp { r: Vec::new() };
    for line in lines.lines() {
        inp.r.push(parse_line(line));
    }
    return inp;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut score: i64 = 0_i64;
        let mut b = self.r[0].start();
        for r in self.r.iter() {
            let (b1, splits) = r.split(b);
            b = b1;
            score = score + splits;
        }
        return score;
    }
    fn metrics2(&self) -> i64 {
        let bs = self.r[0].start();
        let mut bt = TlBeams {
            b: HashMap::from([(bs[0], 1)]),
        };

        println!("{:?}", bt);
        for r in self.r.iter() {
            let b1 = r.splittimeline(bt);
            bt = b1;
            println!("{:?}", bt);
        }
        return bt.count();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(), 21);
        assert_eq!(read_file("test.txt").metrics2(), 40);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
