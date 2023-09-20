use crate::misc_stuff::loadFile;
use std::borrow::Cow;
use hard_xml::{XmlRead, XmlWrite};
use crate::mtg_core::mtg_card::MtgCard;

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "listofcarddatafiles")]
pub struct CardDataFiles<'a> {
    #[xml(child = "filetoinclude")]
    pub files: Vec<FileToInclude<'a>>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "filetoinclude")]
pub struct FileToInclude<'a> {
    #[xml(text)]
    pub text: Cow<'a, str>,
}

/// Load all mtg cards.
pub fn load_mtg_cards() -> Result<Vec<MtgCard>, String> {

    let mut result: Vec<MtgCard> = Vec::with_capacity(60000);
    let mut error = String::new();

    let file_str = loadFile("ListOfCardDataFiles.txt").map_err(|_| "Failed to load file 'ListOfCardDataFiles.txt'".to_string())?;
    let card_files = CardDataFiles::from_str(&file_str).map_err(|_|"Failed to parse file 'ListOfCardDataFiles.txt'".to_string())?;

    for dataFile in card_files.files.iter() {
        let file = String::from("sets/") + &dataFile.text;
        parse_mtg_cards(&mut result, &file);
    }

    Ok(result)
}

/// Parse all mtg cards from the given file. Append parsed cards to given cards vector. 
fn parse_mtg_cards(cards: &mut Vec<MtgCard>, file: &str) -> Result<(), String> {

    let src = loadFile(file).map_err(|_| format!("Failed to load file {:?}", file).to_string())?;

    let mut result: Result<(), String> = Ok(());
    for s in src.lines() {
        if let Some(card) = MtgCard::parse(&s) {
            cards.push(card);
        }
    }
    result
}
