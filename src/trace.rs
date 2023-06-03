#[cfg(feature = "trace")]
use crate::time::LocalTimeFormatter;
#[cfg(feature = "trace")]
use crate::fs::get_exe_dir;
#[cfg(feature = "trace")]
use std::io;
#[cfg(feature = "trace")]
use tracing::*;
use tracing_appender::non_blocking::NonBlocking;

#[cfg(feature = "trace")]
#[macro_export]
macro_rules! init_tracing {
       ($e:expr) => {
          if std::env::var_os("RUST_LOG").is_none() {
                std::env::set_var("RUST_LOG", "poem=debug");
          }
          let log_path = format!("{}/logs", get_exe_dir());
          let file_appender = tracing_appender::rolling::daily(log_path, $e);
          let format = tracing_subscriber::fmt::format()
            .with_level(true)
            .with_target(true)
            .with_timer(LocalTimeFormatter);
           let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
           tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .with_writer(io::stdout)
            .with_writer(non_blocking)
            .with_ansi(true)
            .event_format(format)
            .init();
       };
    }

#[cfg(test)]
#[cfg(feature = "trace")]
mod tests {
    use super::*;

    #[test]
    fn test_init_log() {
        init_tracing!("Seoul");
    }
}