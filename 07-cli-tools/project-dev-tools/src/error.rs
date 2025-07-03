// Error Handling - Fix the Broken Error Types!
//
// TODO: Create comprehensive error handling for the CLI
// Apply lessons from exercise 2 about helpful error messages

// FIXME: Error types are not defined
#[derive(Debug)]
pub enum CliError {
    // TODO: Add error variants for different failure modes
    // FileNotFound(PathBuf),
    // PermissionDenied(String),
    // InvalidFormat(String),
    // ConfigError(String),
    // NetworkError(String),
    // ValidationError(Vec<String>),
}

// TODO: Implement Display for helpful error messages
impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Provide user-friendly error messages with suggestions
        // Example:
        // ❌ File 'input.txt' not found
        // 💡 Check the filename and path, then try again
        
        todo!("Implement user-friendly error display")
    }
}

// TODO: Implement Error trait
impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // TODO: Provide error chaining when appropriate
        None
    }
}

// TODO: Add error conversion helpers
impl From<std::io::Error> for CliError {
    fn from(error: std::io::Error) -> Self {
        // TODO: Convert IO errors to helpful CLI errors
        todo!("Convert IO errors")
    }
}

// TODO: Add result type alias
pub type CliResult<T> = Result<T, CliError>;

// TODO: Add error context helpers
pub trait ErrorContext<T> {
    fn with_context(self, context: &str) -> CliResult<T>;
}

impl<T, E> ErrorContext<T> for Result<T, E> 
where 
    E: std::error::Error + 'static
{
    fn with_context(self, context: &str) -> CliResult<T> {
        // TODO: Add context to errors for better debugging
        todo!("Add error context")
    }
}