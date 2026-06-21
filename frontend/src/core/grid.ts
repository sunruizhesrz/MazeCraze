import { Cell, type Point, type DirectionType, ALL_DIRECTIONS, movePoint } from './types';

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

  static fromArray(cells: number[][]): Grid {
    const g = new Grid(cells[0]?.length ?? 0, cells.length);
    g.cells = cells.map(row => [...row]);
    return g;
  }

  clone(): Grid {
    return Grid.fromArray(this.cells);
  }

  get(p: Point): number | null {
    if (p.x < 0 || p.x >= this.width || p.y < 0 || p.y >= this.height) return null;
    return this.cells[p.y][p.x];
  }

  set(p: Point, cell: number): void {
    if (p.x < 0 || p.x >= this.width || p.y < 0 || p.y >= this.height) return;
    this.cells[p.y][p.x] = cell;
  }

  get cellsArray(): number[][] {
    return this.cells.map(row => [...row]);
  }

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

  carvePassage(from: Point, dir: DirectionType): void {
    const wall = movePoint(from, dir);
    const to = movePoint(wall, dir);
    this.set(wall, Cell.Passage);
    this.set(to, Cell.Passage);
  }

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
