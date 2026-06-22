import { useState, useEffect, useRef, useCallback } from 'react';
import { Cell, Direction, movePoint, type DirectionType, type Point } from './core/types';
import { Grid } from './core/grid';
import { GENERATORS, GENERATOR_NAMES } from './algorithms/generators';
import { SOLVERS, SOLVER_NAMES } from './algorithms/solvers';
import type { Frame } from './algorithms/generators';
import MazeCanvas from './components/MazeCanvas';
import Controls from './components/Controls';
import Stats from './components/Stats';
import './App.css';

// 默认参数
const DEFAULT_WIDTH = 21;
const DEFAULT_HEIGHT = 21;
const DEFAULT_CELL_SIZE = 14;
// 本地最佳记录在 localStorage 中的键名
const RECORDS_KEY = 'mazecraze.challenge.records';

type AppMode = 'idle' | 'generate' | 'solve' | 'challenge';
type DifficultyKey = 'easy' | 'normal' | 'hard';

interface DifficultyConfig {
  label: string;
  width: number;
  height: number;
}

interface BestRecord {
  elapsedMs: number;
  steps: number;
}

const DIFFICULTIES: Record<DifficultyKey, DifficultyConfig> = {
  easy: { label: 'Easy', width: 15, height: 9 },
  normal: { label: 'Normal', width: 21, height: 13 },
  hard: { label: 'Hard', width: 31, height: 19 },
};

// 生成一个全部由墙填充的空网格
function emptyGrid(width: number, height: number): number[][] {
  return Array.from({ length: height }, () =>
    Array.from({ length: width }, () => Cell.Wall)
  );
}

// 将毫秒数格式化为 "秒.毫秒s" 形式
function formatDuration(ms: number): string {
  const seconds = Math.floor(ms / 1000);
  const millis = Math.floor(ms % 1000);
  return `${seconds}.${millis.toString().padStart(3, '0')}s`;
}

// 格式化最佳记录显示
function formatRecord(record: BestRecord | null): string {
  if (!record) return 'No record yet';
  return `${formatDuration(record.elapsedMs)}, ${record.steps} steps`;
}

// 从 localStorage 加载历史最佳记录
function loadRecords(): Record<string, BestRecord> {
  try {
    const raw = localStorage.getItem(RECORDS_KEY);
    return raw ? JSON.parse(raw) : {};
  } catch {
    return {};
  }
}

function App() {
  const [generator, setGenerator] = useState('backtracker');
  const [solver, setSolver] = useState('astar');
  const [difficulty, setDifficulty] = useState<DifficultyKey>('normal');
  const [width, setWidth] = useState(DEFAULT_WIDTH);
  const [height, setHeight] = useState(DEFAULT_HEIGHT);
  const [speed, setSpeed] = useState(5);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentFrame, setCurrentFrame] = useState(0);
  const [frames, setFrames] = useState<Frame[]>([]);
  const [mode, setMode] = useState<AppMode>('idle');
  const [currentGrid, setCurrentGrid] = useState<number[][]>(
    emptyGrid(DEFAULT_WIDTH, DEFAULT_HEIGHT)
  );
  const [message, setMessage] = useState('欢迎使用 MazeCraze！点击"生成迷宫"开始。');
  const [algorithmName, setAlgorithmName] = useState('');
  const [player, setPlayer] = useState<Point>({ x: 1, y: 1 });
  const [challengeSteps, setChallengeSteps] = useState(0);
  const [challengeStart, setChallengeStart] = useState<number | null>(null);
  const [challengeElapsed, setChallengeElapsed] = useState(0);
  const [challengeFinished, setChallengeFinished] = useState(false);
  const [records, setRecords] = useState<Record<string, BestRecord>>(() => loadRecords());

  // 动画与计时器引用
  const animRef = useRef<number | null>(null);
  const animateRef = useRef<(timestamp: number) => void>(() => undefined);
  const lastTimeRef = useRef<number>(0);
  const timerRef = useRef<number | null>(null);

  const recordKey = `${difficulty}|${width}x${height}|${generator}`;
  const bestRecord = records[recordKey] ?? null;

  // 停止当前动画
  const stopAnimation = useCallback(() => {
    setIsPlaying(false);
    if (animRef.current !== null) {
      cancelAnimationFrame(animRef.current);
      animRef.current = null;
    }
  }, []);

  // 逐帧推进的动画回调
  const animate = useCallback(
    (timestamp: number) => {
      if (!isPlaying) return;

      const interval = 1000 / (speed * 10);
      if (timestamp - lastTimeRef.current >= interval) {
        setCurrentFrame((prev) => {
          if (prev >= frames.length - 1) {
            setIsPlaying(false);
            return prev;
          }
          const next = prev + 1;
          const frame = frames[next];
          if (frame) {
            setCurrentGrid(frame.grid);
            setMessage(frame.message);
          }
          return next;
        });
        lastTimeRef.current = timestamp;
      }

      animRef.current = requestAnimationFrame(animateRef.current);
    },
    [isPlaying, frames, speed]
  );

  useEffect(() => {
    animateRef.current = animate;
    if (isPlaying) {
      lastTimeRef.current = 0;
      animRef.current = requestAnimationFrame(animateRef.current);
    } else {
      if (animRef.current !== null) {
        cancelAnimationFrame(animRef.current);
      }
    }
    return () => {
      if (animRef.current !== null) {
        cancelAnimationFrame(animRef.current);
      }
    };
  }, [isPlaying, animate]);

  // 挑战模式计时器
  useEffect(() => {
    if (mode !== 'challenge' || challengeFinished || challengeStart === null) {
      if (timerRef.current !== null) {
        window.clearInterval(timerRef.current);
        timerRef.current = null;
      }
      return;
    }

    timerRef.current = window.setInterval(() => {
      setChallengeElapsed(performance.now() - challengeStart);
    }, 100);

    return () => {
      if (timerRef.current !== null) {
        window.clearInterval(timerRef.current);
        timerRef.current = null;
      }
    };
  }, [mode, challengeFinished, challengeStart]);

  // 切换难度
  const handleDifficultyChange = useCallback((next: string) => {
    const key = next as DifficultyKey;
    const config = DIFFICULTIES[key];
    if (!config) return;
    stopAnimation();
    setDifficulty(key);
    setWidth(config.width);
    setHeight(config.height);
    setMessage(`${config.label} difficulty selected.`);
  }, [stopAnimation]);

  // 标记挑战网格上的起点、终点和玩家位置
  const markChallengeGrid = useCallback((grid: Grid, nextPlayer: Point): number[][] => {
    grid.clearMarkers();
    const start = { x: 1, y: 1 };
    const end = { x: grid.width - 2, y: grid.height - 2 };
    grid.set(start, Cell.Start);
    grid.set(end, Cell.End);
    grid.set(nextPlayer, Cell.Current);
    return grid.cellsArray;
  }, []);

  // 生成迷宫
  const handleGenerate = useCallback(() => {
    stopAnimation();
    const w = width % 2 === 0 ? width + 1 : width;
    const h = height % 2 === 0 ? height + 1 : height;

    setMode('generate');
    setAlgorithmName(GENERATOR_NAMES[generator] ?? generator);
    setMessage(`Generating maze with ${GENERATOR_NAMES[generator]}...`);

    const genFn = GENERATORS[generator];
    if (!genFn) return;

    const result = genFn(w, h);
    setFrames(result.frames);
    setCurrentFrame(0);

    if (result.frames.length > 0) {
      setCurrentGrid(result.frames[0].grid);
      setMessage(result.frames[0].message);
    }

    setMode('idle');
  }, [generator, width, height, stopAnimation]);

  // 求解迷宫
  const handleSolve = useCallback(() => {
    if (frames.length === 0) return;

    stopAnimation();
    const lastFrame = frames[frames.length - 1];
    const grid = Grid.fromArray(lastFrame.grid);
    grid.clearMarkers();

    const start = { x: 1, y: 1 };
    const end = { x: grid.width - 2, y: grid.height - 2 };

    setMode('solve');
    setAlgorithmName(SOLVER_NAMES[solver] ?? solver);
    setMessage(`Solving with ${SOLVER_NAMES[solver]}...`);

    const solveFn = SOLVERS[solver];
    if (!solveFn) return;

    const result = solveFn(grid, start, end);
    setFrames(result.frames);
    setCurrentFrame(0);

    if (result.frames.length > 0) {
      setCurrentGrid(result.frames[0].grid);
      setMessage(result.frames[0].message);
    }

    setMode('idle');
  }, [frames, solver, stopAnimation]);

  // 播放 / 暂停切换
  const handlePlayPause = useCallback(() => {
    if (frames.length === 0) return;
    setIsPlaying((prev) => !prev);
  }, [frames]);

  // 单步前进
  const handleStepForward = useCallback(() => {
    stopAnimation();
    setCurrentFrame((prev) => {
      const next = Math.min(prev + 1, frames.length - 1);
      const frame = frames[next];
      if (frame) {
        setCurrentGrid(frame.grid);
        setMessage(frame.message);
      }
      return next;
    });
  }, [frames, stopAnimation]);

  // 单步后退
  const handleStepBackward = useCallback(() => {
    stopAnimation();
    setCurrentFrame((prev) => {
      const next = Math.max(prev - 1, 0);
      const frame = frames[next];
      if (frame) {
        setCurrentGrid(frame.grid);
        setMessage(frame.message);
      }
      return next;
    });
  }, [frames, stopAnimation]);

  // 重置应用状态
  const handleReset = useCallback(() => {
    stopAnimation();
    setFrames([]);
    setCurrentFrame(0);
    setCurrentGrid(emptyGrid(DEFAULT_WIDTH, DEFAULT_HEIGHT));
    setMessage('欢迎使用 MazeCraze！点击"生成迷宫"开始。');
    setAlgorithmName('');
    setMode('idle');
    setChallengeStart(null);
    setChallengeElapsed(0);
    setChallengeSteps(0);
    setChallengeFinished(false);
    setPlayer({ x: 1, y: 1 });
  }, [stopAnimation]);

  // 开始挑战模式
  const handleStartChallenge = useCallback(() => {
    stopAnimation();
    const config = DIFFICULTIES[difficulty];
    const genFn = GENERATORS[generator];
    if (!genFn) return;

    const result = genFn(config.width, config.height);
    const lastFrame = result.frames[result.frames.length - 1];
    if (!lastFrame) return;

    const grid = Grid.fromArray(lastFrame.grid);
    const start = { x: 1, y: 1 };
    setWidth(config.width);
    setHeight(config.height);
    setFrames(result.frames);
    setCurrentFrame(result.frames.length - 1);
    setCurrentGrid(markChallengeGrid(grid, start));
    setPlayer(start);
    setChallengeSteps(0);
    setChallengeElapsed(0);
    setChallengeStart(performance.now());
    setChallengeFinished(false);
    setMode('challenge');
    setAlgorithmName(GENERATOR_NAMES[generator] ?? generator);
    setMessage(`Challenge started. Reach the red exit.`);
  }, [difficulty, generator, markChallengeGrid, stopAnimation]);

  // 完成挑战并尝试更新最佳记录
  const finishChallenge = useCallback((elapsedMs: number, steps: number) => {
    setChallengeFinished(true);
    setChallengeStart(null);
    setChallengeElapsed(elapsedMs);
    setRecords((prev) => {
      const existing = prev[recordKey];
      const nextRecord = { elapsedMs: Math.round(elapsedMs), steps };
      if (
        existing &&
        (existing.elapsedMs < nextRecord.elapsedMs ||
          (existing.elapsedMs === nextRecord.elapsedMs && existing.steps <= nextRecord.steps))
      ) {
        return prev;
      }
      const updated = { ...prev, [recordKey]: nextRecord };
      localStorage.setItem(RECORDS_KEY, JSON.stringify(updated));
      return updated;
    });
    setMessage('Finished! Best record updated if this run was faster.');
  }, [recordKey]);

  // 在挑战模式下移动玩家
  const movePlayer = useCallback((direction: DirectionType) => {
    if (mode !== 'challenge' || challengeFinished) return;
    const grid = Grid.fromArray(currentGrid);
    grid.clearMarkers();

    const next = movePoint(player, direction);
    const cell = grid.get(next);
    if (cell === null || cell === Cell.Wall) {
      setMessage('Blocked by a wall.');
      setCurrentGrid(markChallengeGrid(grid, player));
      return;
    }

    const nextSteps = challengeSteps + 1;
    const end = { x: grid.width - 2, y: grid.height - 2 };
    setPlayer(next);
    setChallengeSteps(nextSteps);

    // 玩家到达终点
    if (next.x === end.x && next.y === end.y) {
      const elapsed = challengeStart === null ? challengeElapsed : performance.now() - challengeStart;
      setCurrentGrid(markChallengeGrid(grid, next));
      finishChallenge(elapsed, nextSteps);
      return;
    }

    setMessage(`Steps: ${nextSteps}`);
    setCurrentGrid(markChallengeGrid(grid, next));
  }, [
    challengeElapsed,
    challengeFinished,
    challengeStart,
    challengeSteps,
    currentGrid,
    finishChallenge,
    markChallengeGrid,
    mode,
    player,
  ]);

  // 监听全局键盘事件，仅在挑战模式下生效
  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const keyMap: Record<string, DirectionType> = {
        ArrowUp: Direction.North,
        w: Direction.North,
        W: Direction.North,
        ArrowRight: Direction.East,
        d: Direction.East,
        D: Direction.East,
        ArrowDown: Direction.South,
        s: Direction.South,
        S: Direction.South,
        ArrowLeft: Direction.West,
        a: Direction.West,
        A: Direction.West,
      };
      const direction = keyMap[event.key];
      if (direction === undefined) return;
      if (mode === 'challenge') event.preventDefault();
      movePlayer(direction);
    };
    window.addEventListener('keydown', onKeyDown);
    return () => window.removeEventListener('keydown', onKeyDown);
  }, [mode, movePlayer]);

  // 根据迷宫尺寸自适应单元格大小
  const cellSize = Math.max(4, Math.min(DEFAULT_CELL_SIZE, Math.floor(600 / Math.max(width, height))));

  return (
    <div className="app">
      <header className="app-header">
        <h1>MazeCraze</h1>
        <p className="subtitle">Interactive Maze Generator & Solver</p>
      </header>

      <main className="app-main">
        <div className="left-panel">
          <Controls
            generator={generator}
            solver={solver}
            difficulty={difficulty}
            width={width}
            height={height}
            speed={speed}
            isPlaying={isPlaying}
            hasFrames={frames.length > 0}
            isChallengeActive={mode === 'challenge' && !challengeFinished}
            onGeneratorChange={setGenerator}
            onSolverChange={setSolver}
            onDifficultyChange={handleDifficultyChange}
            onWidthChange={setWidth}
            onHeightChange={setHeight}
            onStartChallenge={handleStartChallenge}
            onGenerate={handleGenerate}
            onSolve={handleSolve}
            onPlayPause={handlePlayPause}
            onStepForward={handleStepForward}
            onStepBackward={handleStepBackward}
            onSpeedChange={setSpeed}
            onReset={handleReset}
          />
        </div>

        <div className="center-panel">
          <MazeCanvas grid={currentGrid} cellSize={cellSize} />
          <div className="progress-bar">
            <div
              className="progress-fill"
              style={{
                width: `${frames.length > 0 ? (currentFrame / (frames.length - 1)) * 100 : 0}%`,
              }}
            />
          </div>
        </div>

        <div className="right-panel">
          <Stats
            step={currentFrame}
            totalSteps={frames.length}
            message={message}
            grid={currentGrid}
            algorithm={algorithmName}
            mode={mode}
            challenge={
              mode === 'challenge'
                ? {
                    difficulty: DIFFICULTIES[difficulty].label,
                    elapsed: formatDuration(challengeElapsed),
                    steps: challengeSteps,
                    best: formatRecord(bestRecord),
                    finished: challengeFinished,
                  }
                : undefined
            }
          />
        </div>
      </main>

      <footer className="app-footer">
        <p>
          Rust 核心 + React 前端 | 3 种生成算法 | 4 种求解算法 | 使用 Vite 构建
        </p>
      </footer>
    </div>
  );
}

export default App;
