// Contracts for dependency injection

/// Module trait - all modules must implement this
pub trait Module: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn is_enabled(&self) -> bool {
        true
    }
}

/// Lifecycle trait for modules that need initialization
#[async_trait::async_trait]
pub trait LifecycleModule: Module {
    async fn initialize(&self) -> Result<(), String>;
    async fn shutdown(&self) -> Result<(), String>;
}
