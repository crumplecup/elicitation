//! Parity tests: shadow types are drop-in replacements for upstream chrono types.
//!
//! Each test performs the same operation on both the upstream `chrono` type and the
//! `elicit_chrono` shadow, then asserts the results are identical.  Conversion
//! round-trips (`chrono → shadow → chrono`) additionally verify that no information
//! is lost when crossing the boundary.

use chrono::{Datelike, Timelike};

// ── NaiveDate ─────────────────────────────────────────────────────────────────

#[test]
fn naive_date_construction_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_d = chrono::NaiveDate::from_ymd_opt(2024, 6, 15).ok_or("invalid date")?;
    let shadow_d = elicit_chrono::NaiveDate::from_ymd_opt(2024, 6, 15).ok_or("invalid date")?;

    assert_eq!(shadow_d.year(), chrono_d.year());
    assert_eq!(shadow_d.month(), chrono_d.month());
    assert_eq!(shadow_d.day(), chrono_d.day());
    assert_eq!(shadow_d.ordinal(), chrono_d.ordinal());
    assert_eq!(shadow_d.num_days_from_ce(), chrono_d.num_days_from_ce());
    Ok(())
}

#[test]
fn naive_date_format_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_d = chrono::NaiveDate::from_ymd_opt(2024, 3, 21).ok_or("invalid date")?;
    let shadow_d = elicit_chrono::NaiveDate::from_ymd_opt(2024, 3, 21).ok_or("invalid date")?;

    let fmt = "%Y-%m-%d";
    assert_eq!(
        shadow_d.format(fmt.to_string()),
        chrono_d.format(fmt).to_string()
    );
    Ok(())
}

#[test]
fn naive_date_succ_and_pred_parity() -> Result<(), Box<dyn std::error::Error>> {
    // Feb 29 on a leap year: succ crosses month boundary, pred stays in Feb
    let chrono_d = chrono::NaiveDate::from_ymd_opt(2024, 2, 29).ok_or("invalid date")?;
    let shadow_d = elicit_chrono::NaiveDate::from_ymd_opt(2024, 2, 29).ok_or("invalid date")?;

    let cs = chrono_d.succ_opt().ok_or("chrono succ overflow")?;
    let ss = shadow_d.succ_opt().ok_or("shadow succ overflow")?;
    assert_eq!(ss.month(), cs.month());
    assert_eq!(ss.day(), cs.day());

    let cp = chrono_d.pred_opt().ok_or("chrono pred overflow")?;
    let sp = shadow_d.pred_opt().ok_or("shadow pred overflow")?;
    assert_eq!(sp.month(), cp.month());
    assert_eq!(sp.day(), cp.day());
    Ok(())
}

#[test]
fn naive_date_add_signed_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_base = chrono::NaiveDate::from_ymd_opt(2024, 1, 1).ok_or("invalid date")?;
    let shadow_base = elicit_chrono::NaiveDate::from_ymd_opt(2024, 1, 1).ok_or("invalid date")?;

    let days: i64 = 100;
    let chrono_delta = chrono::TimeDelta::days(days);
    // checked_add_signed takes Duration (shadow for chrono::Duration ≡ chrono::TimeDelta)
    let shadow_delta = elicit_chrono::Duration::try_days(days).ok_or("invalid delta")?;

    let cn = chrono_base.checked_add_signed(chrono_delta).ok_or("overflow")?;
    let sn = shadow_base.checked_add_signed(shadow_delta).ok_or("overflow")?;
    assert_eq!(sn.year(), cn.year());
    assert_eq!(sn.month(), cn.month());
    assert_eq!(sn.day(), cn.day());
    Ok(())
}

#[test]
fn naive_date_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original = chrono::NaiveDate::from_ymd_opt(2000, 12, 31).ok_or("invalid date")?;
    let shadow = elicit_chrono::NaiveDate::from(original);
    // Deref gives back &chrono::NaiveDate; clone restores ownership
    let restored: chrono::NaiveDate = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── NaiveDateTime ─────────────────────────────────────────────────────────────

#[test]
fn naive_datetime_parse_parity() -> Result<(), Box<dyn std::error::Error>> {
    let s = "2024-06-15 14:30:45";
    let fmt = "%Y-%m-%d %H:%M:%S";
    let chrono_dt =
        chrono::NaiveDateTime::parse_from_str(s, fmt).map_err(|e| e.to_string())?;
    let shadow_dt =
        elicit_chrono::NaiveDateTime::parse_from_str(s, fmt).ok_or("shadow parse failed")?;

    assert_eq!(shadow_dt.year(), chrono_dt.year());
    assert_eq!(shadow_dt.month(), chrono_dt.month());
    assert_eq!(shadow_dt.day(), chrono_dt.day());
    assert_eq!(shadow_dt.hour(), chrono_dt.hour());
    assert_eq!(shadow_dt.minute(), chrono_dt.minute());
    assert_eq!(shadow_dt.second(), chrono_dt.second());
    assert_eq!(shadow_dt.timestamp(), chrono_dt.and_utc().timestamp());
    Ok(())
}

#[test]
fn naive_datetime_format_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_dt =
        chrono::NaiveDateTime::parse_from_str("2024-12-25 08:00:00", "%Y-%m-%d %H:%M:%S")
            .map_err(|e| e.to_string())?;
    let shadow_dt =
        elicit_chrono::NaiveDateTime::parse_from_str("2024-12-25 08:00:00", "%Y-%m-%d %H:%M:%S")
            .ok_or("shadow parse failed")?;

    let fmt = "%d/%m/%Y %H:%M";
    assert_eq!(
        shadow_dt.format(fmt.to_string()),
        chrono_dt.format(fmt).to_string()
    );
    Ok(())
}

#[test]
fn naive_datetime_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original =
        chrono::NaiveDateTime::parse_from_str("2024-07-04 23:59:59", "%Y-%m-%d %H:%M:%S")
            .map_err(|e| e.to_string())?;
    let shadow = elicit_chrono::NaiveDateTime::from(original);
    let restored: chrono::NaiveDateTime = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── NaiveTime ─────────────────────────────────────────────────────────────────

#[test]
fn naive_time_construction_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_t = chrono::NaiveTime::from_hms_opt(14, 30, 45).ok_or("invalid time")?;
    let shadow_t =
        elicit_chrono::NaiveTime::from_hms_opt(14, 30, 45).ok_or("invalid time")?;

    assert_eq!(shadow_t.hour(), chrono_t.hour());
    assert_eq!(shadow_t.minute(), chrono_t.minute());
    assert_eq!(shadow_t.second(), chrono_t.second());
    assert_eq!(
        shadow_t.num_seconds_from_midnight(),
        chrono_t.num_seconds_from_midnight()
    );
    Ok(())
}

#[test]
fn naive_time_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original = chrono::NaiveTime::from_hms_opt(23, 59, 59).ok_or("invalid time")?;
    let shadow = elicit_chrono::NaiveTime::from(original);
    let restored: chrono::NaiveTime = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── TimeDelta ─────────────────────────────────────────────────────────────────

#[test]
fn time_delta_seconds_parity() -> Result<(), Box<dyn std::error::Error>> {
    let secs: i64 = 3661; // 1h 1m 1s
    let chrono_d = chrono::TimeDelta::seconds(secs);
    let shadow_d = elicit_chrono::TimeDelta::seconds(secs).ok_or("invalid delta")?;

    assert_eq!(shadow_d.num_seconds(), chrono_d.num_seconds());
    assert_eq!(shadow_d.num_minutes(), chrono_d.num_minutes());
    assert_eq!(shadow_d.num_hours(), chrono_d.num_hours());
    Ok(())
}

#[test]
fn time_delta_days_parity() -> Result<(), Box<dyn std::error::Error>> {
    let chrono_d = chrono::TimeDelta::days(7);
    let shadow_d = elicit_chrono::TimeDelta::days(7).ok_or("invalid delta")?;

    assert_eq!(shadow_d.num_days(), chrono_d.num_days());
    assert_eq!(shadow_d.num_hours(), chrono_d.num_hours());
    assert_eq!(shadow_d.num_seconds(), chrono_d.num_seconds());
    Ok(())
}

#[test]
fn time_delta_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original = chrono::TimeDelta::hours(48);
    let shadow = elicit_chrono::TimeDelta::from(original);
    let restored: chrono::TimeDelta = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── FixedOffset ───────────────────────────────────────────────────────────────

#[test]
fn fixed_offset_east_parity() -> Result<(), Box<dyn std::error::Error>> {
    let secs = 5 * 3600 + 30 * 60; // UTC+05:30 (India)
    let chrono_o = chrono::FixedOffset::east_opt(secs).ok_or("invalid offset")?;
    let shadow_o = elicit_chrono::FixedOffset::east_opt(secs).ok_or("invalid offset")?;

    assert_eq!(shadow_o.local_minus_utc(), chrono_o.local_minus_utc());
    assert_eq!(shadow_o.utc_minus_local(), -chrono_o.local_minus_utc());
    Ok(())
}

#[test]
fn fixed_offset_west_parity() -> Result<(), Box<dyn std::error::Error>> {
    let secs = 5 * 3600; // UTC-05:00 (US Eastern)
    let chrono_o = chrono::FixedOffset::west_opt(secs).ok_or("invalid offset")?;
    let shadow_o = elicit_chrono::FixedOffset::west_opt(secs).ok_or("invalid offset")?;

    assert_eq!(shadow_o.local_minus_utc(), chrono_o.local_minus_utc());
    Ok(())
}

#[test]
fn fixed_offset_display_parity() -> Result<(), Box<dyn std::error::Error>> {
    let secs = 5 * 3600 + 30 * 60;
    let chrono_o = chrono::FixedOffset::east_opt(secs).ok_or("invalid offset")?;
    let shadow_o = elicit_chrono::FixedOffset::east_opt(secs).ok_or("invalid offset")?;

    assert_eq!(shadow_o.to_string(), chrono_o.to_string());
    Ok(())
}

#[test]
fn fixed_offset_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let secs = 9 * 3600; // UTC+09:00 (Japan)
    let original = chrono::FixedOffset::east_opt(secs).ok_or("invalid offset")?;
    let shadow = elicit_chrono::FixedOffset::from(original);
    // FixedOffset is Copy (not Arc-wrapped); recover chrono::FixedOffset via local_minus_utc
    let restored =
        chrono::FixedOffset::east_opt(shadow.local_minus_utc()).ok_or("restore failed")?;
    assert_eq!(restored, original);
    Ok(())
}

// ── DateTime (UTC) ────────────────────────────────────────────────────────────

#[test]
fn datetime_utc_parse_parity() -> Result<(), Box<dyn std::error::Error>> {
    let s = "2024-01-15T12:30:00Z";
    let chrono_dt: chrono::DateTime<chrono::Utc> =
        s.parse().map_err(|e: chrono::ParseError| e.to_string())?;
    let shadow_dt = elicit_chrono::DateTime::parse(s).ok_or("shadow parse failed")?;

    assert_eq!(shadow_dt.year(), chrono_dt.year());
    assert_eq!(shadow_dt.month(), chrono_dt.month());
    assert_eq!(shadow_dt.day(), chrono_dt.day());
    assert_eq!(shadow_dt.hour(), chrono_dt.hour());
    assert_eq!(shadow_dt.minute(), chrono_dt.minute());
    assert_eq!(shadow_dt.second(), chrono_dt.second());
    assert_eq!(shadow_dt.timestamp(), chrono_dt.timestamp());
    Ok(())
}

#[test]
fn datetime_utc_add_delta_parity() -> Result<(), Box<dyn std::error::Error>> {
    let s = "2024-01-01T00:00:00Z";
    let chrono_dt: chrono::DateTime<chrono::Utc> =
        s.parse().map_err(|e: chrono::ParseError| e.to_string())?;
    let shadow_dt = elicit_chrono::DateTime::parse(s).ok_or("shadow parse failed")?;

    let delta_secs: i64 = 86400 + 3600 + 60; // 1 day, 1 hour, 1 minute
    let chrono_result = chrono_dt + chrono::TimeDelta::seconds(delta_secs);
    // checked_add_signed takes Duration (shadow for chrono::Duration ≡ chrono::TimeDelta)
    let shadow_delta =
        elicit_chrono::Duration::try_seconds(delta_secs).ok_or("invalid delta")?;
    let shadow_result = shadow_dt
        .checked_add_signed(shadow_delta)
        .ok_or("shadow add overflow")?;

    assert_eq!(shadow_result.timestamp(), chrono_result.timestamp());
    assert_eq!(shadow_result.day(), chrono_result.day());
    assert_eq!(shadow_result.hour(), chrono_result.hour());
    Ok(())
}

#[test]
fn datetime_utc_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original: chrono::DateTime<chrono::Utc> = "2024-06-15T12:00:00Z"
        .parse()
        .map_err(|e: chrono::ParseError| e.to_string())?;
    let shadow = elicit_chrono::DateTime::from(original);
    let restored: chrono::DateTime<chrono::Utc> = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── DateTimeFixed (DateTime<FixedOffset>) ─────────────────────────────────────

#[test]
fn datetime_fixed_parse_parity() -> Result<(), Box<dyn std::error::Error>> {
    let s = "2024-01-15T12:30:00+09:00";
    let chrono_dt =
        chrono::DateTime::parse_from_rfc3339(s).map_err(|e| e.to_string())?;
    let shadow_dt =
        elicit_chrono::DateTimeFixed::parse(s).ok_or("shadow parse failed")?;

    assert_eq!(shadow_dt.timestamp(), chrono_dt.timestamp());
    assert_eq!(shadow_dt.year(), chrono_dt.year());
    assert_eq!(shadow_dt.hour(), chrono_dt.hour());
    Ok(())
}

#[test]
fn datetime_fixed_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let original =
        chrono::DateTime::parse_from_rfc3339("2024-03-15T08:00:00+05:30")
            .map_err(|e| e.to_string())?;
    let shadow = elicit_chrono::DateTimeFixed::from(original);
    let restored: chrono::DateTime<chrono::FixedOffset> = (*shadow).clone();
    assert_eq!(restored, original);
    Ok(())
}

// ── Weekday ───────────────────────────────────────────────────────────────────

#[test]
fn weekday_successor_cycle_parity() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    // Each day's succ must equal the next day in the cycle
    let days = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    for (i, name) in days.iter().enumerate() {
        let next_name = days[(i + 1) % 7];
        let shadow_day = elicit_chrono::Weekday::from_str(name)
            .map_err(|e| format!("parse {name}: {e}"))?;
        let shadow_next = shadow_day.succ();
        assert_eq!(
            shadow_next.to_string(),
            next_name,
            "succ of {name} should be {next_name}"
        );
    }
    Ok(())
}

#[test]
fn weekday_successor_matches_chrono() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let name = "Wed";
    let chrono_day: chrono::Weekday =
        name.parse().map_err(|e: chrono::ParseWeekdayError| e.to_string())?;
    let shadow_day = elicit_chrono::Weekday::from_str(name).map_err(|e| e)?;

    assert_eq!(shadow_day.succ().to_string(), chrono_day.succ().to_string());
    assert_eq!(shadow_day.pred().to_string(), chrono_day.pred().to_string());
    assert_eq!(
        shadow_day.num_days_from_monday(),
        chrono_day.num_days_from_monday()
    );
    Ok(())
}

// ── WeekdaySet ────────────────────────────────────────────────────────────────

#[test]
fn weekday_set_build_and_query() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let mon = elicit_chrono::Weekday::from_str("Mon").map_err(|e| e)?;
    let wed = elicit_chrono::Weekday::from_str("Wed").map_err(|e| e)?;
    let fri = elicit_chrono::Weekday::from_str("Fri").map_err(|e| e)?;

    let set =
        elicit_chrono::WeekdaySet::from_array(vec![mon.clone(), wed.clone(), fri.clone()]);

    assert_eq!(set.len(), 3);
    assert!(set.contains(mon.clone()));
    assert!(set.contains(wed.clone()));
    assert!(set.contains(fri.clone()));
    assert!(!set.is_empty());

    let removed = set.remove(wed.clone());
    assert_eq!(removed.len(), 2);
    assert!(!removed.contains(wed));
    Ok(())
}

#[test]
fn weekday_set_operations() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    let mon = elicit_chrono::Weekday::from_str("Mon").map_err(|e| e)?;
    let tue = elicit_chrono::Weekday::from_str("Tue").map_err(|e| e)?;
    let wed = elicit_chrono::Weekday::from_str("Wed").map_err(|e| e)?;

    let a = elicit_chrono::WeekdaySet::from_array(vec![mon.clone(), tue.clone()]);
    let b = elicit_chrono::WeekdaySet::from_array(vec![tue.clone(), wed.clone()]);

    let union = a.clone().union(b.clone());
    assert_eq!(union.len(), 3);

    let intersect = a.clone().intersection(b.clone());
    assert_eq!(intersect.len(), 1);
    assert!(intersect.contains(tue.clone()));

    let diff = a.clone().difference(b.clone());
    assert_eq!(diff.len(), 1);
    assert!(diff.contains(mon));

    let sym = a.symmetric_difference(b);
    assert_eq!(sym.len(), 2);
    Ok(())
}

// ── Free functions: parse / parse_and_remainder ───────────────────────────────

#[test]
fn format_parse_parity() -> Result<(), Box<dyn std::error::Error>> {
    let fmt = "%Y-%m-%d";
    let input = "2024-06-15";

    let items = elicit_chrono::StrftimeItems::new(fmt.to_string());
    let mut parsed = elicit_chrono::Parsed::new();
    elicit_chrono::parse(&mut parsed, input, items).map_err(|e| e.to_string())?;

    let chrono_items = chrono::format::StrftimeItems::new(fmt);
    let mut chrono_parsed = chrono::format::Parsed::new();
    chrono::format::parse(&mut chrono_parsed, input, chrono_items)
        .map_err(|e| e.to_string())?;

    assert_eq!(parsed.year(), chrono_parsed.year);
    assert_eq!(parsed.month(), chrono_parsed.month);
    assert_eq!(parsed.day(), chrono_parsed.day);
    Ok(())
}

#[test]
fn format_parse_and_remainder_parity() -> Result<(), Box<dyn std::error::Error>> {
    let fmt = "%Y-%m-%d";
    let input = "2024-06-15 extra stuff";

    let items = elicit_chrono::StrftimeItems::new(fmt.to_string());
    let mut parsed = elicit_chrono::Parsed::new();
    let remainder =
        elicit_chrono::parse_and_remainder(&mut parsed, input, items)
            .map_err(|e| e.to_string())?;

    let chrono_items = chrono::format::StrftimeItems::new(fmt);
    let mut chrono_parsed = chrono::format::Parsed::new();
    let chrono_remainder =
        chrono::format::parse_and_remainder(&mut chrono_parsed, input, chrono_items)
            .map_err(|e| e.to_string())?;

    assert_eq!(remainder, chrono_remainder);
    Ok(())
}

// ── TimeZone trait impls ───────────────────────────────────────────────────────

#[test]
fn utc_timezone_with_ymd_and_hms_parity() -> Result<(), Box<dyn std::error::Error>> {
    use chrono::TimeZone;
    let chrono_dt = chrono::Utc
        .with_ymd_and_hms(2024, 6, 15, 12, 0, 0)
        .single()
        .ok_or("chrono ymd failed")?;
    let shadow_dt =
        elicit_chrono::Utc::with_ymd_and_hms(2024, 6, 15, 12, 0, 0)
            .ok_or("shadow ymd failed")?;

    assert_eq!(shadow_dt.timestamp(), chrono_dt.timestamp());
    Ok(())
}
