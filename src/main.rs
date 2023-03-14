use chrono::{NaiveDate};
use regex::Regex;
use reqwest::blocking::get;
use serde::{Serialize, Deserialize};
use serde_json;
use std::env;

#[derive(Serialize, Deserialize)]
struct Holiday {
    date_start: Option<String>,
    date_end: Option<String>,
    summary: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: URL is a required parameter");
        std::process::exit(1);
    }

    let uk_holidays = get(&args[1])?.text()?;

    let holiday_regex = Regex::new(r"BEGIN:VEVENT\r\nDTEND;VALUE=DATE:(\d+)\r\nDTSTART;VALUE=DATE:(\d+)\r\nSUMMARY:(.+)\r\nUID:.+\r\nSEQUENCE:\d+\r\nDTSTAMP:(\d{8}T\d{6}Z)\r\nEND:VEVENT").unwrap();
    let holidays: Vec<Holiday> = holiday_regex
        .captures_iter(&uk_holidays)
        .map(|captures| {
            let date_end = NaiveDate::parse_from_str(&captures[1], "%Y%m%d").ok().map(|date| date.format("%Y-%m-%d").to_string());
            let date_start = NaiveDate::parse_from_str(&captures[2], "%Y%m%d").ok().map(|date| date.format("%Y-%m-%d").to_string());
            let summary = captures[3].to_string();
            Holiday {
                date_start,
                date_end,
                summary,
            }
        })
        .collect();

    let pretty_holidays = serde_json::to_string_pretty(&holidays)?;

    println!("{}", pretty_holidays);

    Ok(())
}

