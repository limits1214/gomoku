import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import DefaultLayout from './layout/DefaultLayout.tsx'
import TestChat from './page/test/TestChat.tsx'
import Test from './page/test/Test.tsx'
import Home from './page/Home.tsx'
import Room from './page/Room.tsx'
import NotFound from './page/NotFound.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <BrowserRouter basename='/gomoku'>
      <Routes>
        <Route element={<DefaultLayout/>}>
          <Route index element={<Home/>} />
          <Route path='room/:roomSn' element={<Room/>} />
        </Route>
        <Route path='test'>
          <Route index element={<Test/>} />
          <Route path='chat' element={<TestChat/>} />
        </Route>
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  </StrictMode>,
)
