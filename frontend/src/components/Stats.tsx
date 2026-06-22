import { Cell } from '../core/types';

interface Props {
  step: number;
  totalSteps: number;
  message: string;
  grid: number[][];
  algorithm: string;
  mode: 'generate' | 'solve' | 'idle' | 'challenge';
  challenge?: {
    difficulty: string;
    elapsed: string;
    steps: number;
    best: string;
    finished: boolean;
  };
}

export default function Stats({ step, totalSteps, message, grid, algorithm, mode, challenge }: Props) {
  // 展平网格并统计各类单元格数量
  const flatGrid = grid.flat();
  const pathLength = flatGrid.filter((c) => c === Cell.Path).length;
  const visitedCount = flatGrid.filter((c) => c === Cell.Visited || c === Cell.Current).length;
  const passageCount = flatGrid.filter((c) => c === Cell.Passage).length;

  return (
    <div className="stats-panel">
      <h3>统计信息</h3>
      <div className="stat-row">
        <span className="stat-label">模式：</span>
        <span className="stat-value mode-badge" data-mode={mode}>
          {mode === 'generate'
            ? '生成中'
            : mode === 'solve'
              ? '求解中'
              : mode === 'challenge'
                ? challenge?.finished
                  ? '已完成'
                  : '挑战中'
                : '空闲'}
        </span>
      </div>
      {challenge && (
        <>
          <div className="stat-row">
            <span className="stat-label">难度：</span>
            <span className="stat-value">{challenge.difficulty}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">用时：</span>
            <span className="stat-value">{challenge.elapsed}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">步数：</span>
            <span className="stat-value">{challenge.steps}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">最佳：</span>
            <span className="stat-value">{challenge.best}</span>
          </div>
        </>
      )}
      <div className="stat-row">
        <span className="stat-label">算法：</span>
        <span className="stat-value">{algorithm || '-'}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">帧数：</span>
        <span className="stat-value">{step} / {totalSteps}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">进度：</span>
        <span className="stat-value">
          {totalSteps > 0 ? Math.round((step / totalSteps) * 100) : 0}%
        </span>
      </div>
      <div className="stat-row">
        <span className="stat-label">路径长度：</span>
        <span className="stat-value">{pathLength}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">已访问：</span>
        <span className="stat-value">{visitedCount}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">通道数：</span>
        <span className="stat-value">{passageCount}</span>
      </div>
      <div className="message-box">
        <span className="stat-label">消息：</span>
        <p>{message}</p>
      </div>
      {/* 颜色图例 */}
      <div className="legend">
        <h4>图例</h4>
        <div className="legend-item"><span className="legend-color" style={{ background: '#1a1a2e' }} /> 墙</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#16213e' }} /> 通道</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#0f3460' }} /> 已访问</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#e94560' }} /> 当前位置</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#00d9ff' }} /> 路径</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#00ff88' }} /> 起点</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#ff6b6b' }} /> 终点</div>
      </div>
    </div>
  );
}
