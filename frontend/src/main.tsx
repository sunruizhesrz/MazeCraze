import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'

// 将根组件挂载到 #root 节点上
createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
