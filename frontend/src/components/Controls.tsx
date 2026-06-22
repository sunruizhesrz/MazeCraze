import { GENERATOR_NAMES } from '../algorithms/generators';
import { SOLVER_NAMES } from '../algorithms/solvers';

interface Props {
  generator: string;
  solver: string;
  difficulty: string;
  width: number;
  height: number;
  speed: number;
  isPlaying: boolean;
  hasFrames: boolean;
  isChallengeActive: boolean;
  onGeneratorChange: (g: string) => void;
  onSolverChange: (s: string) => void;
  onDifficultyChange: (difficulty: string) => void;
  onWidthChange: (w: number) => void;
  onHeightChange: (h: number) => void;
  onStartChallenge: () => void;
  onGenerate: () => void;
  onSolve: () => void;
  onPlayPause: () => void;
  onStepForward: () => void;
  onStepBackward: () => void;
  onSpeedChange: (s: number) => void;
  onReset: () => void;
}

export default function Controls({
  generator,
  solver,
  difficulty,
  width,
  height,
  speed,
  isPlaying,
  hasFrames,
  isChallengeActive,
  onGeneratorChange,
  onSolverChange,
  onDifficultyChange,
  onWidthChange,
  onHeightChange,
  onStartChallenge,
  onGenerate,
  onSolve,
  onPlayPause,
  onStepForward,
  onStepBackward,
  onSpeedChange,
  onReset,
}: Props) {
  return (
    <div className="controls-panel">
      {/* 挑战模式 */}
      <div className="control-group">
        <h3>挑战模式</h3>
        <div className="difficulty-options" role="group" aria-label="挑战难度">
          {[
            ['easy', '简单'],
            ['normal', '普通'],
            ['hard', '困难'],
          ].map(([key, label]) => (
            <button
              key={key}
              className={difficulty === key ? 'difficulty-btn active' : 'difficulty-btn'}
              type="button"
              onClick={() => onDifficultyChange(key)}
            >
              {label}
            </button>
          ))}
        </div>
        <button className="btn-primary challenge-start" onClick={onStartChallenge}>
          {isChallengeActive ? '重新挑战' : '开始挑战'}
        </button>
      </div>

      {/* 生成器选择 */}
      <div className="control-group">
        <h3>生成算法</h3>
        <select value={generator} onChange={(e) => onGeneratorChange(e.target.value)}>
          {Object.entries(GENERATOR_NAMES).map(([key, name]) => (
            <option key={key} value={key}>{name}</option>
          ))}
        </select>
      </div>

      {/* 求解器选择 */}
      <div className="control-group">
        <h3>求解算法</h3>
        <select value={solver} onChange={(e) => onSolverChange(e.target.value)}>
          {Object.entries(SOLVER_NAMES).map(([key, name]) => (
            <option key={key} value={key}>{name}</option>
          ))}
        </select>
      </div>

      {/* 尺寸调节 */}
      <div className="control-group">
        <h3>迷宫尺寸</h3>
        <div className="size-inputs">
          <label>
            宽：
            <input
              type="range"
              min={5}
              max={51}
              step={2}
              value={width}
              onChange={(e) => onWidthChange(Number(e.target.value))}
            />
            <span>{width}</span>
          </label>
          <label>
            高：
            <input
              type="range"
              min={5}
              max={51}
              step={2}
              value={height}
              onChange={(e) => onHeightChange(Number(e.target.value))}
            />
            <span>{height}</span>
          </label>
        </div>
      </div>

      {/* 操作按钮 */}
      <div className="control-group actions">
        <button className="btn-primary" onClick={onGenerate}>
          生成迷宫
        </button>
        <button className="btn-secondary" onClick={onSolve} disabled={!hasFrames}>
          求解迷宫
        </button>
        <button className="btn-danger" onClick={onReset}>
          重置
        </button>
      </div>

      {/* 播放控制 */}
      <div className="control-group playback">
        <h3>播放控制</h3>
        <div className="playback-buttons">
          <button onClick={onStepBackward}>{'<<'}</button>
          <button className="play-btn" onClick={onPlayPause}>
            {isPlaying ? '||' : '>'}
          </button>
          <button onClick={onStepForward}>{'>>'}</button>
        </div>
        <label className="speed-label">
          速度：{speed}x
          <input
            type="range"
            min={1}
            max={20}
            value={speed}
            onChange={(e) => onSpeedChange(Number(e.target.value))}
          />
        </label>
      </div>
    </div>
  );
}
