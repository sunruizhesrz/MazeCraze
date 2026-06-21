export const Cell = {
  Wall: 0,
  Passage: 1,
  Visited: 2,
  Current: 3,
  Path: 4,
  Start: 5,
  End: 6,
} as const;

export type CellType = (typeof Cell)[keyof typeof Cell];

export interface Point {
  x: number;
  y: number;
}

export const Direction = {
  North: 0,
  East: 1,
  South: 2,
  West: 3,
} as const;

export type DirectionType = (typeof Direction)[keyof typeof Direction];

export const ALL_DIRECTIONS: DirectionType[] = [
  Direction.North,
  Direction.East,
  Direction.South,
  Direction.West,
];

export const DIRECTION_VECTORS: Record<DirectionType, Point> = {
  [Direction.North]: { x: 0, y: -1 },
  [Direction.East]: { x: 1, y: 0 },
  [Direction.South]: { x: 0, y: 1 },
  [Direction.West]: { x: -1, y: 0 },
};

export interface Frame {
  grid: number[][];
  message: string;
  step: number;
}

export interface AnimationResult {
  frames: Frame[];
  width: number;
  height: number;
  totalSteps: number;
}

export function addPoint(a: Point, b: Point): Point {
  return { x: a.x + b.x, y: a.y + b.y };
}

export function movePoint(p: Point, dir: DirectionType): Point {
  return addPoint(p, DIRECTION_VECTORS[dir]);
}

export function opposite(dir: DirectionType): DirectionType {
  return ((dir + 2) % 4) as DirectionType;
}
