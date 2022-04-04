use std::collections::HashMap;
use crate::DAYS::{FRIDAY, THURSDAY, TUESDAY, WEDNESDAY};

fn check_leap_year(year: u32) -> bool {
    return year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
}

#[derive(Copy, Clone)]
enum DAYS {
    MONDAY,
    TUESDAY,
    WEDNESDAY,
    THURSDAY,
    FRIDAY,
    SATURDAY,
    SUNDAY,
}

#[derive(Copy, Clone)]
enum MONTHS {
    JAN,
    FEB,
    MAR,
    APR,
    MAY,
    JUN,
    JUL,
    AUG,
    SEP,
    OCT,
    NOV,
    DEC,
}

fn next_day(day: DAYS) -> DAYS {
    match day {
        DAYS::MONDAY => DAYS::TUESDAY,
        DAYS::TUESDAY => DAYS::WEDNESDAY,
        DAYS::WEDNESDAY => DAYS::THURSDAY,
        DAYS::THURSDAY => DAYS::FRIDAY,
        DAYS::FRIDAY => DAYS::SATURDAY,
        DAYS::SATURDAY => DAYS::SUNDAY,
        DAYS::SUNDAY => DAYS::MONDAY
    }
}

fn next_month(month: MONTHS) -> MONTHS {
    match month {
        MONTHS::JAN => MONTHS::FEB,
        MONTHS::FEB => MONTHS::MAR,
        MONTHS::MAR => MONTHS::APR,
        MONTHS::APR => MONTHS::MAY,
        MONTHS::MAY => MONTHS::JUN,
        MONTHS::JUN => MONTHS::JUL,
        MONTHS::JUL => MONTHS::AUG,
        MONTHS::AUG => MONTHS::SEP,
        MONTHS::SEP => MONTHS::OCT,
        MONTHS::OCT => MONTHS::NOV,
        MONTHS::NOV => MONTHS::DEC,
        MONTHS::DEC => MONTHS::JAN
    }
}

fn days_in_month(month: MONTHS, year: u32) -> u32 {
    match month {
        MONTHS::JAN => 31,
        MONTHS::FEB => {
            if check_leap_year(year) {
                29
            } else {
                28
            }
        },
        MONTHS::MAR => 31,
        MONTHS::APR => 30,
        MONTHS::MAY => 31,
        MONTHS::JUN => 30,
        MONTHS::JUL => 31,
        MONTHS::AUG => 31,
        MONTHS::SEP => 30,
        MONTHS::OCT => 31,
        MONTHS::NOV => 30,
        MONTHS::DEC => 31
    }
}

fn main() {
    let mut result = 0;
    let mut date = 1;
    let mut day = DAYS::MONDAY;
    let mut year = 1900;
    let mut month = MONTHS::JAN;
    while year < 2001 {
        for i in 0..12 {
            let days_in_month = days_in_month(month, year);
            if year > 1900 && matches!(day, DAYS::SUNDAY) { result += 1 }
            while date <= days_in_month {
                date += 1;
                day = next_day(day);
            }
            date = 1;
            month = next_month(month);
        }
        year += 1;
    }
    println!("{}", result);
}