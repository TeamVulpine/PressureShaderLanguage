use std::{io, ops::Deref, path::Path};

use pawkit_interner::InternString;

pub trait ImportResolver {
    fn resolve_module_identifier(
        &self,
        requesting: &str,
        requested: &str,
    ) -> Result<InternString, io::Error>;

    fn resolve_module_source(&self, module: &str) -> Result<InternString, io::Error>;
}

pub struct FileSystemImportResolver;

impl ImportResolver for FileSystemImportResolver {
    fn resolve_module_identifier(
        &self,
        requesting: &str,
        requested: &str,
    ) -> Result<InternString, io::Error> {
        let requesting = Path::new(requesting);
        let requesting_dir = requesting.parent().unwrap_or(Path::new(""));

        let path = requesting_dir.join(requested);

        let path = path.canonicalize()?;

        return Ok(path.to_string_lossy().deref().into());
    }

    fn resolve_module_source(&self, module: &str) -> Result<InternString, io::Error> {
        return std::fs::read_to_string(module).map(Into::into);
    }
}
