// Module registry for managing all application modules
use std::collections::HashMap;
use std::sync::Arc;
use super::contracts::Module;

pub struct ModuleRegistry {
    modules: HashMap<String, Arc<dyn Module>>,
}

impl ModuleRegistry {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
        }
    }

    pub fn register(&mut self, module: Arc<dyn Module>) {
        let name = module.name().to_string();
        self.modules.insert(name, module);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Module>> {
        self.modules.get(name)
    }

    pub fn list(&self) -> Vec<&Arc<dyn Module>> {
        self.modules.values().collect()
    }

    pub fn count(&self) -> usize {
        self.modules.len()
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}
