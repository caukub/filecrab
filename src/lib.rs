use fluent::bundle::FluentBundle;
use fluent::FluentResource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use unic_langid::LanguageIdentifier;

#[derive(Serialize)]
pub struct AppResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Deserialize)]
pub struct UserPathRequest {
    pub directory: String,
    pub file: Option<String>,
}

pub mod api;
pub mod authorization;
pub mod configuration;
pub mod database;
pub mod error;
pub mod localization;
pub mod routes;
pub mod tracing;

pub use error::*;

pub type Bundle = FluentBundle<FluentResource, intl_memoizer::concurrent::IntlLangMemoizer>;
pub type Locales = HashMap<LanguageIdentifier, Bundle>;

#[derive(Default)]
pub struct Localizer {
    pub locales: Locales,
}

impl Localizer {
    pub fn add_bundle<P>(
        &mut self,
        locale: LanguageIdentifier,
        ftl_paths: &[P],
    ) -> Result<(), anyhow::Error>
    where
        P: std::fmt::Debug + AsRef<Path>,
    {
        let mut bundle = FluentBundle::new_concurrent(vec![locale.clone()]);

        for path in ftl_paths {
            let ftl = std::fs::read_to_string(path)?;
            let ftl = FluentResource::try_new(ftl).unwrap();

            bundle.add_resource_overriding(ftl);
        }

        self.locales.insert(locale, bundle);

        Ok(())
    }
}
