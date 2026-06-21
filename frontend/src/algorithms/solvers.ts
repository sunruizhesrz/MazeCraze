import { Grid } from '../core/grid';
import { Cell, type Point, type DirectionType, movePoint } from '../core/types';
import type { Frame, AnimationResult } from './generators';

class SolverRecorder {
  frames: Frame[] = [];
  private grid: Grid;

  constructor(grid: Grid) {
    this.grid = grid;
  }

  record(message: string): void {
    this.frames.push({
      grid: this.grid.cellsArray,
      message,
      step: this.frames.length,
    });
  }
}

function reconstructPath(
  grid: Grid,
  parent: Map<string, { point: Point; dir: DirectionType } | null>,
  end: Point,
  recorder: SolverRecorder
): void {
  const endKey = `${end.x},${end.y}`;
  let current = parent.get(endKey);
  while (current && current.dir !== undefined) {
    grid.set(current.point, Cell.Path);
    recorder.record(`Path at (${current.point.x}, ${current.point.y})`);
    const pk = `${current.point.x},${current.point.y}`;
    current = parent.get(pk);
  }
  grid.set(end, Cell.End);
  recorder.record('Solution complete!');
}

function heuristic(a: Point, b: Point): number {
  return Math.abs(a.x - b.x) + Math.abs(a.y - b.y);
}

// ==================== BFS ====================

export function solveBFS(grid: Grid, start: Point, end: Point): AnimationResult {
  const working = grid.clone();
  const recorder = new SolverRecorder(working);
  recorder.record('Starting BFS');

  const queue: Point[] = [start];
  const visited = new Set<string>();
  visited.add(`${start.x},${start.y}`);
  const parent = new Map<string, { point: Point; dir: DirectionType } | null>();
  parent.set(`${start.x},${start.y}`, null);

  while (queue.length > 0) {
    const current = queue.shift()!;

    if (current.x === end.x && current.y === end.y) {
      recorder.record('Exit found!');
      reconstructPath(working, parent, end, recorder);
      return {
        frames: recorder.frames,
        width: grid.width,
        height: grid.height,
        totalSteps: recorder.frames.length,
      };
    }

    working.set(current, Cell.Visited);
    recorder.record(`Visiting (${current.x}, ${current.y})`);

    for (const { point: next } of working.directNeighbors(current)) {
      const nk = `${next.x},${next.y}`;
      if (!visited.has(nk)) {
        visited.add(nk);
        parent.set(nk, { point: current, dir: 0 as DirectionType });
        working.set(next, Cell.Current);
        queue.push(next);
      }
    }
  }

  recorder.record('No solution found');
  return {
    frames: recorder.frames,
    width: grid.width,
    height: grid.height,
    totalSteps: recorder.frames.length,
  };
}

// ==================== DFS ====================

export function solveDFS(grid: Grid, start: Point, end: Point): AnimationResult {
  const working = grid.clone();
  const recorder = new SolverRecorder(working);
  recorder.record('Starting DFS');

  const stack: Point[] = [start];
  const visited = new Set<string>();
  visited.add(`${start.x},${start.y}`);
  const parent = new Map<string, { point: Point; dir: DirectionType } | null>();
  parent.set(`${start.x},${start.y}`, null);

  while (stack.length > 0) {
    const current = stack.pop()!;

    if (current.x === end.x && current.y === end.y) {
      recorder.record('Exit found!');
      reconstructPath(working, parent, end, recorder);
      return {
        frames: recorder.frames,
        width: grid.width,
        height: grid.height,
        totalSteps: recorder.frames.length,
      };
    }

    let foundNext = false;
    for (const { point: next } of working.directNeighbors(current)) {
      const nk = `${next.x},${next.y}`;
      if (!visited.has(nk)) {
        visited.add(nk);
        parent.set(nk, { point: current, dir: 0 as DirectionType });
        working.set(next, Cell.Current);
        stack.push(current);
        stack.push(next);
        recorder.record(`Exploring (${next.x}, ${next.y})`);
        foundNext = true;
        break;
      }
    }

    if (!foundNext) {
      working.set(current, Cell.Visited);
    }
  }

  recorder.record('No solution found');
  return {
    frames: recorder.frames,
    width: grid.width,
    height: grid.height,
    totalSteps: recorder.frames.length,
  };
}

// ==================== A* ====================

export function solveAStar(grid: Grid, start: Point, end: Point): AnimationResult {
  const working = grid.clone();
  const recorder = new SolverRecorder(working);
  recorder.record('Starting A* Search');

  const openSet = new Map<string, number>();
  const gScore = new Map<string, number>();
  const parent = new Map<string, { point: Point; dir: DirectionType } | null>();

  const sKey = `${start.x},${start.y}`;
  openSet.set(sKey, heuristic(start, end));
  gScore.set(sKey, 0);
  parent.set(sKey, null);

  while (openSet.size > 0) {
    let currentKey = '';
    let currentF = Infinity;
    let current: Point = { x: 0, y: 0 };
    for (const [k, f] of openSet) {
      if (f < currentF) {
        currentF = f;
        currentKey = k;
        const [cx, cy] = k.split(',').map(Number);
        current = { x: cx, y: cy };
      }
    }

    if (current.x === end.x && current.y === end.y) {
      recorder.record('Exit found!');
      reconstructPath(working, parent, end, recorder);
      return {
        frames: recorder.frames,
        width: grid.width,
        height: grid.height,
        totalSteps: recorder.frames.length,
      };
    }

    openSet.delete(currentKey);
    working.set(current, Cell.Visited);
    recorder.record(`Visiting (${current.x}, ${current.y})`);

    for (const { point: next } of working.directNeighbors(current)) {
      const nk = `${next.x},${next.y}`;
      const tentativeG = (gScore.get(currentKey) ?? Infinity) + 1;
      if (tentativeG < (gScore.get(nk) ?? Infinity)) {
        gScore.set(nk, tentativeG);
        const fScore = tentativeG + heuristic(next, end);
        openSet.set(nk, fScore);
        parent.set(nk, { point: current, dir: 0 as DirectionType });
        if (working.get(next) !== Cell.Visited) {
          working.set(next, Cell.Current);
        }
      }
    }
  }

  recorder.record('No solution found');
  return {
    frames: recorder.frames,
    width: grid.width,
    height: grid.height,
    totalSteps: recorder.frames.length,
  };
}

// ==================== Wall Follower ====================

export function solveWallFollower(grid: Grid, start: Point, end: Point): AnimationResult {
  const working = grid.clone();
  const recorder = new SolverRecorder(working);
  recorder.record('Starting Wall Follower (left-hand rule)');

  let current: Point = { ...start };
  let dir: DirectionType = 2; // South
  const visited = new Set<string>();
  const maxSteps = grid.width * grid.height * 4;
  let steps = 0;

  working.set(start, Cell.Start);

  while (steps < maxSteps) {
    const ck = `${current.x},${current.y}`;
    if (!visited.has(ck)) {
      visited.add(ck);
      recorder.record(`Following wall at (${current.x}, ${current.y})`);
    }

    if (current.x === end.x && current.y === end.y) {
      working.set(end, Cell.End);
      recorder.record('Exit found!');
      return {
        frames: recorder.frames,
        width: grid.width,
        height: grid.height,
        totalSteps: recorder.frames.length,
      };
    }

    const leftDir = ((dir + 3) % 4) as DirectionType;
    const leftPoint = movePoint(current, leftDir);
    const forwardPoint = movePoint(current, dir);
    const rightDir = ((dir + 1) % 4) as DirectionType;
    const rightPoint = movePoint(current, rightDir);
    const backDir = ((dir + 2) % 4) as DirectionType;
    const backPoint = movePoint(current, backDir);

    const leftCell = working.get(leftPoint);
    const forwardCell = working.get(forwardPoint);
    const rightCell = working.get(rightPoint);
    const backCell = working.get(backPoint);

    if (leftCell !== null && leftCell !== Cell.Wall) {
      dir = leftDir;
      current = leftPoint;
      working.set(current, Cell.Current);
    } else if (forwardCell !== null && forwardCell !== Cell.Wall) {
      current = forwardPoint;
      working.set(current, Cell.Current);
    } else if (rightCell !== null && rightCell !== Cell.Wall) {
      dir = rightDir;
      current = rightPoint;
      working.set(current, Cell.Current);
    } else if (backCell !== null && backCell !== Cell.Wall) {
      dir = backDir;
      current = backPoint;
      working.set(current, Cell.Current);
    } else {
      break;
    }

    steps++;
  }

  recorder.record('Wall follower completed');
  return {
    frames: recorder.frames,
    width: grid.width,
    height: grid.height,
    totalSteps: recorder.frames.length,
  };
}

export const SOLVERS: Record<string, (grid: Grid, start: Point, end: Point) => AnimationResult> = {
  bfs: solveBFS,
  dfs: solveDFS,
  astar: solveAStar,
  'wall-follower': solveWallFollower,
};

export const SOLVER_NAMES: Record<string, string> = {
  bfs: 'Breadth-First Search',
  dfs: 'Depth-First Search',
  astar: 'A* Search',
  'wall-follower': 'Wall Follower',
};
