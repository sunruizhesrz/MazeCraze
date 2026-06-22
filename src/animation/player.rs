use super::AnimationRecorder;

/// 播放速度倍率。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Speed {
    Slow,    // 0.5x
    Normal,  // 1x
    Fast,    // 2x
    Faster,  // 5x
    Instant, // 跳过动画
}

impl Speed {
    /// 每帧的持续时间（毫秒）。
    pub fn frame_duration_ms(&self) -> u64 {
        match self {
            Speed::Slow => 200,
            Speed::Normal => 100,
            Speed::Fast => 50,
            Speed::Faster => 20,
            Speed::Instant => 0,
        }
    }

    /// 切换到下一档速度。
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

/// 当前播放状态。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Finished,
}

/// 以可控速度回放动画记录器。
pub struct AnimationPlayer {
    recorder: AnimationRecorder,
    current_frame: usize,
    state: PlaybackState,
    speed: Speed,
}

impl AnimationPlayer {
    /// 从记录器创建一个新的播放器。
    pub fn new(recorder: AnimationRecorder) -> Self {
        Self {
            recorder,
            current_frame: 0,
            state: PlaybackState::Playing,
            speed: Speed::Normal,
        }
    }

    /// 当前帧索引。
    pub fn current_frame(&self) -> usize {
        self.current_frame
    }

    /// 总帧数。
    pub fn total_frames(&self) -> usize {
        self.recorder.total_steps()
    }

    /// 当前播放状态。
    pub fn state(&self) -> PlaybackState {
        self.state
    }

    /// 当前速度。
    pub fn speed(&self) -> Speed {
        self.speed
    }

    /// 设置播放速度。
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// 切换播放/暂停。
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

    /// 向前一帧（若正在播放则暂停）。
    pub fn step_forward(&mut self) {
        self.state = PlaybackState::Paused;
        if self.current_frame + 1 < self.total_frames() {
            self.current_frame += 1;
        } else {
            self.state = PlaybackState::Finished;
        }
    }

    /// 向后一帧（若正在播放则暂停）。
    pub fn step_backward(&mut self) {
        self.state = PlaybackState::Paused;
        if self.current_frame > 0 {
            self.current_frame -= 1;
        }
    }

    /// 跳转到最后一帧。
    pub fn jump_to_end(&mut self) {
        self.current_frame = self.total_frames().saturating_sub(1);
        self.state = PlaybackState::Finished;
    }

    /// 跳转到第一帧。
    pub fn jump_to_start(&mut self) {
        self.current_frame = 0;
        self.state = PlaybackState::Paused;
    }

    /// 若正在播放，则推进一帧。
    /// 返回 true 表示帧发生了变化。
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

    /// 访问底层的帧序列。
    pub fn frames(&self) -> &[super::Frame] {
        self.recorder.frames()
    }

    /// 检查动画是否已结束。
    pub fn is_finished(&self) -> bool {
        self.state == PlaybackState::Finished
    }
}
