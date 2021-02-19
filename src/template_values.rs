use crate::config::ConfigValues;
use crate::emoji;
use crate::Args;

use anyhow::{Context, Result};
use console::style;
use itertools::*;
use std::{
    collections::{hash_map::Iter, HashMap},
    fs,
    path::Path,
};

pub(crate) struct TemplateValues {
    value_map: HashMap<String, toml::Value>,
}

impl TemplateValues {
    pub fn collect_from_files(args: &Args) -> anyhow::Result<Self> {
        let value_map = args
            .template_values_file
            .iter()
            .flat_map(|v| v.iter())
            .map(|path| {
                get_config_file_values(path).with_context(|| {
                    format!(
                        "{} {} {}",
                        emoji::ERROR,
                        style("Failed to read values from file:").bold().red(),
                        style(path).bold().red(),
                    )
                })
            })
            .fold_ok(
                Default::default(),
                |mut m1: HashMap<String, toml::Value>, m2| {
                    m1.extend(m2.into_iter());
                    m1
                },
            )?;

        Ok(Self { value_map })
    }

    pub fn iter(&self) -> Iter<'_, std::string::String, toml::Value> {
        self.value_map.iter()
    }
}

fn get_config_file_values<T>(path: T) -> Result<HashMap<String, toml::Value>>
where
    T: AsRef<Path>,
{
    match fs::read_to_string(path) {
        Ok(ref contents) => toml::from_str::<ConfigValues>(contents)
            .map(|v| v.values)
            .map_err(|e| e.into()),
        Err(e) => anyhow::bail!(
            "{} {} {}",
            emoji::ERROR,
            style("Values File Error:").bold().red(),
            style(e).bold().red(),
        ),
    }
}
