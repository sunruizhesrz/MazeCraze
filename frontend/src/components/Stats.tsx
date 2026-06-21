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
  const flatGrid = grid.flat();
  const pathLength = flatGrid.filter((c) => c === Cell.Path).length;
  const visitedCount = flatGrid.filter((c) => c === Cell.Visited || c === Cell.Current).length;
  const passageCount = flatGrid.filter((c) => c === Cell.Passage).length;

  return (
    <div className="stats-panel">
      <h3>Statistics</h3>
      <div className="stat-row">
        <span className="stat-label">Mode:</span>
        <span className="stat-value mode-badge" data-mode={mode}>
          {mode === 'generate'
            ? 'Generating'
            : mode === 'solve'
              ? 'Solving'
              : mode === 'challenge'
                ? challenge?.finished
                  ? 'Finished'
                  : 'Challenge'
                : 'Idle'}
        </span>
      </div>
      {challenge && (
        <>
          <div className="stat-row">
            <span className="stat-label">Difficulty:</span>
            <span className="stat-value">{challenge.difficulty}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">Time:</span>
            <span className="stat-value">{challenge.elapsed}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">Steps:</span>
            <span className="stat-value">{challenge.steps}</span>
          </div>
          <div className="stat-row">
            <span className="stat-label">Best:</span>
            <span className="stat-value">{challenge.best}</span>
          </div>
        </>
      )}
      <div className="stat-row">
        <span className="stat-label">Algorithm:</span>
        <span className="stat-value">{algorithm || '-'}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">Frame:</span>
        <span className="stat-value">{step} / {totalSteps}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">Progress:</span>
        <span className="stat-value">
          {totalSteps > 0 ? Math.round((step / totalSteps) * 100) : 0}%
        </span>
      </div>
      <div className="stat-row">
        <span className="stat-label">Path Length:</span>
        <span className="stat-value">{pathLength}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">Visited:</span>
        <span className="stat-value">{visitedCount}</span>
      </div>
      <div className="stat-row">
        <span className="stat-label">Passages:</span>
        <span className="stat-value">{passageCount}</span>
      </div>
      <div className="message-box">
        <span className="stat-label">Message:</span>
        <p>{message}</p>
      </div>
      <div className="legend">
        <h4>Legend</h4>
        <div className="legend-item"><span className="legend-color" style={{ background: '#1a1a2e' }} /> Wall</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#16213e' }} /> Passage</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#0f3460' }} /> Visited</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#e94560' }} /> Current</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#00d9ff' }} /> Path</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#00ff88' }} /> Start</div>
        <div className="legend-item"><span className="legend-color" style={{ background: '#ff6b6b' }} /> End</div>
      </div>
    </div>
  );
}
