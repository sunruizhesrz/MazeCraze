//! Animation recording and playback system.
//!
//! The [`AnimationRecorder`] captures every intermediate state of a maze
//! generation or solving process as a sequence of [`Frame`]s. The
//! [`AnimationPlayer`] can then play back these frames at various speeds.

pub mod frame;
pub mod player;
pub mod recorder;

pub use frame::Frame;
pub use player::{AnimationPlayer, PlaybackState, Speed};
pub use recorder::AnimationRecorder;
