use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct Inp {
    rng: Vec<Range>,
}

fn parse_line(line: &str) -> Inp {
    let mut inp = Inp { rng: Vec::new() };
    if line.len() == 0 {
        return inp;
    }
    let v: Vec<&str> = line.split(",").collect();
    for range in v {
        let d: Vec<&str> = range.split("-").collect();
        inp.rng.push(Range{
           start: d[0].to_string().parse::<i64>().unwrap(),
           end: d[1].to_string().parse::<i64>().unwrap(),
        });
    }
    return inp;
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let inp = Inp { rng: Vec::new() };
    for line in lines.lines() {
        return parse_line(line);
    }
    return inp;
}

fn is_silly(v : i64) -> bool {
    let digits = 1+v.ilog10();
    let d2 = digits/2;
    let b: i64 = 10_i64.pow(d2);
    return v / b == v % b;
}

//
//  rotate - turn, say, 12345 into 45123
//   
fn shiftrot(v: i64, s:u32) -> i64 {
    let digits = 1+v.ilog10();
    let b: i64 = 10_i64.pow(s);
    let rem: i64 = v % b;
    let b1: i64 = 10_i64.pow(digits - s);
    return rem * b1 + v / b;
}
    

fn is_silly2(v : i64) -> bool {
    let digits = 1+v.ilog10();
    let d2 = digits/2;
    for i in 1..d2 {
        if shiftrot(v,i) == v {
            return true;
        };
    }
    return false;
}

impl Inp {
    fn metrics(&self) -> i64 {
        let mut score : i64 = 0_i64;
        for r in self.rng.iter() {
            for n in r.start..r.end+1 {
                if is_silly(n) {
                    score += n
                }
            }
        }
        return score;
    }
    fn metrics2(&self) -> i64 {
        let mut score : i64 = 0_i64;
        for r in self.rng.iter() {
            for n in r.start..r.end+1 {

                if is_silly2(n) {
                    println!("silly: {:?}", n);
                    score += n
                }
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
        assert_eq!(read_file("test.txt").metrics(), 1227775554);
        assert_eq!(read_file("test.txt").metrics2(), 4174379265);
    }

    #[test]
    fn shiftrot_works() {
        assert_eq!(shiftrot(12345,2), 45123);
        assert_eq!(shiftrot(11,1),11);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics());
    println!("Part2 {:?}", v.metrics2());
}
