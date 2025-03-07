use cockpit::App;

fn main() {
    console_log::init_with_level(log::Level::Debug).unwrap();
    yew::Renderer::<App>::new().render();
}
