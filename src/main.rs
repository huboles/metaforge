fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let opts = metaforge::get_opts()?;

    metaforge::build_dir(&opts)
}
