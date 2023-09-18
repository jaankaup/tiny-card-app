use dioxus_html::input_data::keyboard_types::Code;
use dioxus_desktop::{Config, WindowBuilder};
use dioxus::prelude::*;

use tiny_card_app::props::main_app_props::StartupProps;

fn main() {


    let config = Config::new()
                 .with_window(WindowBuilder::default()
                 .with_title("Mtg booster generator")
                 .with_inner_size(dioxus_desktop::LogicalSize::new(1920, 1080))
    );

    dioxus_desktop::launch_with_props(MainView, StartupProps {app_name: "heko".to_string()}, config);
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    cx.render(rsx! {

        // Top level layout.
        div {
            
        }
    })
}
