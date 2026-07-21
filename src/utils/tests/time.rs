#[cfg(test)]
mod tests {
    use crate::utils;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    #[test]
    fn test_now_epoch_ms_is_nonzero_and_close_to_system_time() {
        let ms = utils::time::now_epoch_ms();
        assert!(ms > 0);

        let sys_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let diff = sys_ms.abs_diff(ms);
        assert!(diff <= 2_000);
    }

    #[test]
    fn test_civil_from_days_epoch_day_is_1970_01_01() {
        let (y, m, d) = utils::time::civil_from_days(0);
        assert_eq!((y, m, d), (1970, 1, 1));
    }

    #[test]
    fn test_civil_from_days_known_dates() {
        assert_eq!(utils::time::civil_from_days(1), (1970, 1, 2));
        assert_eq!(utils::time::civil_from_days(-1), (1969, 12, 31));
    }

    #[test]
    fn test_format_unix_timestamp_epoch() {
        assert_eq!(utils::time::format_unix_timestamp(0), "01/01/1970-00:00:00");
    }

    #[test]
    fn test_format_unix_timestamp_one_day_plus_one_second() {
        assert_eq!(
            utils::time::format_unix_timestamp(86_400 + 1),
            "01/02/1970-00:00:01"
        );
    }

    #[test]
    fn test_format_unix_timestamp_end_of_first_day() {
        assert_eq!(
            utils::time::format_unix_timestamp(86_399),
            "01/01/1970-23:59:59"
        );
    }

    #[test]
    fn test_format_unix_timestamp_known_reference_2000_01_01() {
        assert_eq!(
            utils::time::format_unix_timestamp(946_684_800),
            "01/01/2000-00:00:00"
        );
    }

    #[test]
    fn test_format_unix_timestamp_leap_day_2000_02_29() {
        assert_eq!(
            utils::time::format_unix_timestamp(951_782_400),
            "02/29/2000-00:00:00"
        );
    }

    #[test]
    fn test_format_system_time_epoch() {
        let t = UNIX_EPOCH;
        assert_eq!(utils::time::format_system_time(t), "01/01/1970-00:00:00");
    }

    #[test]
    fn test_format_system_time_before_epoch_returns_epoch() {
        let t = UNIX_EPOCH - Duration::from_secs(1);
        assert_eq!(utils::time::format_system_time(t), "01/01/1970-00:00:00");
    }

    #[test]
    fn test_now_unix_nano_is_numeric_and_increases() {
        let n1 = utils::time::now_unix_nano();
        let n2 = utils::time::now_unix_nano();

        let a: u128 = n1.parse().expect("now_unix_nano should be numeric");
        let b: u128 = n2.parse().expect("now_unix_nano should be numeric");

        assert!(b >= a);
    }
}
