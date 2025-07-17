
pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}]",
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::fs::File::create("output.log")?)
        .apply()?;
    Ok(())
}
