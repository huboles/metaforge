fn main() -> eyre::Result<()> {
    let opts = metaforge::get_opts()?;

    if opts.new {
        return metaforge::new_site(&opts);
    }

    if let Some(_) = &opts.file {
        let str = metaforge::single_file(&opts)?;
        println!("{str}");
        Ok(())
    } else {
        return metaforge::build_site(&opts);
    }
}
