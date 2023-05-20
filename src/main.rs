fn main() -> eyre::Result<()> {
    let opts = metaforge::get_opts()?;

    if opts.new {
        return metaforge::new_site(&opts);
    }

    if opts.file.is_some() {
        let str = metaforge::single_file(&opts)?;
        println!("{str}");
        Ok(())
    } else {
        metaforge::build_site(&opts)
    }
}
