use std::collections::HashMap;

use pawkit_interner::InternString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModuleIndex(pub u32);

pub struct Module {
    pub filename: InternString,
    pub source: InternString,
}

pub struct ModuleCache {
    modules: Vec<Module>,
    mapping: HashMap<InternString, ModuleIndex>,
}

impl ModuleCache {
    pub fn new() -> Self {
        return Self {
            modules: Vec::new(),
            mapping: HashMap::new(),
        };
    }

    pub const fn is_empty(&self) -> bool {
        return self.modules.is_empty();
    }
    pub const fn len(&self) -> usize {
        return self.modules.len();
    }

    pub fn insert(&mut self, filename: InternString, source: InternString) -> Option<ModuleIndex> {
        if self.contains(&filename) {
            return None;
        }

        let index = ModuleIndex(self.len() as u32);

        self.mapping.insert(filename.clone(), index);

        self.modules.push(Module { filename, source });

        return Some(index);
    }

    pub fn get(&self, id: ModuleIndex) -> Option<&Module> {
        return self.modules.get(id.0 as usize);
    }

    pub fn get_by_filename(&self, filename: &InternString) -> Option<(ModuleIndex, &Module)> {
        let index = self.get_index(filename)?;

        let module = self.modules.get(index.0 as usize)?;

        return Some((index, module));
    }

    pub fn contains(&self, filename: &InternString) -> bool {
        return self.mapping.contains_key(filename);
    }

    pub fn get_index(&self, filename: &InternString) -> Option<ModuleIndex> {
        return self.mapping.get(filename).copied();
    }
}
