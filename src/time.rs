#[cfg(feature = "time")]
use chrono::{Local, Utc};

#[cfg(feature = "time")]
use chrono::TimeZone;
#[cfg(feature = "time")]
use chrono_tz::Tz;

#[cfg(feature = "trace")]
use tracing_subscriber::fmt::{format::Writer, time::FormatTime};

use anyhow::{anyhow, Result as AnyResult};
use chrono::DateTime;

/// Generate a 17-character timestamp string with milliseconds in specified timezone
/// Format: YYYYMMDDHHMMSSmmm (e.g., 20231231235959999)
/// 
/// # Arguments
/// * `zone_name` - A string slice that holds the timezone name (e.g., "Asia/Seoul")
/// 
/// # Returns
/// * A String containing the formatted timestamp
#[cfg(feature = "time")]
pub fn timestamp_char17_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d%H%M%S%3f").to_string()
}

/// Generate a 6-character date string in specified timezone
/// Format: YYMMDD (e.g., 231231)
/// 
/// # Arguments
/// * `zone_name` - A string slice that holds the timezone name (e.g., "Asia/Seoul")
/// 
/// # Returns
/// * A String containing the formatted date
#[cfg(feature = "time")]
pub fn date_char6_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%y%m%d").to_string()
}

/// Generate an 8-character date string with dashes in specified timezone
/// Format: YY-MM-DD (e.g., 23-12-31)
/// 
/// # Arguments
/// * `zone_name` - A string slice that holds the timezone name (e.g., "Asia/Seoul")
/// 
/// # Returns
/// * A String containing the formatted date
#[cfg(feature = "time")]
pub fn date_char8_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%y-%m-%d").to_string()
}

/// Generate an 8-character date string without dashes in specified timezone
/// Format: YYYYMMDD (e.g., 20231231)
/// 
/// # Arguments
/// * `zone_name` - A string slice that holds the timezone name (e.g., "Asia/Seoul")
/// 
/// # Returns
/// * A String containing the formatted date
#[cfg(feature = "time")]
pub fn date_char8_zone2(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d").to_string()
}

/// Generate a 14-character datetime string in specified timezone
/// Format: YYYYMMDDHHMMSS (e.g., 20231231235959)
/// 
/// # Arguments
/// * `zone_name` - A string slice that holds the timezone name (e.g., "Asia/Seoul")
/// 
/// # Returns
/// * A String containing the formatted datetime
#[cfg(feature = "time")]
pub fn datetime_char14_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d%H%M%S").to_string()
}

/// A formatter for local time that implements the FormatTime trait
/// Used for formatting timestamps in tracing subscriber
#[cfg(feature = "time")]
pub struct LocalTimeFormatter;

/// Implementation of FormatTime for LocalTimeFormatter
/// Formats time as ISO 8601 with milliseconds (e.g., 2023-12-31T23:59:59.999)
#[cfg(feature = "time")]
impl FormatTime for LocalTimeFormatter {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%FT%T%.3f"))
    }
}

/// Parse a 14-character timestamp string into a DateTime<Local>
/// Expected format: YYYYMMDDHHMMSS (e.g., 20231231235959)
/// 
/// # Arguments
/// * `timestamp` - A string slice containing the 14-character timestamp
/// 
/// # Returns
/// * A Result containing either the parsed DateTime<Local> or an error
/// 
/// # Errors
/// * Returns an error if the timestamp string is not in the expected format
#[cfg(feature = "time")]
pub fn timestamp_from_char14(timestamp: &str) -> AnyResult<DateTime<Local>> {
    match Local.datetime_from_str(timestamp, "%Y%m%d%H%M%S") {
        Ok(dt) => Ok(dt),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

#[cfg(test)]
#[cfg(feature = "time")]
mod tests {
    use super::*;
    use chrono::Datelike;
    use chrono::Timelike;

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
    fn test_date_char8_zone2() {
        let result = date_char8_zone2("Asia/Seoul");
        println!("test_date_char8_zone2: {}", result);
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_datetime_char14_zone() {
        let result = datetime_char14_zone("Asia/Seoul");
        println!("test_datetime_char14_zone: {}", result);
        assert_eq!(result.len(), 14);
    }

    #[test]
    fn test_timestamp_from_char14_valid() {
        // Test with a valid timestamp
        let result = timestamp_from_char14("20231231235959");
        assert!(result.is_ok());
        let dt = result.unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 12);
        assert_eq!(dt.day(), 31);
        assert_eq!(dt.hour(), 23);
        assert_eq!(dt.minute(), 59);
        assert_eq!(dt.second(), 59);
    }

    #[test]
    fn test_timestamp_from_char14_invalid() {
        // Test with an invalid timestamp
        let result = timestamp_from_char14("invalid");
        assert!(result.is_err());
        
        // Test with a timestamp that has wrong length
        let result = timestamp_from_char14("202312312359");
        assert!(result.is_err());
    }

    #[test]
    fn test_different_timezones() {
        // Test that different timezones produce different results
        let seoul_time = timestamp_char17_zone("Asia/Seoul");
        let utc_time = timestamp_char17_zone("UTC");
        // They might be the same if executed at specific moments, but generally should differ
        println!("Seoul time: {}", seoul_time);
        println!("UTC time: {}", utc_time);
        assert_eq!(seoul_time.len(), 17);
        assert_eq!(utc_time.len(), 17);
    }

    #[test]
    fn test_local_time_formatter() {
        // Test that we can create a LocalTimeFormatter instance
        let formatter = LocalTimeFormatter;
        // We can't easily test the actual formatting without capturing stdout,
        // but we can ensure the struct can be instantiated
        assert!(true); // Placeholder assertion
    }

    #[test]
    fn test_timestamp_char17_zone_length() {
        // Test that the timestamp is always 17 characters long
        let timestamp = timestamp_char17_zone("UTC");
        assert_eq!(timestamp.len(), 17);
        
        let timestamp = timestamp_char17_zone("America/New_York");
        assert_eq!(timestamp.len(), 17);
    }

    #[test]
    fn test_date_char6_zone_length() {
        // Test that the date is always 6 characters long
        let date = date_char6_zone("UTC");
        assert_eq!(date.len(), 6);
        
        let date = date_char6_zone("Europe/London");
        assert_eq!(date.len(), 6);
    }

    #[test]
    fn test_date_char8_zone_format() {
        // Test that the date follows YY-MM-DD format
        let date = date_char8_zone("UTC");
        assert_eq!(date.len(), 8);
        assert_eq!(date.chars().nth(2), Some('-'));
        assert_eq!(date.chars().nth(5), Some('-'));
    }

    #[test]
    fn test_date_char8_zone2_format() {
        // Test that the date follows YYYYMMDD format
        let date = date_char8_zone2("UTC");
        assert_eq!(date.len(), 8);
        // Should only contain digits
        assert!(date.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_datetime_char14_zone_length() {
        // Test that the datetime is always 14 characters long
        let datetime = datetime_char14_zone("UTC");
        assert_eq!(datetime.len(), 14);
        
        let datetime = datetime_char14_zone("Asia/Tokyo");
        assert_eq!(datetime.len(), 14);
    }

    #[test]
    fn test_timestamp_from_char14_edge_cases() {
        // Test with leap year date
        let result = timestamp_from_char14("20240229000000");
        assert!(result.is_ok());
        
        // Test with invalid dates
        let result = timestamp_from_char14("20230229000000"); // Invalid Feb 29 in non-leap year
        assert!(result.is_err());
        
        let result = timestamp_from_char14("20231301000000"); // Invalid month
        assert!(result.is_err());
        
        let result = timestamp_from_char14("20231232000000"); // Invalid day
        assert!(result.is_err());
        
        let result = timestamp_from_char14("20231231250000"); // Invalid hour
        assert!(result.is_err());
    }

    #[test]
    fn test_various_timezones() {
        // Test various common timezones
        let timezones = vec![
            "UTC",
            "America/New_York",
            "Europe/London",
            "Asia/Tokyo",
            "Australia/Sydney",
        ];
        
        for tz in timezones {
            let timestamp = timestamp_char17_zone(tz);
            assert_eq!(timestamp.len(), 17);
        }
    }
}
