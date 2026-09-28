use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use pylentil_common::errors::PylentilError;

use crate::{lint::Lint, rules::{pycodestyle, pylint}};

pub struct LintRegistry {
    registry: HashMap<&'static str, Arc<dyn Lint>>,
}

impl LintRegistry {
    fn build() -> Self {
        let all: Vec<Arc<dyn Lint>> = vec![
            // Pycodestyle
            Arc::new(pycodestyle::bare_except::BareExcept),

            // Pylint
            Arc::new(pylint::useless_return::UselessReturn),
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
            .get(code.as_str())
            .cloned()
            .ok_or_else(|| PylentilError::InvalidLintCode { code })
    }

    pub fn contains_lint(&self, lint: Arc<dyn Lint>) -> bool {
        self.registry.values().collect::<Vec<_>>().contains(&&lint)
    }

    pub fn get_instance() -> &'static LintRegistry {
        static INSTANCE: OnceLock<LintRegistry> = OnceLock::new();
        INSTANCE.get_or_init(LintRegistry::build)
    }
}
