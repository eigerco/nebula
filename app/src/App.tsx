import React from 'react'
import './App.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { Wizard } from './Wizard'
import { Readme } from './Readme'

export function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path={'/'} element={<Wizard />} />
        <Route path={'/readme'} element={<Readme />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
