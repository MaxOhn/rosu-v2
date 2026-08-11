use rosu_v2::prelude::MonthlyCount;
use time::Date;

#[test]
fn deserialize_monthly_playcounts() {
    // Real data from osu! API: user.monthly_playcounts[]
    let json = r#"[
        { "start_date": "2007-10-01", "count": 483 },
        { "start_date": "2007-11-01", "count": 381 },
        { "start_date": "2007-12-01", "count": 94 }
    ]"#;

    let counts: Vec<MonthlyCount> = serde_json::from_str(json).unwrap();

    assert_eq!(counts.len(), 3);
    assert_eq!(counts[0].start_date, Date::from_calendar_date(2007, time::Month::October, 1).unwrap());
    assert_eq!(counts[0].count, 483);
    assert_eq!(counts[2].start_date, Date::from_calendar_date(2007, time::Month::December, 1).unwrap());
    assert_eq!(counts[2].count, 94);
}
