import { Grid } from '../core/grid';
import { Cell, type Point, ALL_DIRECTIONS, movePoint } from '../core/types';

// 动画中的一帧
export interface Frame {
  grid: number[][];
  message: string;
  step: number;
}

// 动画生成结果
export interface AnimationResult {
  frames: Frame[];
  width: number;
  height: number;
  totalSteps: number;
}

// 动画记录器，用于在生成过程中捕获每一帧
class Recorder {
  frames: Frame[] = [];
  private grid: Grid;

  constructor(grid: Grid) {
    this.grid = grid;
  }

  // 记录当前网格状态为一帧
  record(message: string): void {
    this.frames.push({
      grid: this.grid.cellsArray,
      message,
      step: this.frames.length,
    });
  }

  getGrid(): Grid {
    return this.grid;
  }
}

// Fisher-Yates 洗牌
function shuffle<T>(array: T[]): T[] {
  const arr = [...array];
  for (let i = arr.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [arr[i], arr[j]] = [arr[j], arr[i]];
  }
  return arr;
}

// 初始化全墙网格，并在 (1,1) 处开凿起始通道
function initGrid(width: number, height: number): { grid: Grid; start: Point } {
  const grid = new Grid(width, height);
  const start: Point = { x: 1, y: 1 };
  grid.set(start, Cell.Passage);
  return { grid, start };
}

// ==================== 递归回溯 ====================

export function generateBacktracker(width: number, height: number): AnimationResult {
  const { grid, start } = initGrid(width, height);
  const recorder = new Recorder(grid);
  recorder.record('Starting maze generation');

  function carve(current: Point) {
    recorder.record(`Visiting (${current.x}, ${current.y})`);
    const neighbors = shuffle(grid.passageNeighbors(current));
    for (const { point: next, dir } of neighbors) {
      if (grid.get(next) === Cell.Wall) {
        grid.carvePassage(current, dir);
        recorder.record(`Carved passage to (${next.x}, ${next.y})`);
        carve(next);
      }
    }
  }

  carve(start);
  recorder.record('Maze generation complete!');

  return {
    frames: recorder.frames,
    width,
    height,
    totalSteps: recorder.frames.length,
  };
}

// ==================== 随机化 Prim ====================

export function generatePrim(width: number, height: number): AnimationResult {
  const { grid, start } = initGrid(width, height);
  const recorder = new Recorder(grid);
  recorder.record('Starting Prim algorithm');

  // 前沿墙列表：{ wall: 中间墙, to: 墙后的目标单元格 }
  const walls: { wall: Point; to: Point }[] = [];

  function addWalls(p: Point) {
    for (const dir of ALL_DIRECTIONS) {
      const wall = movePoint(p, dir);
      const beyond = movePoint(wall, dir);
      if (grid.get(wall) === Cell.Wall && grid.get(beyond) === Cell.Wall) {
        walls.push({ wall, to: beyond });
      }
    }
  }

  addWalls(start);

  while (walls.length > 0) {
    const idx = Math.floor(Math.random() * walls.length);
    const { wall, to } = walls[idx];
    walls.splice(idx, 1);

    if (grid.get(to) === Cell.Wall) {
      grid.set(wall, Cell.Passage);
      grid.set(to, Cell.Passage);
      recorder.record(`Added passage to (${to.x}, ${to.y})`);
      addWalls(to);
    }
  }

  recorder.record('Maze generation complete!');

  return {
    frames: recorder.frames,
    width,
    height,
    totalSteps: recorder.frames.length,
  };
}

// ==================== 随机化 Kruskal ====================

export function generateKruskal(width: number, height: number): AnimationResult {
  const grid = new Grid(width, height);
  const recorder = new Recorder(grid);
  recorder.record('Starting Kruskal algorithm');

  const passages: Point[] = [];
  // 候选墙列表：{ wall: 中间墙, from: 单元格 a, to: 单元格 b }
  const walls: { wall: Point; from: Point; to: Point }[] = [];

  // 收集所有奇数索引位置的通道单元格，以及它们之间的墙
  for (let y = 1; y < height; y += 2) {
    for (let x = 1; x < width; x += 2) {
      const p: Point = { x, y };
      grid.set(p, Cell.Passage);
      passages.push(p);

      if (x + 2 < width) {
        walls.push({ wall: { x: x + 1, y }, from: p, to: { x: x + 2, y } });
      }
      if (y + 2 < height) {
        walls.push({ wall: { x, y: y + 1 }, from: p, to: { x, y: y + 2 } });
      }
    }
  }

  const shuffledWalls = shuffle(walls);

  // 并查集
  const parent = new Map<string, string>();
  function key(p: Point): string {
    return `${p.x},${p.y}`;
  }
  // 带路径压缩的查找
  function find(p: Point): string {
    const k = key(p);
    if (!parent.has(k)) parent.set(k, k);
    let root = parent.get(k)!;
    while (root !== parent.get(root)) {
      root = parent.get(root)!;
    }
    let curr = k;
    while (curr !== root) {
      const next = parent.get(curr)!;
      parent.set(curr, root);
      curr = next;
    }
    return root;
  }
  // 合并两个集合
  function union(a: Point, b: Point): void {
    const ra = find(a);
    const rb = find(b);
    if (ra !== rb) parent.set(ra, rb);
  }

  // 初始化每个通道单元格自成一集合
  for (const p of passages) {
    parent.set(key(p), key(p));
  }

  // 随机遍历墙，若两侧不在同一集合则打通
  for (const { wall, from, to } of shuffledWalls) {
    if (find(from) !== find(to)) {
      grid.set(wall, Cell.Passage);
      union(from, to);
      recorder.record(`Removed wall at (${wall.x}, ${wall.y})`);
    }
  }

  recorder.record('Maze generation complete!');

  return {
    frames: recorder.frames,
    width,
    height,
    totalSteps: recorder.frames.length,
  };
}

// 生成器注册表
export const GENERATORS: Record<string, (w: number, h: number) => AnimationResult> = {
  backtracker: generateBacktracker,
  prim: generatePrim,
  kruskal: generateKruskal,
};

// 生成器的人类可读名称
export const GENERATOR_NAMES: Record<string, string> = {
  backtracker: 'Recursive Backtracker',
  prim: 'Randomized Prim',
  kruskal: 'Randomized Kruskal',
};
