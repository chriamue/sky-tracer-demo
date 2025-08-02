use cockpit::App;

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("Failed to initialize logger");
    yew::Renderer::<App>::new().render();
}
