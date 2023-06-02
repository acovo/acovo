#[cfg(feature = "time")]
use chrono::{DateTime, Utc};
#[cfg(feature = "time")]
use chrono::TimeZone;
#[cfg(feature = "time")]
use chrono_tz::Tz;
#[cfg(feature = "time")]
use chrono_tz::UTC;

#[cfg(feature = "time")]
pub fn timestamp_char17_zone(zone_name: &str) -> String {
    let tz: Tz = zone_name.parse().unwrap();
    let now = Utc::now().with_timezone(&tz);
    now.format("%Y%m%d%H%M%S%3f").to_string()
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
}