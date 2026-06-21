use super::AnimationRecorder;

/// Playback speed multiplier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    Slow,    // 0.5x
    Normal,  // 1x
    Fast,    // 2x
    Faster,  // 5x
    Instant, // skip animation
}

impl Speed {
    /// Duration per frame in milliseconds.
    pub fn frame_duration_ms(&self) -> u64 {
        match self {
            Speed::Slow => 200,
            Speed::Normal => 100,
            Speed::Fast => 50,
            Speed::Faster => 20,
            Speed::Instant => 0,
        }
    }

    /// Cycle to the next speed.
    pub fn next(&self) -> Self {
        match self {
            Speed::Slow => Speed::Normal,
            Speed::Normal => Speed::Fast,
            Speed::Fast => Speed::Faster,
            Speed::Faster => Speed::Instant,
            Speed::Instant => Speed::Slow,
        }
    }
}

/// Current playback state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Finished,
}

/// Plays back an animation recorder at a controlled speed.
pub struct AnimationPlayer {
    recorder: AnimationRecorder,
    current_frame: usize,
    state: PlaybackState,
    speed: Speed,
}

impl AnimationPlayer {
    /// Create a new player from a recorder.
    pub fn new(recorder: AnimationRecorder) -> Self {
        Self {
            recorder,
            current_frame: 0,
            state: PlaybackState::Playing,
            speed: Speed::Normal,
        }
    }

    /// Current frame index.
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// Total number of frames.
    pub fn total_frames(&self) -> usize {
        self.recorder.total_steps()
    }

    /// Current playback state.
    pub fn state(&self) -> PlaybackState {
        self.state
    }

    /// Current speed.
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// Set playback speed.
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Toggle play/pause.
    pub fn toggle_playback(&mut self) {
        self.state = match self.state {
            PlaybackState::Playing => PlaybackState::Paused,
            PlaybackState::Paused => PlaybackState::Playing,
            PlaybackState::Finished => {
                self.current_frame = 0;
                PlaybackState::Playing
            }
        };
    }

    /// Step forward by one frame (pauses if playing).
    pub fn step_forward(&mut self) {
        self.state = PlaybackState::Paused;
        if self.current_frame + 1 < self.total_frames() {
            self.current_frame += 1;
        } else {
            self.state = PlaybackState::Finished;
        }
    }

    /// Step backward by one frame (pauses if playing).
    pub fn step_backward(&mut self) {
        self.state = PlaybackState::Paused;
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }

    /// Jump to the last frame.
    pub fn jump_to_end(&mut self) {
        self.current_frame = self.total_frames().saturating_sub(1);
        self.state = PlaybackState::Finished;
    }

    /// Jump to the first frame.
    pub fn jump_to_start(&mut self) {
        self.current_frame = 0;
        self.state = PlaybackState::Paused;
    }

    /// Advance playback by one frame if currently playing.
    /// Returns true if the frame changed.
    pub fn tick(&mut self) -> bool {
        if self.state != PlaybackState::Playing {
            return false;
        }
        if self.current_frame + 1 < self.total_frames() {
            self.current_frame += 1;
            true
        } else {
            self.state = PlaybackState::Finished;
            false
        }
    }

    /// Access the underlying frames.
    pub fn frames(&self) -> &[super::Frame] {
        self.recorder.frames()
    }

    /// Check if the animation is finished.
    pub fn is_finished(&self) -> bool {
        self.state == PlaybackState::Finished
    }
}
