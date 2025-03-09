use gloo_net::http::Request;
use gloo_utils::document;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use sky_tracer::protocol::airports::AirportResponse;
use sky_tracer::protocol::flights::FlightPositionResponse;
use sky_tracer::protocol::flights::FlightResponse;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::prelude::*;

#[derive(Clone)]
struct FlightData {
    flight: FlightResponse,
    position: Option<FlightPositionResponse>,
}

#[derive(Clone, Default)]
struct MapState {
    flights: HashMap<String, FlightData>,
    airports: HashMap<String, AirportResponse>,
}

pub enum Msg {
    UpdateFlights(Vec<FlightResponse>),
    UpdatePosition(String, FlightPositionResponse),
    UpdateAirport(String, AirportResponse),
    FetchFlights,
    FetchPositions,
}

pub struct FlightMap {
    map: Map,
    container: HtmlElement,
    state: MapState,
    _flights_interval: Option<gloo_timers::callback::Interval>,
    _positions_interval: Option<gloo_timers::callback::Interval>,
}

impl Component for FlightMap {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        // Create map container
        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map-container");

        // Initialize map
        let map = Map::new_with_element(&container, &MapOptions::default());

        Self {
            map,
            container,
            state: MapState::default(),
            _flights_interval: None,
            _positions_interval: None,
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Set initial view
            self.map.set_view(&LatLng::new(50.0, 10.0), 4.0);

            // Add tile layer
            TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(&self.map);

            // Setup intervals
            let link = ctx.link().clone();
            self._flights_interval = Some(gloo_timers::callback::Interval::new(5_000, move || {
                link.send_message(Msg::FetchFlights);
            }));

            let link = ctx.link().clone();
            self._positions_interval =
                Some(gloo_timers::callback::Interval::new(5_000, move || {
                    link.send_message(Msg::FetchPositions);
                }));

            // Initial fetch
            ctx.link().send_message(Msg::FetchFlights);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchFlights => {
                let link = ctx.link().clone();
                wasm_bindgen_futures::spawn_local(async move {
                    if let Ok(response) = Request::get("/api/flights").send().await {
                        if let Ok(flights) = response.json::<Vec<FlightResponse>>().await {
                            link.send_message(Msg::UpdateFlights(flights));
                        }
                    }
                });
                false
            }
            Msg::UpdateFlights(flights) => {
                for flight in flights {
                    // Fetch airport data if needed
                    let link = ctx.link().clone();
                    let code = flight.departure.clone();
                    if !self.state.airports.contains_key(&code) {
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(response) =
                                Request::get(&format!("/api/airports/search?code={}", code))
                                    .send()
                                    .await
                            {
                                if let Ok(mut airports) =
                                    response.json::<Vec<AirportResponse>>().await
                                {
                                    if let Some(airport) = airports.pop() {
                                        link.send_message(Msg::UpdateAirport(code, airport));
                                    }
                                }
                            }
                        });
                    }

                    let link = ctx.link().clone();
                    let code = flight.arrival.clone();
                    if !self.state.airports.contains_key(&code) {
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Ok(response) =
                                Request::get(&format!("/api/airports/search?code={}", code))
                                    .send()
                                    .await
                            {
                                if let Ok(mut airports) =
                                    response.json::<Vec<AirportResponse>>().await
                                {
                                    if let Some(airport) = airports.pop() {
                                        link.send_message(Msg::UpdateAirport(code, airport));
                                    }
                                }
                            }
                        });
                    }

                    self.state.flights.insert(
                        flight.flight_number.clone(),
                        FlightData {
                            flight,
                            position: None,
                        },
                    );
                }
                self.update_map_markers();
                true
            }
            Msg::FetchPositions => {
                let flight_numbers: Vec<String> = self.state.flights.keys().cloned().collect();
                for flight_number in flight_numbers {
                    let link = ctx.link().clone();
                    let flight_number_clone = flight_number.clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        if let Ok(response) =
                            Request::get(&format!("/api/flights/{}/position", flight_number_clone))
                                .send()
                                .await
                        {
                            if let Ok(position) = response.json::<FlightPositionResponse>().await {
                                link.send_message(Msg::UpdatePosition(
                                    flight_number_clone,
                                    position,
                                ));
                            }
                        }
                    });
                }
                false
            }
            Msg::UpdatePosition(flight_number, position) => {
                if let Some(flight_data) = self.state.flights.get_mut(&flight_number) {
                    flight_data.position = Some(position);
                    self.update_map_markers();
                }
                true
            }
            Msg::UpdateAirport(code, airport) => {
                self.state.airports.insert(code, airport);
                self.update_map_markers();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="map-section">
                <h2>{"Live Flight Map"}</h2>
                {self.render_map()}
            </div>
        }
    }
}

impl FlightMap {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }

    fn update_map_markers(&mut self) {
        // Add new markers and paths
        for data in self.state.flights.values() {
            // ... (implement marker creation code here)
        }
    }
}
