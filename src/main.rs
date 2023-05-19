fn main() -> eyre::Result<()> {
    let opts = metaforge::get_opts()?;

    if opts.new {
        metaforge::new_site(&opts)
    } else {
        metaforge::build_dir(&opts)
    }
}
