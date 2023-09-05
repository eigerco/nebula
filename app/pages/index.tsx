import React from 'react'
import Readme from '../src/Readme.md'

export default function Page({ children }) {
  return (
    <div className="container">
      <Readme />
    </div>
  )
}
