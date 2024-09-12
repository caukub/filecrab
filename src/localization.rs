use crate::{Locales, Localizer};
use std::path::PathBuf;
use std::str::FromStr;
use unic_langid::LanguageIdentifier;

pub fn get_locales() -> Locales {
    let mut localizer = Localizer::default();
    let cs_identifier = LanguageIdentifier::from_str("cs").unwrap();
    let en_identifier = LanguageIdentifier::from_str("en").unwrap();
    let cs_path = &[PathBuf::from("./locales/cs.ftl")];
    let en_path = &[PathBuf::from("./locales/en.ftl")];
    localizer.add_bundle(cs_identifier, cs_path).unwrap();
    localizer.add_bundle(en_identifier, en_path).unwrap();

    localizer.locales
}
