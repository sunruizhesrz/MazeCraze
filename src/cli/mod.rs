use clap::Parser;

/// MazeCraze 的命令行参数。
#[derive(Parser, Debug)]
#[command(name = "mazecraze")]
#[command(about = "A TUI interactive maze generator and solver")]
#[command(version)]
pub struct Cli {
    /// 以 TUI 模式运行（默认，当未指定其他标志时）
    #[arg(short, long)]
    pub tui: bool,

    /// 迷宫生成算法
    #[arg(short, long, value_name = "ALGORITHM")]
    pub generate: Option<String>,

    /// 迷宫求解算法
    #[arg(short, long, value_name = "ALGORITHM")]
    pub solve: Option<String>,

    /// 迷宫尺寸（WIDTHxHEIGHT）
    #[arg(short = 'S', long, value_name = "DIMENSIONS", default_value = "21x11")]
    pub size: String,

    /// 将最终迷宫导出到文件
    #[arg(short, long, value_name = "PATH")]
    pub export: Option<String>,

    /// 用于生成非完美迷宫的环路率（0.0-1.0）
    #[arg(long, value_name = "RATE", default_value = "0.0")]
    pub loop_rate: f64,

    /// 使用 ASCII 渲染器替代 Unicode
    #[arg(long)]
    pub ascii: bool,
}

impl Cli {
    /// 将形如 "21x11" 的尺寸字符串解析为 (width, height)。
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
