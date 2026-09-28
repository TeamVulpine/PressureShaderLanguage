use std::collections::{HashMap, HashSet, hash_map::Entry};

use pawkit_interner::InternString;

use crate::{
    library::{
        attribute::{
            Attribute, AttributeVtable, DuplicateAttributeError,
            intrinsic::create_intrinsic_attributes,
        },
        import::{FileSystemImportResolver, ImportResolver},
    },
    module_cache::{ModuleCache, ModuleIndex},
};

pub struct LibraryBuilder<R: ImportResolver> {
    import_resolver: R,

    root_modules: Vec<InternString>,

    attributes: HashMap<&'static [&'static str], AttributeVtable>,
}

impl LibraryBuilder<FileSystemImportResolver> {
    pub fn new() -> Self {
        return Self {
            import_resolver: FileSystemImportResolver,
            root_modules: Vec::new(),
            attributes: create_intrinsic_attributes(),
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
            attributes: self.attributes,
        };
    }

    pub fn add_root_module(self, module: InternString) -> Self {
        return self.add_root_modules(std::iter::once(module));
    }

    pub fn add_root_modules(mut self, modules: impl IntoIterator<Item = InternString>) -> Self {
        self.root_modules.extend(modules);

        return self;
    }

    pub fn register_attribute<T: Attribute + ttmap::Type>(
        mut self,
    ) -> Result<Self, DuplicateAttributeError> {
        match self.attributes.entry(T::PATH) {
            Entry::Occupied(_) => {
                return Err(DuplicateAttributeError(T::PATH));
            }
            Entry::Vacant(vacant) => {
                let vtable = AttributeVtable::of::<T>();
                vacant.insert(vtable);
                return Ok(self);
            }
        }
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
