use crate::mcp::prompts::SkyNexusPrompts;
use crate::mcp::resources;

use crate::mcp::tools::{AirportTools, BabelTools, DateTimeTools, FlightTools, MapTools, SatelliteTools};
use rmcp::{
    RoleServer, ServerHandler,
    handler::server::prompt::PromptContext,
    model::{
        CallToolResult, GetPromptRequestParams, GetPromptResult, Implementation,
        ListPromptsResult, ListResourceTemplatesResult, ListResourcesResult, ListToolsResult,
        PaginatedRequestParams, ReadResourceRequestParams, ReadResourceResult, ServerCapabilities,
        ServerInfo,
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
    map: MapTools,
    prompts: SkyNexusPrompts,
}

impl SkyNexusTools {
    pub fn new() -> Self {
        Self {
            airports: AirportTools::new(),
            flights: FlightTools::new(),
            satellites: SatelliteTools::new(),
            datetime: DateTimeTools::new(),
            babel: BabelTools::new(),
            map: MapTools::new(),
            prompts: SkyNexusPrompts::new(),
        }
    }
}

impl ServerHandler for SkyNexusTools {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(Implementation::from_build_env())
        .with_instructions(
            "Sky Nexus MCP server — comprehensive aviation data.\n\
            Tools: list/get airports, manage flights, track satellites, calculate positions, check delays\n\
            Resources: airports://{code} — live airport data by IATA code\n\
            Prompts: airport-briefing, flight-route-analysis, delay-investigation, aviation-network-overview",
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
        tools.extend(
            self.babel
                .list_tools(request.clone(), context.clone())
                .await?
                .tools,
        );
        tools.extend(self.map.list_tools(request, context).await?.tools);
        Ok(ListToolsResult::with_all_items(tools))
    }

    async fn call_tool(
        &self,
        request: rmcp::model::CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResult, rmcp::ErrorData> {
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
            "generate_flight_map" => self.map.call_tool(request, context).await,
            name => Err(rmcp::ErrorData::invalid_params(
                format!("unknown tool: {name}"),
                None,
            )),
        }
    }

    async fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourcesResult, rmcp::ErrorData> {
        Ok(resources::list_resources())
    }

    async fn list_resource_templates(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListResourceTemplatesResult, rmcp::ErrorData> {
        Ok(resources::list_resource_templates())
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> Result<ReadResourceResult, rmcp::ErrorData> {
        resources::read_resource(request).await
    }

    async fn list_prompts(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> Result<ListPromptsResult, rmcp::ErrorData> {
        Ok(ListPromptsResult {
            prompts: self.prompts.prompt_router.list_all(),
            next_cursor: None,
            meta: None,
        })
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<GetPromptResult, rmcp::ErrorData> {
        self.prompts
            .prompt_router
            .get_prompt(PromptContext::new(
                &self.prompts,
                request.name,
                request.arguments,
                context,
            ))
            .await
    }
}
