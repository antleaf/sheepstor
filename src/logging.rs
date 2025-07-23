use flexi_logger::{style, DeferredNow};
use log::Record;

pub fn configure_flexi_logger(debug: bool) -> Result<(), Box<dyn std::error::Error>> {
    let logging_level = if debug { "debug" } else { "info" };

    flexi_logger::Logger::try_with_str(logging_level)?
        .log_to_stdout()
        .set_palette("1;5;32;3;-".parse().unwrap())
        // .format(flexi_logger::colored_opt_format)
        .format(my_own_format)
        .start()?;

    Ok(())
}


pub fn my_own_format(
    w: &mut dyn std::io::Write,
    _now: &mut DeferredNow,
    record: &Record,
) -> Result<(), std::io::Error> {
    let level = record.level();
    write!(
        w,
        "{}: {}:{} ",
        style(level).paint(level.to_string()),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
    )?;
    write!(w, "{}", record.args().to_string())
}
