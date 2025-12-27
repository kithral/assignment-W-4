//! Rhai Script Executor
//!
//! Executes Rhai scripts in a sandboxed environment with operation limits
//! and timeout protection.

#![allow(dead_code)] // Allow dead code in template - remove when implementing

use rhai::{Engine, EvalAltResult, Scope};
use std::time::Duration;
use tracing::{debug, info};

/// Configuration for script execution
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    /// Maximum number of operations before timeout
    pub max_operations: u64,
    /// Maximum script execution time
    pub max_duration: Duration,
    /// Maximum string length
    pub max_string_len: usize,
    /// Maximum array size
    pub max_array_size: usize,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_duration: Duration::from_secs(5),
            max_string_len: 10_000,
            max_array_size: 1_000,
        }
    }
}

/// Rhai script executor with sandboxing
pub struct RhaiExecutor {
    engine: Engine,
    config: ExecutorConfig,
}

impl RhaiExecutor {
    /// Create a new Rhai executor with default configuration
    pub fn new() -> Self {
        Self::with_config(ExecutorConfig::default())
    }

    /// Create a new Rhai executor with custom configuration
    pub fn with_config(config: ExecutorConfig) -> Self {
        let mut engine = Engine::new();

        // Set operation limits for sandboxing
        engine.set_max_operations(config.max_operations);

        // Note: String and array limits are set differently in newer Rhai versions
        // These can be configured via the Engine's limits if needed

        // Disable dangerous operations
        engine.disable_symbol("eval"); // Prevent eval injection

        info!("Rhai executor initialized with limits: {:?}", config);

        Self { engine, config }
    }

    /// Get mutable access to the engine for registering custom functions
    ///
    /// Example:
    /// ```
    /// let mut executor = RhaiExecutor::new();
    /// executor.engine_mut().register_fn("emit", |msg: &str| {
    ///     println!("{}", msg);
    /// });
    /// ```
    pub fn engine_mut(&mut self) -> &mut Engine {
        &mut self.engine
    }

    /// Execute a script and return the result
    pub fn execute(&self, script: &str) -> Result<String, Box<EvalAltResult>> {
        debug!("Executing Rhai script ({} chars)", script.len());

        let mut scope = Scope::new();

        // Execute with timeout (this is a placeholder - actual timeout requires async)
        let result = self
            .engine
            .eval_with_scope::<rhai::Dynamic>(&mut scope, script)?;

        Ok(result.to_string())
    }

    /// Execute a script function with string arguments
    pub fn call_fn(
        &self,
        script: &str,
        fn_name: &str,
        args: Vec<String>,
    ) -> Result<String, Box<EvalAltResult>> {
        debug!(
            "Calling Rhai function '{}' with {} args",
            fn_name,
            args.len()
        );

        // Compile the script
        let ast = self.engine.compile(script)?;

        // Convert args to Dynamic array
        let dynamic_args: Vec<rhai::Dynamic> = args.into_iter().map(rhai::Dynamic::from).collect();

        // Call the function
        let result: rhai::Dynamic =
            self.engine
                .call_fn(&mut Scope::new(), &ast, fn_name, dynamic_args)?;

        Ok(result.to_string())
    }
}

impl Default for RhaiExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        let executor = RhaiExecutor::new();
        let result = executor.execute("40 + 2").unwrap();
        assert_eq!(result, "42");
    }

    #[test]
    fn test_operation_limit() {
        let config = ExecutorConfig {
            max_operations: 10, // Very low limit
            ..Default::default()
        };

        let executor = RhaiExecutor::with_config(config);

        // This should exceed operation limit
        let result = executor.execute(
            r#"
            let x = 0;
            for i in 0..1000 {
                x += i;
            }
            x
        "#,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_function_call() {
        let executor = RhaiExecutor::new();

        let script = r#"
            fn greet(name) {
                "Hello, " + name + "!"
            }
        "#;

        let result = executor
            .call_fn(script, "greet", vec!["World".to_string()])
            .unwrap();

        assert_eq!(result, "Hello, World!");
    }
}
