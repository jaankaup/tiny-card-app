use dioxus_html::input_data::keyboard_types::Code;
use dioxus_desktop::{Config, WindowBuilder, launch_with_props};
use dioxus::prelude::*;

use tiny_card_app::props::main_app_props::StartupProps;
use tiny_card_app::mtg_core::mtg_card::{MtgCard, CardImage};
use tiny_card_app::parsers::resource_loader::load_mtg_cards;

fn main() {

    // Load mtg cards.
    let cards = load_mtg_cards();
    println!("{:?}", cards);

    let config = Config::new()
                 .with_window(WindowBuilder::default()
                 .with_title("Mtg booster generator")
                 .with_inner_size(dioxus_desktop::LogicalSize::new(1920, 1080))
    );

    launch_with_props(MainView, StartupProps {app_name: "heko".to_string()}, config);
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
