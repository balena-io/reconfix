use chrono;
use lazy_static::lazy_static;
use regex::Regex;
use serde_json::Value;

use crate::validator::{scope::ScopedSchema, state::ValidationState, types::validate_as_string};

lazy_static! {
    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js#L90
    static ref DATE_REGEX: Regex =
        Regex::new(r"^(\d\d\d\d)-(\d\d)-(\d\d)$").unwrap();

    // ajv v6.7.0 compatible
    // https://github.com/epoberezkin/ajv/blob/v6.7.0/lib/compile/formats.js#L104
    static ref TIME_REGEX: Regex =
        Regex::new(r"^(\d\d):(\d\d):(\d\d)(\.\d+)?(z|[+-]\d\d:\d\d)?$").unwrap();
}

pub fn validate_as_datetime(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let mut state = validate_as_string(scope, data);

    if state.is_valid()
        && chrono::DateTime::parse_from_rfc3339(data.as_str().expect("invalid validate_as_string")).is_err()
    {
        state.push_error(scope.error("type", "unable to parse as 'datetime'"));
    }

    state
}

fn is_leap_year(year: usize) -> bool {
    // https://tools.ietf.org/html/rfc3339#appendix-C
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days(year: usize, month: usize) -> usize {
    const DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS[month - 1]
    }
}

pub fn validate_as_date(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let s = match data.as_str() {
        Some(x) => x,
        None => return scope.error("type", "expected `date`").into(),
    };

    let captures = match DATE_REGEX.captures(s) {
        Some(x) => x,
        _ => return scope.error("type", "expected `date`").into(),
    };

    let year: usize = (&captures[1]).parse().expect("invalid regex");
    let month: usize = (&captures[2]).parse().expect("invalid regex");
    let day: usize = (&captures[3]).parse().expect("invalid regex");

    if month >= 1 && month <= 12 && day >= 1 && day <= days(year, month) {
        ValidationState::new()
    } else {
        scope.error("type", "invalid `date` range").into()
    }
}

pub fn validate_as_time(scope: &ScopedSchema, data: &Value) -> ValidationState {
    let s = match data.as_str() {
        Some(x) => x,
        None => return scope.error("type", "expected `time`").into(),
    };

    let captures = match TIME_REGEX.captures(s) {
        Some(x) => x,
        _ => return scope.error("type", "expected `time`").into(),
    };

    let hour: usize = (&captures[1]).parse().expect("invalid regex");
    let min: usize = (&captures[2]).parse().expect("invalid regex");
    let sec: usize = (&captures[3]).parse().expect("invalid regex");

    if (hour <= 23 && min <= 59 && sec <= 59) || (hour == 23 && min == 59 && sec == 60) {
        ValidationState::new()
    } else {
        scope.error("type", "invalid `time` range").into()
    }
}
