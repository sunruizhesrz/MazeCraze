//! 迷宫可视化的渲染引擎。
//!
//! 每个渲染器都实现 [`Renderer`] trait，将 [`Grid`] 转换为
//! 人类可读的字符串表示。

pub mod ascii;
pub mod unicode;

pub use ascii::AsciiRenderer;
pub use unicode::UnicodeRenderer;

/// 渲染器将 [`Grid`] 转换为字符串表示。
pub trait Renderer {
    /// 将网格渲染为字符串。
    fn render(&self, grid: &crate::core::Grid) -> String;

    /// 人类可读名称。
    fn name(&self) -> &'static str;
}

/// 可用渲染器的注册表。
pub fn all_renderers() -> Vec<Box<dyn Renderer>> {
    vec![
        Box::new(UnicodeRenderer::new()),
        Box::new(AsciiRenderer::new()),
    ]
}
