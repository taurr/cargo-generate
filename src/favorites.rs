use crate::{
    app_config::{AppConfig, FavoriteConfig},
    Args,
};
use anyhow::{Context, Result};

pub(crate) fn list_favorites(app_config: &AppConfig, args: &Args) {
    println!("Listing favorites");

    let data = {
        let mut d = app_config
            .favorites
            .iter()
            .filter(|(key, _)| args.favorite.as_ref().map_or(true, |f| key.starts_with(f)))
            .collect::<Vec<(&String, &FavoriteConfig)>>();
        d.sort_by_key(|(key, _)| key.to_string());
        d
    };

    println!("Possible favorites:");
    let longest_key = data.iter().map(|(k, _)| k.len()).max().unwrap_or(0);
    let longest_key = ((longest_key + 5) / 4) * 4;
    data.iter().for_each(|(key, conf)| {
        println!(
            "{}:{}{}",
            key,
            " ".repeat(longest_key - key.len()),
            conf.description.as_ref().cloned().unwrap_or_default()
        );
    });
}

pub(crate) fn resolve_favorite<'a>(
    app_config: &'a AppConfig,
    args: &mut Args,
) -> Result<Option<&'a FavoriteConfig>> {
    if args.git.is_some() {
        return Ok(None);
    }

    let favorite_name = args
        .favorite
        .as_ref()
        .context("Please specify either --git option, or a predefined favorite")?;

    let favorite = app_config
        .favorites
        .get(favorite_name.as_str())
        .with_context(|| {
            format!(
                "Unknown favorite: {}\nFavorites must be defined in the configuration file",
                favorite_name,
            )
        })?;

    if let Some(values) = favorite.template_values.clone() {
        let mut favorite_values = vec![values];
        favorite_values.append(&mut args.template_values_file.take().unwrap_or_default());
        args.template_values_file = Some(favorite_values);
    }
    args.git = favorite.git.clone();
    args.branch = args
        .branch
        .as_ref()
        .or_else(|| favorite.branch.as_ref())
        .cloned();

    Ok(Some(favorite))
}
