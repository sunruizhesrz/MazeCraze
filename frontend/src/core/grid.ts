import { Cell, type Point, type DirectionType, ALL_DIRECTIONS, movePoint } from './types';

// 迷宫网格，使用二维数组存储单元格状态
export class Grid {
  private cells: number[][];
  readonly width: number;
  readonly height: number;

  constructor(width: number, height: number, fill: number = Cell.Wall) {
    this.width = width;
    this.height = height;
    this.cells = Array.from({ length: height }, () =>
      Array.from({ length: width }, () => fill)
    );
  }

  // 从二维数组构造网格（会进行拷贝）
  static fromArray(cells: number[][]): Grid {
    const g = new Grid(cells[0]?.length ?? 0, cells.length);
    g.cells = cells.map(row => [...row]);
    return g;
  }

  // 克隆当前网格
  clone(): Grid {
    return Grid.fromArray(this.cells);
  }

  // 获取指定点处的单元格；越界返回 null
  get(p: Point): number | null {
    if (p.x < 0 || p.x >= this.width || p.y < 0 || p.y >= this.height) return null;
    return this.cells[p.y][p.x];
  }

  // 设置指定点处的单元格；越界则忽略
  set(p: Point, cell: number): void {
    if (p.x < 0 || p.x >= this.width || p.y < 0 || p.y >= this.height) return;
    this.cells[p.y][p.x] = cell;
  }

  // 返回单元格数组的拷贝
  get cellsArray(): number[][] {
    return this.cells.map(row => [...row]);
  }

  // 获取距离为 2 的"通道邻居"（中间有墙的相邻通道候选）
  passageNeighbors(p: Point): { point: Point; dir: DirectionType }[] {
    const result: { point: Point; dir: DirectionType }[] = [];
    for (const dir of ALL_DIRECTIONS) {
      const neighbor = movePoint(p, dir);
      if (this.get(neighbor) === Cell.Wall) {
        const beyond = movePoint(neighbor, dir);
        if (this.get(beyond) === Cell.Wall) {
          result.push({ point: beyond, dir });
        }
      }
    }
    return result;
  }

  // 获取直接相邻（距离 1）的通道邻居
  directNeighbors(p: Point): { point: Point; dir: DirectionType }[] {
    const result: { point: Point; dir: DirectionType }[] = [];
    for (const dir of ALL_DIRECTIONS) {
      const neighbor = movePoint(p, dir);
      if (this.get(neighbor) !== null && this.get(neighbor) !== Cell.Wall) {
        result.push({ point: neighbor, dir });
      }
    }
    return result;
  }

  // 从 from 沿 dir 方向开凿通道，打通中间墙和目标单元格
  carvePassage(from: Point, dir: DirectionType): void {
    const wall = movePoint(from, dir);
    const to = movePoint(wall, dir);
    this.set(wall, Cell.Passage);
    this.set(to, Cell.Passage);
  }

  // 返回所有通道单元格坐标
  passages(): Point[] {
    const result: Point[] = [];
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        if (this.cells[y][x] === Cell.Passage) {
          result.push({ x, y });
        }
      }
    }
    return result;
  }

  // 清除可视化标记（Visited、Current、Path），将它们还原为 Passage
  clearMarkers(): void {
    for (let y = 0; y < this.height; y++) {
      for (let x = 0; x < this.width; x++) {
        const c = this.cells[y][x];
        if (c === Cell.Visited || c === Cell.Current || c === Cell.Path) {
          this.cells[y][x] = Cell.Passage;
        }
      }
    }
  }
}
