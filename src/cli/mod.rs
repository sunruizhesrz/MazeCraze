use clap::Parser;

/// Command-line arguments for MazeCraze.
#[derive(Parser, Debug)]
#[command(name = "mazecraze")]
#[command(about = "A TUI interactive maze generator and solver")]
#[command(version)]
pub struct Cli {
    /// Run in TUI mode (default if no other flags)
    #[arg(short, long)]
    pub tui: bool,

    /// Maze generator algorithm
    #[arg(short, long, value_name = "ALGORITHM")]
    pub generate: Option<String>,

    /// Maze solver algorithm
    #[arg(short, long, value_name = "ALGORITHM")]
    pub solve: Option<String>,

    /// Maze dimensions (WIDTHxHEIGHT)
    #[arg(short = 'S', long, value_name = "DIMENSIONS", default_value = "21x11")]
    pub size: String,

    /// Export the final maze to a file
    #[arg(short, long, value_name = "PATH")]
    pub export: Option<String>,

    /// Loop rate (0.0-1.0) for non-perfect mazes
    #[arg(long, value_name = "RATE", default_value = "0.0")]
    pub loop_rate: f64,

    /// Use ASCII renderer instead of Unicode
    #[arg(long)]
    pub ascii: bool,
}

impl Cli {
    /// Parse dimensions string like "21x11" into (width, height).
    pub fn parse_size(&self) -> Result<(usize, usize), String> {
        let parts: Vec<&str> = self.size.split('x').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Invalid size format: {}. Expected WIDTHxHEIGHT",
                self.size
            ));
        }
        let width = parts[0]
            .parse::<usize>()
            .map_err(|e| format!("Invalid width: {}", e))?;
        let height = parts[1]
            .parse::<usize>()
            .map_err(|e| format!("Invalid height: {}", e))?;
        Ok((width, height))
    }
}
