use crate::core::Grid;

/// 动画序列中的单帧。
///
/// 每一帧都是迷宫网格在特定步骤的快照，
/// 并附带一段人类可读的动作描述。
#[derive(Clone, Debug)]
pub struct Frame {
    /// 当前步骤的网格状态。
    pub grid: Grid,
    /// 步骤编号（从 0 开始）。
    pub step_number: usize,
    /// 动作的人类可读描述。
    pub description: String,
}

impl Frame {
    /// 创建一个新的帧。
    pub fn new(grid: Grid, step_number: usize, description: impl Into<String>) -> Self {
        Self {
            grid,
            step_number,
            description: description.into(),
        }
    }
}
