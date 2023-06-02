#[cfg(feature = "trace")]
use crate::time::LocalTimeFormatter;
#[cfg(feature = "trace")]
use crate::fs::get_exe_dir;
#[cfg(feature = "trace")]
use std::io;
#[cfg(feature = "trace")]
use tracing::*;

#[cfg(feature = "trace")]
pub fn init_log(filename:&str) {

    let log_path = format!("{}/logs", get_exe_dir());
    let file_appender = tracing_appender::rolling::daily(log_path, filename);
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let format = tracing_subscriber::fmt::format()
        .with_level(true)
        .with_target(true)
        .with_timer(LocalTimeFormatter);

    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_writer(io::stdout)
        .with_writer(non_blocking)
        .with_ansi(true)
        .event_format(format)
        .init();
}

#[cfg(test)]
#[cfg(feature = "trace")]
mod tests {
    use super::*;

    #[test]
    fn test_init_log() {
        init_log("Seoul");
    }
}