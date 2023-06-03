#[cfg(feature = "time")]
use chrono::{DateTime, Utc};
#[cfg(feature = "time")]
use chrono::{TimeZone,Local};
#[cfg(feature = "time")]
use chrono_tz::Tz;
#[cfg(feature = "time")]
use chrono_tz::UTC;
#[cfg(feature = "trace")]
use tracing_subscriber::fmt::format::Writer;
#[cfg(feature = "trace")]
use tracing_subscriber::fmt::time::FormatTime;

#[cfg(feature = "time")]
pub fn timestamp_char17_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d%H%M%S%3f").to_string()
}

#[cfg(feature = "time")]
pub fn date_char6_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%y%m%d").to_string()
}

#[cfg(feature = "time")]
pub fn date_char8_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%y-%m-%d").to_string()
}

#[cfg(feature = "time")]
pub fn datetime_char14_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d%H%M%S").to_string()
}

#[cfg(feature = "time")]
pub struct LocalTimeFormatter;
#[cfg(feature = "time")]
impl FormatTime for LocalTimeFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

#[cfg(test)]
#[cfg(feature = "time")]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp_char17_zone() {
        let result = timestamp_char17_zone("Asia/Seoul");
        println!("test_timestamp_char17_zone: {}", result);
        assert_eq!(result.len(), 17);
    }

    #[test]
    fn test_date_char6_zone() {
        let result = date_char6_zone("Asia/Seoul");
        println!("test_date_char6_zone: {}", result);
        assert_eq!(result.len(), 6);
    }

    #[test]
    fn test_date_char8_zone() {
        let result = date_char8_zone("Asia/Seoul");
        println!("test_date_char8_zone: {}", result);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_datetime_char14_zone() {
        let result = datetime_char14_zone("Asia/Seoul");
        println!("test_datetime_char14_zone: {}", result);
        assert_eq!(result.len(), 14);
    }
}