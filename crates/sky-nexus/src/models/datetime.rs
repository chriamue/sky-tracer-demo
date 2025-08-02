use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, ToSchema)]
pub struct DateTimeResponse {
    /// Formatted date and time string
    pub formatted_time: String,
    /// Timezone used
    pub timezone: String,
    /// Unix timestamp
    pub unix_timestamp: i64,
    /// ISO week number
    pub iso_week: u32,
    /// Day of year
    pub day_of_year: u32,
    /// UTC offset in seconds
    pub utc_offset: i32,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct GetDateTimeQuery {
    /// Timezone (optional, defaults to UTC)
    #[schema(example = "America/New_York")]
    pub timezone: Option<String>,
    /// Format string (optional, defaults to RFC3339)
    #[schema(example = "%Y-%m-%d %H:%M:%S")]
    pub format: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AviationTimesResponse {
    /// Current times in major aviation hubs
    pub times: Vec<AviationTimeZone>,
    /// UTC Unix timestamp
    pub unix_timestamp: i64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AviationTimeZone {
    /// Location name
    pub name: String,
    /// Timezone identifier
    pub timezone: String,
    /// Formatted local time
    pub local_time: String,
    /// Timezone abbreviation
    pub abbreviation: String,
    /// UTC offset in hours
    pub utc_offset_hours: f32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TimezoneComparisonRequest {
    /// Source timezone
    #[schema(example = "America/New_York")]
    pub from_timezone: String,
    /// Target timezone
    #[schema(example = "Europe/London")]
    pub to_timezone: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TimezoneComparisonResponse {
    /// Source timezone information
    pub from: TimezoneInfo,
    /// Target timezone information
    pub to: TimezoneInfo,
    /// Time difference in hours
    pub difference_hours: f32,
    /// Time difference description
    pub description: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TimezoneInfo {
    /// Timezone identifier
    pub timezone: String,
    /// Current local time
    pub local_time: String,
    /// UTC offset in hours
    pub utc_offset_hours: f32,
    /// Timezone abbreviation
    pub abbreviation: String,
}

#[derive(Debug, Clone)]
pub struct DateTimeConfig {
    pub timezone: Option<String>,
    pub format: Option<String>,
}

impl From<GetDateTimeQuery> for DateTimeConfig {
    fn from(query: GetDateTimeQuery) -> Self {
        Self {
            timezone: query.timezone,
            format: query.format,
        }
    }
}

impl DateTimeConfig {
    pub fn new() -> Self {
        Self {
            timezone: None,
            format: None,
        }
    }

    pub fn with_timezone(mut self, timezone: String) -> Self {
        self.timezone = Some(timezone);
        self
    }

    pub fn with_format(mut self, format: String) -> Self {
        self.format = Some(format);
        self
    }

    pub fn timezone(&self) -> &str {
        self.timezone.as_deref().unwrap_or("UTC")
    }

    pub fn format(&self) -> &str {
        self.format.as_deref().unwrap_or("%Y-%m-%d %H:%M:%S UTC")
    }
}

impl Default for DateTimeConfig {
    fn default() -> Self {
        Self::new()
    }
}
