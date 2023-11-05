use std::{io::Write, os::unix::prelude::PermissionsExt, path::PathBuf};

use tempfile::TempDir;

pub(crate) fn create_script_file(
    filename: &str,
    code: &str,
) -> Result<(PathBuf, TempDir), std::io::Error> {
    let tmpdir = tempfile::tempdir()?;
    let path = tmpdir.as_ref().join(std::path::Path::new(filename));
    let mut f = std::fs::File::create(&path)?;
    f.write_all(code.as_bytes())?;

    let mut perms = std::fs::metadata(&path)?.permissions();
    perms.set_mode(perms.mode() | 0o111);
    std::fs::set_permissions(&path, perms)?;

    Ok((path, tmpdir))
}
