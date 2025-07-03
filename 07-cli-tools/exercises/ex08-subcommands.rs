// Exercise 8: Subcommands - Fix the Broken Git-Style Command Structure!
//
// EXERCISE PROGRESS: [░░░░░░░░░░] 0% Complete (7 subcommand features to fix)
//
// Your task: Create a complex CLI with subcommands like git, cargo, or docker
// This demonstrates advanced CLI architecture and command organization
//
// INSTRUCTIONS:
// 1. Run program to see it doesn't handle subcommands properly
// 2. Fix subcommand parsing and dispatch
// 3. Test: `cargo run --bin ex08-subcommands -- file process input.txt`
// 4. Add help, aliases, and nested subcommands
//
// LEARNING STRATEGY:
// - Start with basic subcommand enum and parsing
// - Add proper help for each subcommand
// - Implement command aliases and shortcuts
// - Add nested subcommands (like `git remote add`)
//
// SUBCOMMAND FEATURES TO FIX:
// [] Define subcommand structure with clap
// [] Implement proper command dispatch
// [] Add subcommand-specific help
// [] Create command aliases and shortcuts
// [] Handle global vs per-command options
// [] Implement nested subcommands
// [] Add shell completion support

use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

// FIXME: This CLI structure is broken and incomplete
#[derive(Parser)]
#[clap(name = "devtool", version = "1.0.0")]
#[clap(about = "A developer tool with multiple commands")]
#[clap(long_about = None)]
struct Cli {
    /// Enable verbose output for all commands
    #[clap(short, long, global = true)]
    verbose: bool,
    
    /// Config file path (applies to all commands)
    #[clap(short, long, global = true)]
    config: Option<PathBuf>,
    
    #[clap(subcommand)]
    command: Commands,  // FIXME: Commands enum doesn't exist!
}

// TODO: Define the subcommand structure
// HINT: Like creating different controllers in ASP.NET MVC
#[derive(Subcommand)]
enum Commands {
    // TODO: Add subcommands for file operations
    // File { /* file-specific args */ },
    
    // TODO: Add subcommands for database operations  
    // Database { /* db-specific args */ },
    
    // TODO: Add subcommands for server operations
    // Server { /* server-specific args */ },
}

// TODO: Define args for file operations
#[derive(Args)]
struct FileArgs {
    // TODO: File subcommands like process, validate, convert
    // #[clap(subcommand)]
    // command: FileCommands,
}

// TODO: Define file subcommands
#[derive(Subcommand)]
enum FileCommands {
    // Process files: devtool file process input.txt
    // Process {
    //     input: PathBuf,
    //     #[clap(short, long)]
    //     output: Option<PathBuf>,
    // },
    
    // Validate files: devtool file validate *.txt
    // Validate {
    //     files: Vec<PathBuf>,
    //     #[clap(short, long)]
    //     strict: bool,
    // },
    
    // Convert files: devtool file convert input.csv --to json
    // Convert {
    //     input: PathBuf,
    //     #[clap(long, value_enum)]
    //     to: OutputFormat,
    // },
}

// TODO: Define output format enum
#[derive(clap::ValueEnum, Clone)]
enum OutputFormat {
    // Json,
    // Yaml, 
    // Toml,
    // Csv,
}

// FIXME: This main function doesn't handle subcommands
fn main() {
    // TERRIBLE: Just tries to parse without proper subcommand handling
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: devtool <command> [options]");
        println!("Commands: file, database, server");
        return;
    }
    
    let command = &args[1];
    
    // TERRIBLE: Manual string matching instead of proper parsing
    match command.as_str() {
        "file" => {
            println!("File command - not implemented!");  // FIXME: No actual implementation!
        },
        "database" => {
            println!("Database command - not implemented!");  // FIXME: No actual implementation!
        },
        "server" => {
            println!("Server command - not implemented!");  // FIXME: No actual implementation!
        },
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

// TODO: Implement proper main with clap parsing
fn main_fixed() {
    let cli = Cli::parse();
    
    // TODO: Set up global options (verbose, config)
    if cli.verbose {
        println!("Verbose mode enabled");
    }
    
    if let Some(config_path) = &cli.config {
        println!("Using config: {}", config_path.display());
    }
    
    // TODO: Dispatch to appropriate command handler
    match cli.command {
        // Commands::File(args) => handle_file_command(args),
        // Commands::Database(args) => handle_database_command(args),  
        // Commands::Server(args) => handle_server_command(args),
    }
}

// TODO: Implement file command handler
fn handle_file_command(args: FileArgs) {
    // TODO: Handle file subcommands
    // match args.command {
    //     FileCommands::Process { input, output } => {
    //         process_file(input, output)
    //     },
    //     FileCommands::Validate { files, strict } => {
    //         validate_files(files, strict)
    //     },
    //     FileCommands::Convert { input, to } => {
    //         convert_file(input, to)
    //     },
    // }
    todo!("Implement file command handling")
}

// TODO: Implement individual file operations
fn process_file(input: PathBuf, output: Option<PathBuf>) {
    println!("Processing file: {}", input.display());
    if let Some(out) = output {
        println!("Output to: {}", out.display());
    }
    // TODO: Actual file processing logic
}

fn validate_files(files: Vec<PathBuf>, strict: bool) {
    println!("Validating {} files (strict: {})", files.len(), strict);
    for file in files {
        println!("  Validating: {}", file.display());
        // TODO: Actual validation logic
    }
}

fn convert_file(input: PathBuf, to: OutputFormat) {
    println!("Converting {} to {:?}", input.display(), to);
    // TODO: Actual conversion logic
}

// TODO: Implement database command structure
#[derive(Args)]
struct DatabaseArgs {
    /// Database connection URL
    #[clap(short, long, env = "DATABASE_URL")]
    url: String,
    
    #[clap(subcommand)]
    command: DatabaseCommands,
}

#[derive(Subcommand)]
enum DatabaseCommands {
    /// Connect to database and run queries
    Connect {
        /// SQL query to execute
        #[clap(short, long)]
        query: Option<String>,
    },
    
    /// Create database migration
    Migrate {
        /// Migration name
        name: String,
        
        /// Run migration up
        #[clap(long)]
        up: bool,
        
        /// Run migration down  
        #[clap(long)]
        down: bool,
    },
    
    /// Backup database
    Backup {
        /// Output file for backup
        output: PathBuf,
        
        /// Compress backup
        #[clap(short, long)]
        compress: bool,
    },
}

// TODO: Implement server command structure  
#[derive(Args)]
struct ServerArgs {
    #[clap(subcommand)]
    command: ServerCommands,
}

#[derive(Subcommand)]
enum ServerCommands {
    /// Start the server
    Start {
        /// Port to listen on
        #[clap(short, long, default_value = "8080")]
        port: u16,
        
        /// Host to bind to
        #[clap(long, default_value = "localhost")]
        host: String,
        
        /// Run in daemon mode
        #[clap(short, long)]
        daemon: bool,
    },
    
    /// Stop the server
    Stop {
        /// Force stop without graceful shutdown
        #[clap(short, long)]
        force: bool,
    },
    
    /// Show server status
    Status,
    
    /// Server configuration management
    Config {
        #[clap(subcommand)]
        action: ConfigActions,
    },
}

// TODO: Nested subcommands example
#[derive(Subcommand)]
enum ConfigActions {
    /// Show current configuration
    Show,
    
    /// Set a configuration value
    Set {
        /// Configuration key
        key: String,
        /// Configuration value
        value: String,
    },
    
    /// Get a configuration value
    Get {
        /// Configuration key
        key: String,
    },
    
    /// Reset configuration to defaults
    Reset {
        /// Confirm reset without prompting
        #[clap(short, long)]
        yes: bool,
    },
}

// TODO: Implement command aliases and shortcuts
fn setup_aliases() {
    // TODO: Allow shortcuts like:
    // devtool f p input.txt  (instead of devtool file process input.txt)
    // devtool db c           (instead of devtool database connect)
    // devtool s start        (instead of devtool server start)
    
    // HINT: Use clap's alias attribute or custom parsing
}

// TODO: Add shell completion support
fn generate_completions() {
    // TODO: Generate shell completions for bash, zsh, fish
    // Should complete:
    // - Command names
    // - Option names  
    // - File paths
    // - Enum values
    
    // HINT: Use clap_complete crate
    println!("TODO: Generate shell completions");
}

// TODO: Add custom help formatting
fn custom_help() {
    // TODO: Create custom help that shows:
    // - Command examples
    // - Common usage patterns
    // - Tips and tricks
    
    println!("TODO: Implement custom help formatting");
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    
    #[test]
    fn verify_cli() {
        // TODO: Test that CLI structure is valid
        Cli::command().debug_assert()
    }
    
    #[test]
    fn test_subcommand_parsing() {
        // TODO: Test parsing of various subcommand combinations
        // devtool file process input.txt
        // devtool database migrate test --up
        // devtool server start --port 3000
        todo!("Test subcommand parsing")
    }
    
    #[test]
    fn test_global_options() {
        // TODO: Test that global options work with all subcommands
        // devtool --verbose file process input.txt
        // devtool --config custom.toml server start
        todo!("Test global options")
    }
    
    #[test]
    fn test_help_output() {
        // TODO: Test that help is generated correctly for all commands
        todo!("Test help output")
    }
}

// SUBCOMMAND EXAMPLES:
//
// BASIC USAGE:
// devtool file process input.txt --output result.txt
// devtool database connect --query "SELECT * FROM users"
// devtool server start --port 8080 --daemon
//
// WITH GLOBAL OPTIONS:
// devtool --verbose file validate *.txt
// devtool --config prod.toml server start
//
// NESTED SUBCOMMANDS:
// devtool server config show
// devtool server config set port 9000
// devtool server config reset --yes
//
// ALIASES (TODO):
// devtool f p input.txt    (file process)
// devtool db c             (database connect)
// devtool s start          (server start)
//
// HELP EXAMPLES:
// devtool --help                    # Main help
// devtool file --help               # File command help
// devtool file process --help       # File process subcommand help
// devtool server config --help      # Nested command help
//
// COMMAND STRUCTURE PATTERNS:
// git-style:   git remote add origin url
// cargo-style: cargo build --release
// docker-style: docker container run --rm image
// npm-style:   npm run build
//
// C# COMPARISON:
// C#: CommandLineParser with verbs, ASP.NET Core controllers
// Rust: clap Subcommand derive, enum-based dispatch
//
// C#: [Verb("process", HelpText = "Process files")]
// Rust: #[clap(about = "Process files")]
//
// C#: switch (parsedArgs.Verb) { case "process" => ... }
// Rust: match cli.command { Commands::File(args) => ... }
//
// 📊 PROGRESS TRACKER:
// Feature 1 (Basic subcommands): [ ] Working
// Feature 2 (Command dispatch): [ ] Working
// Feature 3 (Subcommand help): [ ] Working
// Feature 4 (Global options): [ ] Working
// Feature 5 (Nested commands): [ ] Working
// Feature 6 (Aliases): [ ] Working
// Feature 7 (Completions): [ ] Working
//
// 🎯 SUCCESS CRITERIA:
// ✅ Multiple levels of subcommands work
// ✅ Global options apply to all commands
// ✅ Help is available for every command level
// ✅ Command structure is intuitive and discoverable
// ✅ Shell completions work properly
// ✅ Error messages are helpful and specific
// ✅ Command aliases provide convenient shortcuts
//
// 🚀 FINAL CHALLENGE:
// You've completed all exercises! Now build the dev-tools project!