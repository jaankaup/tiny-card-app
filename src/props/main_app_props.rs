use dioxus::prelude::*;
use crate::mtg_core::mtg_card::MtgCard;

/// Properties that are given to the the main gui application.
#[derive(PartialEq, Props)]
pub struct StartupProps {
    pub app_name: String,
    pub mtg_cards: Vec<MtgCard>,
    // pub errors: String,
}
