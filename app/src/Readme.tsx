import React from 'react'
import { Navbar } from './Navbar'

export function Readme({ currentPage }: any) {
  return (
    <div className='p-3'>
      <Navbar currentPage="#/readme" />
      <h1>Nebula</h1>
      <p>Nebula is a tool for easing development of Soroban smart contracts.</p>
    </div>
  )
}
