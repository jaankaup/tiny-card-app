// use dioxus_html::input_data::keyboard_types::Code;
use std::collections::HashMap;
use dioxus_desktop::{Config, WindowBuilder, launch_with_props};
use dioxus::prelude::*;

use tiny_card_app::props::main_app_props::StartupProps;
use tiny_card_app::mtg_core::mtg_card::{MtgCard, CardImage};
use tiny_card_app::mtg_core::mana_symbol::{ManaSymbol, ManaType};
use tiny_card_app::parsers::resource_loader::load_mtg_cards;

fn main() {

    // Load mtg cards.
    let cards = load_mtg_cards().expect("Failed to load mtg cards.");

    let config = Config::new()
                 .with_window(WindowBuilder::default()
                 .with_title("Mtg booster generator")
                 .with_inner_size(dioxus_desktop::LogicalSize::new(1920, 1080))
    );

    launch_with_props(
        MainView,
        StartupProps {
            app_name: "heko".to_string(),
            mtg_cards: cards,
        },
        config
    );
}

/// The main component of the application.
#[allow(non_snake_case)]
fn MainView(cx: Scope<StartupProps>) -> Element {

    //let mana = use_state(cx, || [Some(ManaType::Plains), Some(ManaType::Mountain), Some(ManaType::Forest), Some(ManaType::Island), Some(ManaType::Swamp]));
    //let on_symbol_clicked = move |mt: ManaType| {
    //   match mt {
    //       ManaType::Plains => {
    //           
    //       },
    //       ManaType::Mountain => {
    //           
    //       },
    //       ManaType::Forest => {
    //           
    //       },
    //       ManaType::Island => {
    //           
    //       },
    //       ManaType::Swamp => {
    //           
    //       },
    //   } 
    //}; 

    let cards = use_state(cx, || cx.props.mtg_cards.to_owned());
    let mana_checked = use_state(cx, || HashMap::from(
            [(ManaType::Plains, false), (ManaType::Mountain, false),
             (ManaType::Forest, false), (ManaType::Island, false),
             (ManaType::Swamp, false)])
    );
    let white_checked = use_state(cx, || false);
    let red_checked = use_state(cx, || false);
    let green_checked = use_state(cx, || false);
    let blue_checked = use_state(cx, || false);
    let black_checked = use_state(cx, || false);

    cx.render(rsx! {

        // Top level layout.
        div {
            // div {
            //     for i in 0..1000 {
            //         CardImage { mtg_card: &cards.get()[i] }
            //     }
            // }

        },
        render! {
            ManaSymbol { mana_type: ManaType::Plains, on_clicked: move |event: bool | { mana_checked.with_mut(|m|{ m.insert(ManaType::Plains, event);}); } },
            ManaSymbol { mana_type: ManaType::Swamp, on_clicked: move |event: bool | { mana_checked.with_mut(|m|{ m.insert(ManaType::Swamp, event);}); } },
            ManaSymbol { mana_type: ManaType::Island, on_clicked: move |event: bool | { mana_checked.with_mut(|m|{ m.insert(ManaType::Island, event);}); } },
            ManaSymbol { mana_type: ManaType::Mountain, on_clicked: move |event: bool | { mana_checked.with_mut(|m|{ m.insert(ManaType::Mountain, event);}); } },
            ManaSymbol { mana_type: ManaType::Forest, on_clicked: move |event: bool | { mana_checked.with_mut(|m|{ m.insert(ManaType::Forest, event);}); } },
        }
    })
}
