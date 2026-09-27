use std::collections::{HashMap, HashSet};

use pawkit_interner::InternString;

use crate::{
    library::import::{FileSystemImportResolver, ImportResolver},
    module_cache::{ModuleCache, ModuleIndex},
};

pub struct LibraryBuilder<R: ImportResolver> {
    import_resolver: R,

    root_modules: Vec<InternString>,
}

impl LibraryBuilder<FileSystemImportResolver> {
    pub fn new() -> Self {
        return Self {
            import_resolver: FileSystemImportResolver,
            root_modules: Vec::new(),
        };
    }
}

impl<R: ImportResolver> LibraryBuilder<R> {
    pub fn with_import_resolver<NR: ImportResolver>(
        self,
        import_resolver: NR,
    ) -> LibraryBuilder<NR> {
        return LibraryBuilder {
            import_resolver,
            root_modules: self.root_modules,
        };
    }

    pub fn add_root_module(self, module: InternString) -> Self {
        return self.add_root_modules(std::iter::once(module));
    }

    pub fn add_root_modules(mut self, modules: impl IntoIterator<Item = InternString>) -> Self {
        self.root_modules.extend(modules);

        return self;
    }

    pub fn build(self) {
        let mut cache = ModuleCache::new();

        let mut modules_to_resolve = self.root_modules;

        let import_resolver = self.import_resolver;

        let mut discovered_modules = HashSet::<InternString>::new();
        discovered_modules.extend(modules_to_resolve.iter().cloned());

        // TODO: Make it a hashmap of ModuleIndex to Document, once Document is implemented
        let mut parsed_modules = HashMap::<ModuleIndex, ()>::new();

        while let Some(module) = modules_to_resolve.pop() {
            let source = import_resolver
                .resolve_module_source(&module)
                .expect("TODO: Handle this");

            let index = cache
                .insert(module, source.into())
                .expect("TODO: Handle this");

            parsed_modules.insert(index, ());
        }
    }
}
