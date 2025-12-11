use std::cmp::min;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Jb {
    idx: usize,
    v: [i64; 3],
}

impl PartialEq for Jb {
    fn eq(&self, other: &Jb) -> bool {
        return self.v == other.v;
    }
}
#[derive(Debug)]
struct Jbtuple {
    a: Jb,
    b: Jb,
}

impl Eq for Jbtuple {}

impl PartialEq for Jbtuple {
    fn eq(&self, other: &Jbtuple) -> bool {
        return (self.a == other.a && self.b == other.b)
            || (self.a == other.b && self.b == other.a);
    }
}

impl Ord for Jbtuple {
    fn cmp(&self, other: &Jbtuple) -> Ordering {
        if self.eq(other) {
            return Ordering::Equal;
        };
        let d1 = self.a.distance(&self.b);
        let d2 = other.a.distance(&other.b);
        return d1.cmp(&d2);
    }
}

impl PartialOrd for Jbtuple {
    fn partial_cmp(&self, other: &Jbtuple) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

#[derive(Debug)]
struct Inp {
    b: Vec<Jb>,
    distances: BTreeMap<Jbtuple, i64>,
}

impl Jb {
    fn distance(&self, other: &Jb) -> i64 {
        return (self.v[0] - other.v[0]) * (self.v[0] - other.v[0])
            + (self.v[1] - other.v[1]) * (self.v[1] - other.v[1])
            + (self.v[2] - other.v[2]) * (self.v[2] - other.v[2]);
    }
}

fn parse_line(line: &str, idx: usize) -> Jb {
    let mut res = Jb {
        v: [0; 3],
        idx: idx,
    };
    if line.len() == 0 {
        return res;
    }
    let v: Vec<&str> = line.split(",").collect();
    for i in 0..3 {
        res.v[i] = v[i].to_string().parse::<i64>().unwrap();
    }
    return res;
}

fn read_file(filename: &str) -> Inp {
    let lines = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut inp = Inp {
        b: Vec::new(),
        distances: BTreeMap::new(),
    };
    let mut idx = 0;
    for line in lines.lines() {
        inp.b.push(parse_line(line, idx));
        idx = idx + 1;
    }

    for b1 in inp.b.iter() {
        for b2 in inp.b.iter() {
            if b1 != b2 {
                inp.distances
                    .insert(Jbtuple { a: *b1, b: *b2 }, b1.distance(&b2));
            }
        }
    }

    return inp;
}

fn cmap_into_counters(cmap: &HashMap<usize, usize>) -> BTreeMap<usize, usize> {
    let mut count_rev: BTreeMap<usize, usize> = BTreeMap::new();
    for (_color, count) in cmap.iter() {
        count_rev
            .entry(2000 - *count)
            .and_modify(|ncolors| *ncolors += 1)
            .or_insert(1);
    }
    println!("count_rev: {:?}", count_rev);
    return count_rev;
}

fn multiplier(count_rev: &BTreeMap<usize, usize>) -> i64 {
    let mut res: i64 = 1_i64;
    let mut i = 0;
    for (rc, ncolors) in count_rev.iter() {
        for _j in 1..min(ncolors + 1, 4 - i) {
            res = res * (2000 - (*rc) as i64);
        }
        i = i + ncolors;
        if i > 3 {
            return res;
        };
    }
    return res;
}

fn countcolors(colors: Vec<usize>) -> i64 {
    let mut cmap: HashMap<usize, usize> = HashMap::new();
    for (_idx, c) in colors.iter().enumerate() {
        if *c > 0 {
            cmap.entry(*c).and_modify(|count| *count += 1).or_insert(1);
        }
    }
    println!("cmap: {:?}", cmap);
    let count_rev = cmap_into_counters(&cmap);
    return multiplier(&count_rev);
}

impl Inp {
    fn metrics(&self, maxcount: i64) -> i64 {
        let mut count: i64 = 0_i64;
        let mut maxcolor: usize = 0;
        let mut colors = vec![0; self.b.len()];
        for (jbt, d) in self.distances.iter() {
            println!("d: {:?} count: {:?}", d, count);
            //println!("jbt: {:?} d: {:?}", jbt, d);
            //println!("colors: {:?}", colors);
            if count >= maxcount {
                println!("{:?}", colors);
                return countcolors(colors);
            };
            let idx1 = jbt.a.idx;
            let idx2 = jbt.b.idx;
            if colors[idx1] == 0 && colors[idx2] == 0 {
                maxcolor = maxcolor + 1;
                println!("new color: {:?} for {:?} and {:?}", maxcolor, idx1, idx2);
                colors[idx1] = maxcolor;
                colors[idx2] = maxcolor;
                count = count + 1;
                continue;
            };
            if colors[idx1] == colors[idx2] {
                count = count + 1;
                println!("same color: {:?}", colors[idx1]);
                continue;
            };
            if colors[idx1] == 0 {
                println!("idx1 {:?} joins {:?}", idx1, colors[idx2]);
                colors[idx1] = colors[idx2];
                count = count + 1;
                continue;
            };
            if colors[idx2] == 0 {
                println!("idx2 {:?} joins {:?}", idx2, colors[idx1]);
                colors[idx2] = colors[idx1];
                count = count + 1;
                continue;
            };
            let oldc = colors[idx1];
            println!("color {:?} becomes {:?}", oldc, colors[idx2]);
            for i in 0..colors.len() {
                if colors[i] == oldc {
                    colors[i] = colors[idx2]
                };
            }
            count = count + 1;
        }
        println!("Final count: {:?}", count);
        return countcolors(colors);
    }
    fn is_last(&self, colors: &Vec<usize>) -> bool {
        if colors.len() == 0 {
            return true;
        };
        let c0 = colors[0];
        for c in colors.iter() {
            if *c != c0 {
                return false;
            };
        }
        return true;
    }
    fn countvalue2(&self, jbt: &Jbtuple) -> i64 {
        return jbt.a.v[0] * jbt.b.v[0];
    }
    fn metrics2(&self) -> i64 {
        let mut count: i64 = 0_i64;
        let mut maxcolor: usize = 0;
        let mut colors = vec![0; self.b.len()];
        for (jbt, d) in self.distances.iter() {
            println!("d: {:?} count: {:?}", d, count);
            //println!("jbt: {:?} d: {:?}", jbt, d);
            //println!("colors: {:?}", colors);
            let idx1 = jbt.a.idx;
            let idx2 = jbt.b.idx;
            if colors[idx1] == 0 && colors[idx2] == 0 {
                maxcolor = maxcolor + 1;
                println!("new color: {:?} for {:?} and {:?}", maxcolor, idx1, idx2);
                colors[idx1] = maxcolor;
                colors[idx2] = maxcolor;
                count = count + 1;
                if self.is_last(&colors) {
                    return self.countvalue2(jbt);
                }
                continue;
            };
            if colors[idx1] == colors[idx2] {
                count = count + 1;
                println!("same color: {:?}", colors[idx1]);
                continue;
            };
            if colors[idx1] == 0 {
                println!("idx1 {:?} joins {:?}", idx1, colors[idx2]);
                colors[idx1] = colors[idx2];
                count = count + 1;
                if self.is_last(&colors) {
                    return self.countvalue2(jbt);
                }
                continue;
            };
            if colors[idx2] == 0 {
                println!("idx2 {:?} joins {:?}", idx2, colors[idx1]);
                colors[idx2] = colors[idx1];
                count = count + 1;
                if self.is_last(&colors) {
                    return self.countvalue2(jbt);
                }
                continue;
            };
            let oldc = colors[idx1];
            println!("color {:?} becomes {:?}", oldc, colors[idx2]);
            for i in 0..colors.len() {
                if colors[i] == oldc {
                    colors[i] = colors[idx2]
                };
            }
            count = count + 1;
            if self.is_last(&colors) {
                return self.countvalue2(jbt);
            }
        }
        println!("Final count: {:?}", count);
        return countcolors(colors);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmap_into_counters() {
        let res1 = cmap_into_counters(&HashMap::from([(1, 2), (2, 3), (4, 2)]));
        assert_eq!(res1, BTreeMap::from([(2000 - 2, 2), (2000 - 3, 1)]));
    }

    #[test]
    fn test_multiplier() {
        assert_eq!(
            multiplier(&BTreeMap::from([(2000 - 2, 2), (2000 - 3, 1)])),
            4 * 3
        );
        assert_eq!(
            multiplier(&BTreeMap::from([
                (2000 - 5, 1),
                (2000 - 3, 1),
                (2000 - 2, 4)
            ])),
            5 * 3 * 2
        );
    }
    #[test]
    fn it_works() {
        assert_eq!(read_file("test.txt").metrics(10), 40);
        assert_eq!(read_file("test.txt").metrics2(), 25272);
    }
}

fn main() {
    let v = read_file("input.txt");
    println!("Part1 {:?}", v.metrics(1000));
    println!("Part2 {:?}", v.metrics2());
}
