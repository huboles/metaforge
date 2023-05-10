use color_eyre::Result;
use metaforge::parse_opts;

fn main() -> Result<()> {
    let opts = parse_opts()?;
    println!("{:?}", opts);
    Ok(())
}
