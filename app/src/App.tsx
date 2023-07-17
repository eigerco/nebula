import React from 'react'
import './App.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { Wizard } from './Wizard'
import { Readme } from './Readme'

export function App() {
  return (
    <BrowserRouter basename="/nebula">
      <Routes>
        <Route path={'/'} element={<Readme />} />
        <Route path={'/wizard'} element={<Wizard />} />
        <Route path="*" element={<PageNotFound />} />
      </Routes>
    </BrowserRouter>
  )
}

function PageNotFound() {
  return (
    <div>
      <h2>404 Page not found</h2>
    </div>
  )
}

export default App
