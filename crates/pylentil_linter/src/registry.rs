use std::{
    collections::HashMap,
    os::linux::raw::stat,
    sync::{Arc, OnceLock},
};

use pylentil_common::errors::PylentilError;

use crate::lint::Lint;

pub struct LintRegistry {
    registry: HashMap<String, Arc<dyn Lint>>,
}

impl LintRegistry {
    fn build() -> Self {
        let all: Vec<Arc<dyn Lint>> = vec![
            // Arc::new(BareExcept),
        ];

        let mut registry = HashMap::with_capacity(all.len());

        for lint in all {
            let code = lint.code();
            registry.insert(code, lint);
        }

        LintRegistry { registry }
    }

    pub fn get_lint(&self, code: String) -> Result<Arc<dyn Lint>, PylentilError> {
        self.registry
            .get(&code)
            .cloned()
            .ok_or_else(|| PylentilError::InvalidLintCode { code })
    }

    pub fn get_instance() -> &'static LintRegistry {
        static INSTANCE: OnceLock<LintRegistry> = OnceLock::new();
        INSTANCE.get_or_init(LintRegistry::build)
    }
}
