use yew::prelude::*;

#[function_component(PositionForm)]
pub fn position_form() -> Html {
    html! {
        <div class="position-form">
            <h2>{"Calculate Flight Position"}</h2>
            <form action="/api/position" method="POST" class="position-form">
                <div class="form-group">
                    <label for="departure">{"Departure Airport:"}</label>
                    <input
                        type="text"
                        id="departure"
                        name="departure"
                        required=true
                        placeholder="Enter departure airport"
                    />
                </div>
                <div class="form-group">
                    <label for="arrival">{"Arrival Airport:"}</label>
                    <input
                        type="text"
                        id="arrival"
                        name="arrival"
                        required=true
                        placeholder="Enter arrival airport"
                    />
                </div>
                <div class="form-group">
                    <label for="departure_time">{"Departure Time:"}</label>
                    <input
                        type="datetime-local"
                        id="departure_time"
                        name="departure_time"
                        required=true
                    />
                </div>
                <button type="submit">{"Calculate Position"}</button>
            </form>
        </div>
    }
}
