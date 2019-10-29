use std::str::FromStr;

use super::settings;

pub fn setup(logging: &settings::Logging) -> Result<(), fern::InitError> {
    let log_level = log::LevelFilter::from_str(&logging.level).unwrap_or(log::LevelFilter::Info);

    let colors = fern::colors::ColoredLevelConfig::new()
        .debug(fern::colors::Color::Magenta)
        .trace(fern::colors::Color::Blue)
        .info(fern::colors::Color::Green)
        .warn(fern::colors::Color::Yellow)
        .error(fern::colors::Color::Red);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log_level)
        .level_for("winit", log::LevelFilter::Warn)
        .level_for("gfx_device_gl", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file(&logging.file)?)
        .apply()?;
    Ok(())
}
