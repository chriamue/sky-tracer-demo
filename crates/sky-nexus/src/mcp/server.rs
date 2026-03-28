use crate::mcp::tools::{AirportTools, BabelTools, DateTimeTools, FlightTools, SatelliteTools};
use rmcp::{
    RoleServer, ServerHandler,
    model::{
        CallToolResult, Implementation, ListToolsResult, PaginatedRequestParams,
        ServerCapabilities, ServerInfo,
    },
    service::RequestContext,
};

/// Single MCP server that aggregates all Sky Nexus tool categories.
#[derive(Clone, Debug, Default)]
pub struct SkyNexusTools {
    airports: AirportTools,
    flights: FlightTools,
    satellites: SatelliteTools,
    datetime: DateTimeTools,
    babel: BabelTools,
}

impl SkyNexusTools {
    pub fn new() -> Self {
        Self {
            airports: AirportTools::new(),
            flights: FlightTools::new(),
            satellites: SatelliteTools::new(),
            datetime: DateTimeTools::new(),
            babel: BabelTools::new(),
        }
    }
}

impl ServerHandler for SkyNexusTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::from_build_env())
            .with_instructions(
                "Sky Nexus MCP server — comprehensive aviation data.\n\
                Airports: list_airports, get_airport\n\
                Flights: list_flights, get_flight, create_flight, search_flights_by_route\n\
                Satellites: list_satellites, create_satellite, update_satellite_status, calculate_position\n\
                DateTime: get_current_datetime, get_aviation_times, get_timezone_difference, compare_timezones\n\
                Tracking: get_flights_by_airport, get_flight_position",
            )
    }

    async fn list_tools(
        &self,
        request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, rmcp::ErrorData> {
        let mut tools = vec![];
        tools.extend(
            self.airports
                .list_tools(request.clone(), context.clone())
                .await?
                .tools,
        );
        tools.extend(
            self.flights
                .list_tools(request.clone(), context.clone())
                .await?
                .tools,
        );
        tools.extend(
            self.satellites
                .list_tools(request.clone(), context.clone())
                .await?
                .tools,
        );
        tools.extend(
            self.datetime
                .list_tools(request.clone(), context.clone())
                .await?
                .tools,
        );
        tools.extend(self.babel.list_tools(request, context).await?.tools);
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
        // Dispatch to the sub-handler that owns the requested tool.
        // Tool names correspond to #[tool] definitions in each tools/* file.
        match request.name.as_ref() {
            "list_airports" | "get_airport" => self.airports.call_tool(request, context).await,
            "list_flights" | "get_flight" | "create_flight" | "search_flights_by_route" => {
                self.flights.call_tool(request, context).await
            }
            "list_satellites"
            | "create_satellite"
            | "update_satellite_status"
            | "calculate_position" => self.satellites.call_tool(request, context).await,
            "get_current_datetime"
            | "get_aviation_times"
            | "get_timezone_difference"
            | "compare_timezones" => self.datetime.call_tool(request, context).await,
            "get_flights_by_airport"
            | "get_flight_position"
            | "search_flights_by_airport_pattern" => self.babel.call_tool(request, context).await,
            name => Err(rmcp::ErrorData::invalid_params(
                format!("unknown tool: {name}"),
                None,
            )),
        }
    }
}
