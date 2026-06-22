//! 动画录制与播放系统。
//!
//! [`AnimationRecorder`] 会将迷宫生成或求解过程的每一个中间状态捕获为
//! 一系列 [`Frame`]。随后 [`AnimationPlayer`] 可以按不同速度回放这些帧。

pub mod frame;
pub mod player;
pub mod recorder;

pub use frame::Frame;
pub use player::{AnimationPlayer, PlaybackState, Speed};
pub use recorder::AnimationRecorder;
