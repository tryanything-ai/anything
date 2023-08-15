/// Returns a list of all .bin files available in the models directory
/// (with associated metadata if we have them in our models.json file)
/// and if the model is a model that we don't know about, then we return
/// it first.
pub async fn get_available_models() -> Result<Vec<Model>> {
    let dir = get_models_dir()?;
    let mut known_models = AVAILABLE_MODELS.clone();
    let mut models = fs::read_dir(dir)?
        .filter_map(|file| {
            if let Ok(file) = file {
                if let Some(filename) = file.file_name().to_str() {
                    if filename.ends_with(".bin")
                        && !known_models.iter().any(|m| m.filename.as_str() == filename)
                    {
                        return Some(Model {
                            name: filename.to_string(),
                            filename: filename.to_string(),
                            custom: true,
                            ..Default::default()
                        });
                    }
                }
            }
            None
        })
        .collect::<Vec<_>>();
    models.append(&mut known_models);
    models.sort_by(|a, b| b.custom.cmp(&a.custom));
    Ok(models)
}