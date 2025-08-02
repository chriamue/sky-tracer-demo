use yew::prelude::*;

#[function_component(LaunchSatellite)]
pub fn launch_satellite() -> Html {
    let path_prefix = crate::utils::get_path_prefix();

    html! {
        <div class="launch-satellite">
            <h2>{"🚀 Launch New Satellite"}</h2>
            <div class="launch-container">
                <form action={format!("{}/launch", path_prefix)} method="POST" class="launch-form">
                    <div class="rocket">
                        <div class="rocket-body">
                            <div class="body"></div>
                            <div class="fin fin-left"></div>
                            <div class="fin fin-right"></div>
                            <div class="window"></div>
                        </div>
                        <div class="exhaust-flame"></div>
                        <ul class="exhaust-fumes">
                            <li></li>
                            <li></li>
                            <li></li>
                            <li></li>
                        </ul>
                    </div>
                    <div class="form-group">
                        <label for="name">{"Satellite Name:"}</label>
                        <input
                            type="text"
                            id="name"
                            name="name"
                            required=true
                            placeholder="Enter satellite name"
                        />
                    </div>
                    <button type="submit" class="launch-button">
                        {"Launch Satellite"}
                    </button>
                </form>
            </div>
        </div>
    }
}
