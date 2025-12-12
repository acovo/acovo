//! Test file for time module functionality

#[cfg(test)]
mod tests {
    #[test]
    fn test_time_functions() {
        // Test that time functions work when the time feature is enabled
        #[cfg(feature = "time")]
        {
            use acovo::time::*;
            
            // Test timestamp generation
            let timestamp = timestamp_char17_zone("Asia/Seoul");
            assert_eq!(timestamp.len(), 17);
            
            // Test date generation
            let date6 = date_char6_zone("Asia/Seoul");
            assert_eq!(date6.len(), 6);
            
            let date8 = date_char8_zone("Asia/Seoul");
            assert_eq!(date8.len(), 8);
            
            let date8_2 = date_char8_zone2("Asia/Seoul");
            assert_eq!(date8_2.len(), 8);
            
            let datetime14 = datetime_char14_zone("Asia/Seoul");
            assert_eq!(datetime14.len(), 14);
            
            // Test timestamp parsing
            let parsed = timestamp_from_char14("20231231235959");
            assert!(parsed.is_ok());
            
            // Test invalid timestamp parsing
            let invalid = timestamp_from_char14("invalid");
            assert!(invalid.is_err());
            
            println!("Basic time tests passed!");
        }
    }

    #[test]
    fn test_time_functions_comprehensive() {
        // More comprehensive tests for time functions
        #[cfg(feature = "time")]
        {
            use acovo::time::*;

            // Test various timezones
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

                let date6 = date_char6_zone(tz);
                assert_eq!(date6.len(), 6);

                let date8 = date_char8_zone(tz);
                assert_eq!(date8.len(), 8);

                let date8_2 = date_char8_zone2(tz);
                assert_eq!(date8_2.len(), 8);

                let datetime14 = datetime_char14_zone(tz);
                assert_eq!(datetime14.len(), 14);
            }

            println!("Comprehensive time tests passed!");
        }
    }

    #[test]
    fn test_timestamp_parsing_edge_cases() {
        // Test edge cases for timestamp parsing
        #[cfg(feature = "time")]
        {
            use acovo::time::*;

            // Test leap year date
            let parsed = timestamp_from_char14("20240229000000");
            assert!(parsed.is_ok());

            // Test invalid dates
            let invalid_feb = timestamp_from_char14("20230229000000"); // Invalid Feb 29 in non-leap year
            assert!(invalid_feb.is_err());

            let invalid_month = timestamp_from_char14("20231301000000"); // Invalid month
            assert!(invalid_month.is_err());

            let invalid_day = timestamp_from_char14("20231232000000"); // Invalid day
            assert!(invalid_day.is_err());

            let invalid_hour = timestamp_from_char14("20231231250000"); // Invalid hour
            assert!(invalid_hour.is_err());

            println!("Timestamp parsing edge case tests passed!");
        }
    }

    #[test]
    fn test_date_format_validation() {
        // Test date format validation
        #[cfg(feature = "time")]
        {
            use acovo::time::*;

            // Test that date_char8_zone follows YY-MM-DD format
            let date8 = date_char8_zone("UTC");
            assert_eq!(date8.len(), 8);
            assert_eq!(date8.chars().nth(2), Some('-'));
            assert_eq!(date8.chars().nth(5), Some('-'));

            // Test that date_char8_zone2 follows YYYYMMDD format and contains only digits
            let date8_2 = date_char8_zone2("UTC");
            assert_eq!(date8_2.len(), 8);
            assert!(date8_2.chars().all(|c| c.is_ascii_digit()));

            println!("Date format validation tests passed!");
        }
    }
}