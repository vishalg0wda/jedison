// illustrating usage of iter.
//

struct Words<T> {
    type Item: T
    content: T
}

fn main() -> Result<(), &'static str>{
    "asd".split()
    let stdin = std::io::stdin();
    for line in stdin.lines() {
        dbg!(line).map_err(|_| "error reading line-number")?;
    }
    Ok(())
}
