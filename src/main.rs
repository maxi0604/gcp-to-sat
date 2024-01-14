use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::fmt::Write;
use std::io::{self, prelude::*, BufReader};
fn udx(dx: u64, idx: u64) -> (usize, usize) {
    ((idx % dx) as usize, (idx / dx) as usize)
}

fn idx(dx: usize, x: usize, y: usize) -> usize {
    dx * y + x
}

fn print(vec: &Vec<u64>, n: usize) {
    for i in 0..n {
        println!("{}", vec[(i * n)..((i + 1) * n)].iter().map(|x| x.to_string()).fold(String::new(), |a, b| a + &b))
    }
}

fn check(arr: &Vec<u64>, dx: usize) -> Option<((usize, usize), (usize, usize))> {
    for i in 0..arr.len() {
        let (x2, y2) = udx(dx as u64, i as u64);
        for y1 in 0..y2 {
            for x1 in 0..x2 {
                let col0 = arr[idx(dx, x1, y1)];
                let col1 = arr[idx(dx, x1, y2)];
                let col2 = arr[idx(dx, x2, y1)];
                let col3 = arr[idx(dx, x2, y2)];

                if col0 == col1 && col1 == col2 && col2 == col3 {
                    return Some(((x1, y1), (x2, y2)));
                }
            }
        }
    }
    None
}

fn parse(file: String, n: u64) -> io::Result<Vec<u64>> {
    let inf = File::open(file)?;
    let buf = BufReader::new(inf);
    let soln = buf.lines().nth(1).unwrap()?;
    let mut res = vec![0; (n * n) as usize];
    let pos = soln.split(" ").map(|x| x.parse::<isize>().unwrap()).filter(|x| *x > 0);
    for i in pos {
        let i = i as usize;
        let c = (i - 1) % 4 + 1;
        let idx = (i - 1) / 4;
        res[idx] = c as u64;
    }
    Ok(res)
}

fn main() -> Result<(),Box<dyn Error>>{
    let mut should_parse = false;
    if let Some(arg) = env::args().nth(1) {
        should_parse = (arg == "parse");
    }

    if should_parse {
        let n: u64 = env::args().nth(3).unwrap().parse()?;
        let res = parse(env::args().nth(2).unwrap(), n)?;
        print(&res, n.try_into().unwrap());
        if let Some(((x1, x2), (y1, y2))) = check(&res, n as usize) {
            println!("Incorrect solution. Rect given by ({x1}, {y1}), ({x2}, {y2}) unanimously has color {}", res[idx(n as usize, x2, y2)]);
        }
        else {
            println!("Seems alright");
        }
        return Ok(());
    }

    let n: i64 = env::args().nth(1).unwrap().parse()?;
    let out = File::create(format!("gcp{n}.cnf"))?;
    let mut outf = BufWriter::new(out);
    let mut w = String::new();
    let mut clauses: u64 = 0;

    writeln!(&mut w, "c GCP for {n}")?;

    writeln!(&mut w, "c At least one")?;
    for y in 0..n {
        for x in 0..n {
            for c in 1..=4 {
                let k = 4 * y * n + 4 * x + c;
                write!(&mut w, "{k} ")?;
            }
            clauses += 1;
            writeln!(&mut w, "0")?;
        }
    }

    writeln!(&mut w, "c At most one")?;
    for y in 0..n {
        for x in 0..n {
            for c1 in 1..=4 {
                for c2 in (c1 + 1)..=4 {
                    let k1 = -(4 * y * n + 4 * x + c1);
                    let k2 = -(4 * y * n + 4 * x + c2);

                    writeln!(&mut w, "{k1} {k2} 0")?;
                    clauses += 1;
                }
            }
        }
    }

    writeln!(&mut w, "c The actual thing")?;
    for y1 in 0..(n - 1) {
        for x1 in 0..(n - 1) {
            for y2 in (y1 + 1)..n {
                for x2 in (x1 + 1)..n {
                    for c in 1..=4 {
                        let k1 = -(4 * y1 * n + 4 * x1 + c);
                        let k2 = -(4 * y1 * n + 4 * x2 + c);
                        let k3 = -(4 * y2 * n + 4 * x1 + c);
                        let k4 = -(4 * y2 * n + 4 * x2 + c);
                        writeln!(&mut w, "{k1} {k2} {k3} {k4} 0")?;
                        clauses += 1;
                    }
                }
            }
        }
    }
    {
        use std::io::Write;
        writeln!(&mut outf, "p cnf {} {}", 4 * n * n, clauses)?;
        write!(&mut outf, "{w}")?;
    }
    // println!("{w}");
    Ok(())
}
