use std::error::Error;
use std::io::BufRead;
use std::io::Write;

pub fn preprocess<R: BufRead, W: Write>(r: &mut R, w: &mut W) -> Result<(), Box<dyn Error>> {
    write!(w, "echo \"")?;

    let mut echo = true;
    for line in r.lines().flatten() {
        match line.trim() {
            "#!" => {
                echo = !echo;
                match echo {
                    true => write!(w, "\necho \""),
                    false => writeln!(w, "\""),
                }
            }
            s => writeln!(w, "{}", s),
        }?;
    }

    if echo {
        write!(w, "\"")?;
    }

    Ok(())
}
