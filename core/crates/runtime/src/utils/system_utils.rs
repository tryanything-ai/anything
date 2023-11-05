use std::{
    io::Write,
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};

use crate::errors::{RuntimeError, RuntimeResult};

pub(crate) fn create_script_file(code: &str) -> RuntimeResult<(PathBuf, tempfile::NamedTempFile)> {
    let tmp = tempfile::NamedTempFile::new()?;
    let path = tmp.path().to_path_buf();
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(code.as_bytes())?;
    Ok((path, tmp))
}

pub(crate) fn set_execute_permission(path: &Path) -> RuntimeResult<()> {
    let mut permissions = std::fs::metadata(path)?.permissions();
    permissions.set_mode(permissions.mode() | 0o111);
    std::fs::set_permissions(path, permissions).map_err(RuntimeError::IoError)
}
