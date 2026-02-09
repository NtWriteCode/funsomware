/// Library interface for integration tests
/// Exposes select config values for testing without exposing internals

mod config;

/// Get the configured password (for testing)
pub fn config_password() -> &'static str {
    config::PASSWORD
}

/// Get the configured thread count (for testing)
pub fn config_thread_count() -> usize {
    config::THREAD_COUNT
}
