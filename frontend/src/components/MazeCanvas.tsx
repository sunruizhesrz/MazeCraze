import { useRef, useEffect, useCallback } from 'react';
import { Cell } from '../core/types';

interface Props {
  grid: number[][];
  cellSize?: number;
}

// 高区分度配色：每种单元格类型都有独特的色相与亮度
const CELL_COLORS: Record<number, string> = {
  [Cell.Wall]: '#000000',      // 纯黑 - 墙
  [Cell.Passage]: '#FFFFFF',   // 纯白 - 通道
  [Cell.Visited]: '#FF8C00',   // 深橙 - 已搜索
  [Cell.Current]: '#FF00FF',   // 品红 - 当前位置
  [Cell.Path]: '#00FF00',      // 亮绿 - 最终路径
  [Cell.Start]: '#00FFFF',     // 青色 - 起点
  [Cell.End]: '#FF0000',       // 纯红 - 终点
};

// 需要发光效果的单元格类型
const GLOW_CELLS: Set<number> = new Set([Cell.Path, Cell.Start, Cell.End, Cell.Current]);

export default function MazeCanvas({ grid, cellSize = 12 }: Props) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const draw = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas || !grid.length) return;

    const height = grid.length;
    const width = grid[0]?.length ?? 0;
    const gap = 1; // 单元格间留 1px 间隙以增强边界感
    const drawSize = Math.max(1, cellSize - gap);

    canvas.width = width * cellSize;
    canvas.height = height * cellSize;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // 清除背景（极深色）
    ctx.fillStyle = '#050508';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    for (let y = 0; y < height; y++) {
      for (let x = 0; x < width; x++) {
        const cell = grid[y][x];
        const color = CELL_COLORS[cell] ?? CELL_COLORS[Cell.Wall];
        const cx = x * cellSize;
        const cy = y * cellSize;

        if (GLOW_CELLS.has(cell)) {
          // 特殊单元格的发光效果
          ctx.save();
          ctx.shadowColor = color;
          ctx.shadowBlur = cellSize * 1.5;
          ctx.fillStyle = color;
          ctx.fillRect(cx, cy, drawSize, drawSize);
          ctx.restore();

          // 明亮的内边框
          ctx.strokeStyle = 'rgba(255,255,255,0.7)';
          ctx.lineWidth = 0.8;
          ctx.strokeRect(cx + 0.5, cy + 0.5, drawSize - 1, drawSize - 1);
        } else {
          ctx.fillStyle = color;
          ctx.fillRect(cx, cy, drawSize, drawSize);

          // 为非墙通道添加细微边框，强化边缘
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
