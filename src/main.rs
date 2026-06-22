use clap::Parser;
use mazecraze::cli::Cli;
use mazecraze::core::Point;
use mazecraze::generator::find_generator;
use mazecraze::renderer::{AsciiRenderer, Renderer, UnicodeRenderer};
use mazecraze::solver::find_solver;
use mazecraze::tui::run_tui;
use std::path::Path;
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    if cli.tui || (cli.generate.is_none() && cli.solve.is_none()) {
        run_tui()?;
    } else {
        run_cli_mode(cli)?;
    }
    Ok(())
}

fn run_cli_mode(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = cli.parse_size()?;

    // 生成迷宫
    let grid = if let Some(gen_name) = &cli.generate {
        let generator =
            find_generator(gen_name).ok_or_else(|| format!("Unknown generator: {}", gen_name))?;
        println!("Generating maze with {}...", generator.name());
        let recorder = generator.generate(width, height);
        println!("Generation complete: {} steps", recorder.total_steps());
        recorder.frames().last().unwrap().grid.clone()
    } else {
        let generator = find_generator("recursive backtracker").unwrap();
        let recorder = generator.generate(width, height);
        recorder.frames().last().unwrap().grid.clone()
    };

    // 求解迷宫
    let solved_grid = if let Some(solver_name) = &cli.solve {
        let solver =
            find_solver(solver_name).ok_or_else(|| format!("Unknown solver: {}", solver_name))?;
        println!("Solving with {}...", solver.name());
        let start = Point::new(1, 1);
        let end = Point::new(width - 2, height - 2);
        let recorder = solver.solve(&grid, start, end);
        println!("Solving complete: {} steps", recorder.total_steps());
        recorder.frames().last().unwrap().grid.clone()
    } else {
        grid
    };

    // 渲染并打印
    let renderer: Box<dyn Renderer> = if cli.ascii {
        Box::new(AsciiRenderer::new())
    } else {
        Box::new(UnicodeRenderer::new().with_color(false))
    };
    println!("\n{}", renderer.render(&solved_grid));

    // 如有需要则导出
    if let Some(export_path) = &cli.export {
        mazecraze::export::export_grid(&solved_grid, Path::new(export_path))?;
        println!("Maze exported to: {}", export_path);
    }

    Ok(())
}
