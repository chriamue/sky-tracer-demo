use crate::services::airports::fetch_airport_by_code;
use rmcp::{
    ErrorData as McpError,
    model::{
        AnnotateAble, ListResourceTemplatesResult, ListResourcesResult, RawResource,
        RawResourceTemplate, ReadResourceRequestParams, ReadResourceResult, ResourceContents,
    },
};
use serde_json::json;
use tracing::{error, info};

const URI_PREFIX: &str = "airports://";

/// A small curated list exposed as static resources.
/// Clients can fetch any airport via the URI template.
const FEATURED: &[(&str, &str)] = &[
    ("FRA", "Frankfurt Airport"),
    ("LHR", "London Heathrow"),
    ("CDG", "Paris Charles de Gaulle"),
    ("JFK", "John F. Kennedy International"),
    ("DXB", "Dubai International"),
    ("SIN", "Singapore Changi"),
    ("AMS", "Amsterdam Schiphol"),
    ("HND", "Tokyo Haneda"),
];

fn airport_uri(code: &str) -> String {
    format!("{}{}", URI_PREFIX, code)
}

fn parse_code(uri: &str) -> Option<&str> {
    uri.strip_prefix(URI_PREFIX).filter(|s| !s.is_empty())
}

pub fn list_resources() -> ListResourcesResult {
    let resources = FEATURED
        .iter()
        .map(|(code, name)| {
            RawResource {
                uri: airport_uri(code),
                name: name.to_string(),
                title: Some(format!("{} ({})", name, code)),
                description: Some(format!("Live data for {} airport (IATA: {})", name, code)),
                mime_type: Some("application/json".to_string()),
                size: None,
                icons: None,
                meta: None,
            }
            .no_annotation()
        })
        .collect();

    ListResourcesResult {
        resources,
        next_cursor: None,
        meta: None,
    }
}

pub fn list_resource_templates() -> ListResourceTemplatesResult {
    let template = RawResourceTemplate {
        uri_template: format!("{}{{code}}", URI_PREFIX),
        name: "Airport by IATA code".to_string(),
        title: Some("Airport resource".to_string()),
        description: Some(
            "Fetch live airport data by IATA code. Example: airports://FRA".to_string(),
        ),
        mime_type: Some("application/json".to_string()),
        icons: None,
    }
    .no_annotation();

    ListResourceTemplatesResult {
        resource_templates: vec![template],
        next_cursor: None,
        meta: None,
    }
}

pub async fn read_resource(
    params: ReadResourceRequestParams,
) -> Result<ReadResourceResult, McpError> {
    let code = parse_code(&params.uri).ok_or_else(|| {
        McpError::invalid_params(
            format!(
                "Invalid URI '{}'. Expected format: airports://{{IATA_CODE}}",
                params.uri
            ),
            None,
        )
    })?;

    info!(code = %code, uri = %params.uri, "Reading airport resource");

    let airport = fetch_airport_by_code(code).await.map_err(|e| {
        error!(code = %code, error = %e, "Failed to fetch airport resource");
        McpError::internal_error(
            "Airport not found",
            Some(json!({ "code": code, "error": e.to_string() })),
        )
    })?;

    let body = serde_json::to_string_pretty(&json!({
        "id": airport.id,
        "code": airport.code,
        "name": airport.name,
        "latitude": airport.latitude,
        "longitude": airport.longitude,
    }))
    .unwrap_or_default();

    Ok(ReadResourceResult::new(vec![
        ResourceContents::text(body, params.uri).with_mime_type("application/json"),
    ]))
}
