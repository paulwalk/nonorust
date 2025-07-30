use flexi_logger::{style, DeferredNow};
use log::Record;

pub fn configure_flexi_logger(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let logging_level = if debug { "debug" } else { "info" };
    flexi_logger::Logger::try_with_str(logging_level)?
        .log_to_stdout()
        .set_palette("1;5;32;3;-".parse()?)
        .format(custom_logging_format)
        .start()?;
    Ok(())
}

pub fn custom_logging_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{}: {}:{} ",
        style(level).paint(level.to_string()),
        style(level).paint(record.file().unwrap_or("<unnamed>")),
        style(level).paint(record.line().unwrap_or(0).to_string()),
    )?;
    write!(w, "{}", record.args())
}