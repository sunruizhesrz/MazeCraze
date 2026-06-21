import { useRef, useEffect, useCallback } from 'react';
import { Cell } from '../core/types';

interface Props {
  grid: number[][];
  cellSize?: number;
}

// High-diversity color scheme: each cell type has distinct hue and brightness
const CELL_COLORS: Record<number, string> = {
  [Cell.Wall]: '#000000',      // pure black - invisible
  [Cell.Passage]: '#FFFFFF',   // pure white - clear open space
  [Cell.Visited]: '#FF8C00',   // dark orange - searched area
  [Cell.Current]: '#FF00FF',   // magenta - active cursor
  [Cell.Path]: '#00FF00',      // bright green - final solution
  [Cell.Start]: '#00FFFF',     // cyan - entrance
  [Cell.End]: '#FF0000',       // pure red - exit
};

const GLOW_CELLS: Set<number> = new Set([Cell.Path, Cell.Start, Cell.End, Cell.Current]);

export default function MazeCanvas({ grid, cellSize = 12 }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas || !grid.length) return;

    const height = grid.length;
    const width = grid[0]?.length ?? 0;
    const gap = 1; // 1px gap between cells for definition
    const drawSize = Math.max(1, cellSize - gap);

    canvas.width = width * cellSize;
    canvas.height = height * cellSize;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear background (very dark)
    ctx.fillStyle = '#050508';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const cell = grid[y][x];
        const color = CELL_COLORS[cell] ?? CELL_COLORS[Cell.Wall];
        const cx = x * cellSize;
        const cy = y * cellSize;

        if (GLOW_CELLS.has(cell)) {
          // Glow effect for special cells
          ctx.save();
          ctx.shadowColor = color;
          ctx.shadowBlur = cellSize * 1.5;
          ctx.fillStyle = color;
          ctx.fillRect(cx, cy, drawSize, drawSize);
          ctx.restore();

          // Bright inner border
          ctx.strokeStyle = 'rgba(255,255,255,0.7)';
          ctx.lineWidth = 0.8;
          ctx.strokeRect(cx + 0.5, cy + 0.5, drawSize - 1, drawSize - 1);
        } else {
          ctx.fillStyle = color;
          ctx.fillRect(cx, cy, drawSize, drawSize);

          // Subtle border for non-wall passages to define edges
          if (cell !== Cell.Wall) {
            ctx.strokeStyle = 'rgba(255,255,255,0.08)';
            ctx.lineWidth = 0.5;
            ctx.strokeRect(cx + 0.5, cy + 0.5, drawSize - 1, drawSize - 1);
          }
        }
      }
    }
  }, [grid, cellSize]);

  useEffect(() => {
    draw();
  }, [draw]);

  return (
    <canvas
      ref={canvasRef}
      style={{
        border: '2px solid #333',
        borderRadius: '4px',
        maxWidth: '100%',
        maxHeight: '70vh',
      }}
    />
  );
}
