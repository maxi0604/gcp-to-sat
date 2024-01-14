use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::fmt::Write;

fn main() -> Result<(),Box<dyn Error>>{
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
