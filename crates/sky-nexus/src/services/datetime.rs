use crate::models::datetime::*;
use chrono::{DateTime, Datelike, Utc};
use chrono_tz::Tz;
use thiserror::Error;
use tracing::{info, instrument, warn};

#[derive(Error, Debug)]
pub enum DateTimeServiceError {
    #[error("Invalid timezone: {0}")]
    InvalidTimezone(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

/// Calculate UTC offset in seconds using naive datetime difference
/// This is more robust than trying to use the offset methods
fn calculate_utc_offset_seconds(
    utc_time: DateTime<Utc>,
    local_time: DateTime<chrono_tz::Tz>,
) -> i32 {
    let utc_naive = utc_time.naive_utc();
    let local_naive = local_time.naive_local();
    let duration = local_naive.signed_duration_since(utc_naive);
    duration.num_seconds() as i32
}

#[instrument]
pub async fn get_current_datetime(
    query: GetDateTimeQuery,
) -> Result<DateTimeResponse, DateTimeServiceError> {
    let config = DateTimeConfig::from(query);

    info!(
        "Getting current datetime with timezone: {}, format: {}",
        config.timezone(),
        config.format()
    );

    let now = Utc::now();
    let timezone_str = config.timezone();

    let (formatted_time, utc_offset) = match timezone_str {
        "UTC" => {
            let format_str = if config.format.is_some() {
                config.format()
            } else {
                "%Y-%m-%d %H:%M:%S UTC"
            };
            (now.format(format_str).to_string(), 0)
        }
        tz_str => {
            let tz: Tz = tz_str.parse().map_err(|_| {
                DateTimeServiceError::InvalidTimezone(format!(
                    "Invalid timezone: {}. Examples: UTC, America/New_York, Europe/London",
                    tz_str
                ))
            })?;

            let local_time = now.with_timezone(&tz);
            let format_str = if config.format.is_some() {
                config.format()
            } else {
                "%Y-%m-%d %H:%M:%S %Z"
            };
            let formatted = local_time.format(format_str).to_string();
            let offset = calculate_utc_offset_seconds(now, local_time);

            (formatted, offset)
        }
    };

    Ok(DateTimeResponse {
        formatted_time,
        timezone: timezone_str.to_string(),
        unix_timestamp: now.timestamp(),
        iso_week: now.iso_week().week(),
        day_of_year: now.ordinal(),
        utc_offset,
    })
}

#[instrument]
pub async fn get_aviation_times() -> Result<AviationTimesResponse, DateTimeServiceError> {
    info!("Getting current time in aviation-relevant timezones");

    let now = Utc::now();

    // Common aviation timezones
    let timezone_configs = vec![
        ("UTC", "UTC"),
        ("New York (JFK/LGA/EWR)", "America/New_York"),
        ("Los Angeles (LAX)", "America/Los_Angeles"),
        ("Chicago (ORD/MDW)", "America/Chicago"),
        ("Denver (DEN)", "America/Denver"),
        ("London (LHR/LGW)", "Europe/London"),
        ("Paris (CDG/ORY)", "Europe/Paris"),
        ("Frankfurt (FRA)", "Europe/Berlin"),
        ("Tokyo (NRT/HND)", "Asia/Tokyo"),
        ("Sydney (SYD)", "Australia/Sydney"),
        ("Dubai (DXB)", "Asia/Dubai"),
        ("Singapore (SIN)", "Asia/Singapore"),
    ];

    let mut times = Vec::new();

    for (name, tz_str) in timezone_configs {
        if tz_str == "UTC" {
            times.push(AviationTimeZone {
                name: name.to_string(),
                timezone: tz_str.to_string(),
                local_time: now.format("%Y-%m-%d %H:%M:%S").to_string(),
                abbreviation: "UTC".to_string(),
                utc_offset_hours: 0.0,
            });
        } else {
            match tz_str.parse::<Tz>() {
                Ok(tz) => {
                    let local_time = now.with_timezone(&tz);
                    let offset_seconds = calculate_utc_offset_seconds(now, local_time);
                    let offset_hours = offset_seconds as f32 / 3600.0;

                    times.push(AviationTimeZone {
                        name: name.to_string(),
                        timezone: tz_str.to_string(),
                        local_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                        abbreviation: local_time.format("%Z").to_string(),
                        utc_offset_hours: offset_hours,
                    });
                }
                Err(e) => {
                    warn!("Failed to parse timezone {}: {}", tz_str, e);
                }
            }
        }
    }

    Ok(AviationTimesResponse {
        times,
        unix_timestamp: now.timestamp(),
    })
}

#[instrument]
pub async fn compare_timezones(
    request: TimezoneComparisonRequest,
) -> Result<TimezoneComparisonResponse, DateTimeServiceError> {
    info!(
        "Comparing timezones: {} vs {}",
        request.from_timezone, request.to_timezone
    );

    let now = Utc::now();

    // Handle UTC specially
    let (from_time_formatted, from_offset_seconds, from_abbreviation) =
        if request.from_timezone == "UTC" {
            (
                now.format("%Y-%m-%d %H:%M:%S").to_string(),
                0,
                "UTC".to_string(),
            )
        } else {
            let from_tz: Tz = request.from_timezone.parse().map_err(|_| {
                DateTimeServiceError::InvalidTimezone(format!(
                    "Invalid from_timezone: {}",
                    request.from_timezone
                ))
            })?;

            let from_time = now.with_timezone(&from_tz);
            let from_offset_seconds = calculate_utc_offset_seconds(now, from_time);
            (
                from_time.format("%Y-%m-%d %H:%M:%S").to_string(),
                from_offset_seconds,
                from_time.format("%Z").to_string(),
            )
        };

    let (to_time_formatted, to_offset_seconds, to_abbreviation) = if request.to_timezone == "UTC" {
        (
            now.format("%Y-%m-%d %H:%M:%S").to_string(),
            0,
            "UTC".to_string(),
        )
    } else {
        let to_tz: Tz = request.to_timezone.parse().map_err(|_| {
            DateTimeServiceError::InvalidTimezone(format!(
                "Invalid to_timezone: {}",
                request.to_timezone
            ))
        })?;

        let to_time = now.with_timezone(&to_tz);
        let to_offset_seconds = calculate_utc_offset_seconds(now, to_time);
        (
            to_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            to_offset_seconds,
            to_time.format("%Z").to_string(),
        )
    };

    // Calculate offset difference
    let difference_seconds = to_offset_seconds - from_offset_seconds;
    let difference_hours = difference_seconds as f32 / 3600.0;

    let description = if difference_hours > 0.0 {
        format!(
            "{} is {:.1} hours ahead of {}",
            request.to_timezone, difference_hours, request.from_timezone
        )
    } else if difference_hours < 0.0 {
        format!(
            "{} is {:.1} hours behind {}",
            request.to_timezone,
            difference_hours.abs(),
            request.from_timezone
        )
    } else {
        format!(
            "{} and {} are in the same time zone",
            request.from_timezone, request.to_timezone
        )
    };

    Ok(TimezoneComparisonResponse {
        from: TimezoneInfo {
            timezone: request.from_timezone,
            local_time: from_time_formatted,
            utc_offset_hours: from_offset_seconds as f32 / 3600.0,
            abbreviation: from_abbreviation,
        },
        to: TimezoneInfo {
            timezone: request.to_timezone,
            local_time: to_time_formatted,
            utc_offset_hours: to_offset_seconds as f32 / 3600.0,
            abbreviation: to_abbreviation,
        },
        difference_hours,
        description,
    })
}
