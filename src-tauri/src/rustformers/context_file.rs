use anyhow::{anyhow, Result};
use std::any::Any;
use std::panic::catch_unwind;
use std::path::PathBuf;
use tracing::info;

/// Reads the provided file and attempts to return a `String` containing
/// the text contents of the file. Multiple file types are supported below
/// and can be extended as needed.
#[tracing::instrument]
pub fn read(path: PathBuf) -> Result<String> {
    let extension = path
        .extension()
        .ok_or(anyhow!("Could not determine file extension"))?
        .to_str()
        .ok_or(anyhow!("Could not convert file extension to string"))?;
    info!(extension = extension, "opening context file");
    match extension {
        "txt" | "md" => std::fs::read_to_string(path).map_err(|err| err.into()),
        "pdf" => extract_pdf(&std::fs::read(path)?),
        "html" => Ok(html2text::from_read(
            std::fs::File::open(path)
                .map_err(|err| anyhow!("Opening file: {}", err.to_string()))?,
            1000,
        )),
        _ => Err(anyhow!("Unsupported file extension {}", extension)),
    }
}

// pdf-extract can panic apparently (https://github.com/jrmuizel/pdf-extract/issues/65)
// so let's catch these errors and report them to the interface instead of crashing.
fn extract_pdf(bytes: &[u8]) -> Result<String> {
    catch_unwind(|| pdf_extract::extract_text_from_mem(&bytes))
        .map_err(|err| anyhow!("PDF extraction panicked: {:#?}", get_panic_message(err)))?
        .map_err(|err| err.into())
}

// https://users.rust-lang.org/t/return-value-from-catch-unwind-is-a-useless-any/89134/6
fn get_panic_message(err: Box<dyn Any + Send>) -> String {
    if let Some(string) = err.downcast_ref::<String>() {
        string.clone()
    } else if let Some(string) = err.downcast_ref::<&str>() {
        string.to_string()
    } else {
        "Unknown panic".to_string()
    }
}