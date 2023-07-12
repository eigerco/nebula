import React from 'react'
import './App.css'
import { BrowserRouter, Route, Routes } from 'react-router-dom'
import { Readme } from './Readme'
import { Wizard } from './Wizard'

export function App () {
  return (
    <BrowserRouter>
      <Routes>
        <Route path={'/'} element={<Readme/>} />
        <Route path={'/wizard'} element={<Wizard/>} />
      </Routes>
    </BrowserRouter>
  )
}

export default App
