// 单元格类型枚举
export const Cell = {
  Wall: 0,       // 墙
  Passage: 1,    // 通道
  Visited: 2,    // 已访问（求解过程标记）
  Current: 3,    // 当前位置（动画高亮）
  Path: 4,       // 最终求解路径
  Start: 5,      // 起点
  End: 6,        // 终点
} as const;

export type CellType = (typeof Cell)[keyof typeof Cell];

// 二维坐标点
export interface Point {
  x: number;
  y: number;
}

// 方向枚举（北、东、南、西）
export const Direction = {
  North: 0,
  East: 1,
  South: 2,
  West: 3,
} as const;

export type DirectionType = (typeof Direction)[keyof typeof Direction];

// 全部四个方向
export const ALL_DIRECTIONS: DirectionType[] = [
  Direction.North,
  Direction.East,
  Direction.South,
  Direction.West,
];

// 每个方向对应的位移向量
export const DIRECTION_VECTORS: Record<DirectionType, Point> = {
  [Direction.North]: { x: 0, y: -1 },
  [Direction.East]: { x: 1, y: 0 },
  [Direction.South]: { x: 0, y: 1 },
  [Direction.West]: { x: -1, y: 0 },
};

// 动画中的单帧
export interface Frame {
  grid: number[][];
  message: string;
  step: number;
}

// 动画生成 / 求解的结果
export interface AnimationResult {
  frames: Frame[];
  width: number;
  height: number;
  totalSteps: number;
}

// 两个点相加
export function addPoint(a: Point, b: Point): Point {
  return { x: a.x + b.x, y: a.y + b.y };
}

// 沿指定方向移动点
export function movePoint(p: Point, dir: DirectionType): Point {
  return addPoint(p, DIRECTION_VECTORS[dir]);
}

// 返回相反方向
export function opposite(dir: DirectionType): DirectionType {
  return ((dir + 2) % 4) as DirectionType;
}
