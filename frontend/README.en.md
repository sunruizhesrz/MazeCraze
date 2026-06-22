# MazeCraze Web Frontend

[中文](README.md) | [English](README.en.md)

A browser-based maze visualization interface built with React 18, TypeScript, and Vite. It complements the Rust side with a second interactive visualization surface.

## Features

- **Three-column layout**: Left controls, center maze canvas, right statistics panel
- **3 generation algorithms**: Recursive Backtracker, Randomized Prim, Randomized Kruskal
- **4 solving algorithms**: BFS, DFS, A\*, Wall Follower
- **Challenge mode**: Move through the maze with arrow keys / WASD, with records saved in `localStorage`
- **Animation controls**: Play / pause, step forward / backward, and speed adjustment
- **Canvas rendering**: High-contrast colors with highlighted special cells
- **Responsive sizing**: Cell pixels adapt to the selected maze size

## Quick Start

```bash
# Enter the frontend directory
cd frontend

# Install dependencies
npm install

# Start the development server
npm run dev

# Build for production
npm run build

# Preview the production build
npm run preview
```

## Directory Structure

```
frontend/
├── index.html              # HTML entry
├── package.json            # Dependencies and scripts
├── vite.config.ts          # Vite configuration
├── eslint.config.js        # ESLint configuration
├── tsconfig.json           # TypeScript configuration
└── src/
    ├── main.tsx            # Application mount entry
    ├── App.tsx             # Root component, global state and events
    ├── App.css             # Global styles
    ├── index.css           # Base styles
    ├── core/
    │   ├── types.ts        # Base types such as Cell, Direction, Point
    │   └── grid.ts         # Grid class
    ├── algorithms/
    │   ├── generators.ts   # Three maze generation algorithms
    │   └── solvers.ts      # Four maze solving algorithms
    └── components/
        ├── Controls.tsx    # Control panel component
        ├── MazeCanvas.tsx  # Canvas maze rendering component
        └── Stats.tsx       # Statistics component
```

## Keyboard Controls

| Key | Action |
|-----|--------|
| `↑` / `W` | Move up in challenge mode |
| `↓` / `S` | Move down in challenge mode |
| `←` / `A` | Move left in challenge mode |
| `→` / `D` | Move right in challenge mode |

## Color Legend

| Color | Meaning |
|-----|------|
| Black | Wall |
| White | Passage |
| Orange | Visited |
| Magenta | Current position |
| Bright green | Final path |
| Cyan | Start |
| Red | End |

## ESLint Extension

This project is based on the Vite template and includes the baseline ESLint rules. To enable type-aware rules, use a configuration like this:

```js
export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      // Other configs...

      // Replace tseslint.configs.recommended with this:
      tseslint.configs.recommendedTypeChecked,
      // Stricter rules
      tseslint.configs.strictTypeChecked,
      // Style rules
      tseslint.configs.stylisticTypeChecked,
    ],
    languageOptions: {
      parserOptions: {
        project: ['./tsconfig.node.json', './tsconfig.app.json'],
        tsconfigRootDir: import.meta.dirname,
      },
    },
  },
])
```

You can also install [eslint-plugin-react-x](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-x)
and [eslint-plugin-react-dom](https://github.com/Rel1cx/eslint-react/tree/main/packages/plugins/eslint-plugin-react-dom)
for React-specific lint rules.

## License

MIT
