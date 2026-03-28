use crate::services::airports::fetch_airport_by_code;
use crate::services::babel::fetch_flights_by_airport;
use rmcp::{
    handler::server::{router::prompt::PromptRouter, wrapper::Parameters},
    model::{GetPromptResult, PromptMessage, PromptMessageRole},
    prompt, prompt_router,
    schemars,
};
use serde::Deserialize;

// ── Request parameter types ────────────────────────────────────────────────

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct AirportBriefingParams {
    #[schemars(description = "IATA airport code (e.g. FRA, LHR, JFK)")]
    pub airport_code: String,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FlightRouteParams {
    #[schemars(description = "Departure airport IATA code")]
    pub departure: String,
    #[schemars(description = "Arrival airport IATA code")]
    pub arrival: String,
    #[schemars(description = "Optional date for the flight (YYYY-MM-DD)")]
    pub date: Option<String>,
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DelayInvestigationParams {
    #[schemars(description = "IATA airport code to investigate delays for")]
    pub airport_code: String,
}

// ── Prompt handler struct ──────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SkyNexusPrompts {
    pub prompt_router: PromptRouter<Self>,
}

impl Default for SkyNexusPrompts {
    fn default() -> Self {
        Self::new()
    }
}

#[prompt_router]
impl SkyNexusPrompts {
    pub fn new() -> Self {
        Self {
            prompt_router: Self::prompt_router(),
        }
    }

    /// Generate a detailed briefing prompt for a specific airport.
    /// Fetches live airport data and includes it as context.
    #[prompt(
        name = "airport-briefing",
        description = "Generate a comprehensive airport briefing including location, facilities, and operational context."
    )]
    pub async fn airport_briefing(
        &self,
        Parameters(p): Parameters<AirportBriefingParams>,
    ) -> GetPromptResult {
        let context = match fetch_airport_by_code(&p.airport_code).await {
            Ok(a) => format!(
                "Airport: {} ({})\nLatitude: {}\nLongitude: {}",
                a.name, a.code, a.latitude, a.longitude
            ),
            Err(_) => format!("Airport code: {}", p.airport_code),
        };

        GetPromptResult::new(vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Please provide a detailed aviation briefing for the following airport.\n\n\
                    **Airport data:**\n```\n{context}\n```\n\n\
                    Format your response in Markdown with the following sections:\n\
                    ## ✈️ Airport Overview\n\
                    ## 🌍 Geographic Context\n\
                    ## 🌤️ Typical Weather\n\
                    ## 🛫 Major Airlines & Routes\n\
                    ## 🏢 Terminals & Capacity\n\
                    ## ⚙️ Operational Notes\n\n\
                    Use bullet points, bold key facts, and a summary table where helpful."
                ),
            ),
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                format!(
                    "I'll provide a comprehensive Markdown briefing for **{}** airport.",
                    p.airport_code.to_uppercase()
                ),
            ),
        ])
        .with_description(format!("Airport briefing for {}", p.airport_code.to_uppercase()))
    }

    /// Analyse a flight route between two airports.
    #[prompt(
        name = "flight-route-analysis",
        description = "Analyse a flight route between two airports: distance, typical duration, common airlines, and operational notes."
    )]
    pub async fn flight_route_analysis(
        &self,
        Parameters(p): Parameters<FlightRouteParams>,
    ) -> GetPromptResult {
        let date_hint = p
            .date
            .as_deref()
            .map(|d| format!(" on {}", d))
            .unwrap_or_default();

        let (dep_ctx, arr_ctx) = tokio::join!(
            fetch_airport_by_code(&p.departure),
            fetch_airport_by_code(&p.arrival),
        );

        let dep_info = dep_ctx
            .map(|a| format!("{} ({})", a.name, a.code))
            .unwrap_or_else(|_| p.departure.clone());
        let arr_info = arr_ctx
            .map(|a| format!("{} ({})", a.name, a.code))
            .unwrap_or_else(|_| p.arrival.clone());

        GetPromptResult::new(vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Analyse the flight route from **{dep_info}** to **{arr_info}**{date_hint}.\n\n\
                    Format your response in Markdown with the following sections:\n\
                    ## 🗺️ Route Summary\n\
                    Include a one-line summary with origin → destination and estimated distance.\n\
                    ## ⏱️ Flight Duration & Distance\n\
                    ## ✈️ Typical Aircraft Types\n\
                    ## 🏢 Major Carriers\n\
                    Use a Markdown table: | Airline | Code | Frequency |\n\
                    ## 🕐 Time Zone Changes\n\
                    ## 🌐 Overflight Countries\n\
                    ## ⚠️ Operational & Regulatory Notes\n\n\
                    Use bold for key figures (e.g. **~9h 30m**, **8,500 km**)."
                ),
            ),
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                format!(
                    "I'll analyse the **{}** → **{}** route{} in Markdown.",
                    p.departure.to_uppercase(),
                    p.arrival.to_uppercase(),
                    date_hint
                ),
            ),
        ])
        .with_description(format!(
            "Route analysis: {} → {}",
            p.departure.to_uppercase(),
            p.arrival.to_uppercase()
        ))
    }

    /// Investigate flight delays at an airport.
    #[prompt(
        name = "delay-investigation",
        description = "Investigate and explain current or recent flight delays at an airport."
    )]
    pub async fn delay_investigation(
        &self,
        Parameters(p): Parameters<DelayInvestigationParams>,
    ) -> GetPromptResult {
        let flights_context = match fetch_flights_by_airport(&p.airport_code).await {
            Ok(flights) if !flights.is_empty() => {
                let summary: Vec<String> = flights
                    .iter()
                    .take(5)
                    .map(|f| format!("  - {} ({} → {})", f.flight_number, f.departure, f.arrival))
                    .collect();
                format!(
                    "Active flights at {}:\n{}",
                    p.airport_code.to_uppercase(),
                    summary.join("\n")
                )
            }
            _ => format!("Airport: {}", p.airport_code.to_uppercase()),
        };

        GetPromptResult::new(vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                format!(
                    "Investigate flight delays at **{}** airport.\n\n\
                    **Current flight data:**\n```\n{flights_context}\n```\n\n\
                    Format your response in Markdown:\n\
                    ## 🚦 Delay Status\n\
                    Start with a severity indicator: 🟢 Minor / 🟡 Moderate / 🔴 Severe\n\
                    ## 🔍 Root Cause Analysis\n\
                    Use a numbered list for likely causes in order of probability.\n\
                    ## 🔗 Knock-on Effects\n\
                    ## 💡 Passenger Recommendations\n\
                    Use a checklist: `- [ ] item`\n\
                    ## 📊 Affected Flights\n\
                    If flight data is available, list them in a table: | Flight | Route | Status |",
                    p.airport_code.to_uppercase()
                ),
            ),
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                format!(
                    "I'll investigate delays at **{}** and present a structured Markdown report.",
                    p.airport_code.to_uppercase()
                ),
            ),
        ])
        .with_description(format!(
            "Delay investigation for {}",
            p.airport_code.to_uppercase()
        ))
    }

    /// High-level overview of the entire Sky Tracer aviation network.
    #[prompt(
        name = "aviation-network-overview",
        description = "Generate an overview prompt for the Sky Tracer aviation tracking network — no arguments needed."
    )]
    pub async fn aviation_network_overview(&self) -> Vec<PromptMessage> {
        vec![
            PromptMessage::new_text(
                PromptMessageRole::User,
                "You have access to the **Sky Tracer** aviation network via MCP.\n\n\
                Format your response in Markdown:\n\n\
                ## 🛫 Sky Tracer — Capability Overview\n\
                ### 🔧 Tools\n\
                List all available tools grouped by category in a table: | Tool | Description |\n\
                ### 📦 Resources\n\
                Explain the `airports://{code}` URI scheme with an example.\n\
                ### 💬 Prompts\n\
                List all prompts with their required arguments.\n\
                ### 💡 Suggested Queries\n\
                Provide exactly **3** example questions a user might ask, formatted as a numbered list.\n\
                Each suggestion should use a different capability (tool, resource, prompt)."
                    .to_string(),
            ),
            PromptMessage::new_text(
                PromptMessageRole::Assistant,
                "Here's a Markdown overview of everything I can do with the Sky Tracer aviation network:"
                    .to_string(),
            ),
        ]
    }
}
