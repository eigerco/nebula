import React from 'react'
import { Navbar } from './Navbar'

export function Readme({ currentPage }: any) {
  return (
    <div className="p-3">
      <Navbar currentPage="#/" />
      <h2>What is it?</h2>
      <p>Nebula is a tool for easing development of Soroban smart contracts.</p>
      <h2>Development</h2>
      <p>
        Nebula aims to provide the following functionality:
        <ul>
          <li>Compile contracts</li>
          <li>Download compiled contracts</li>
          <li>Deploy contracts</li>
          <li>An IDE playground</li>
          <li>
            A contract management system via <code>#import</code>
          </li>
        </ul>
      </p>
      <h2>Standardization</h2>
      <p>
        Nebula aims to provide standard interfaces for commonly used contacts.
        <ul>
          <li>Math</li>
          <li>Voting</li>
          <li>Lottery</li>
          <li>Split Payments</li>
          <li>Auction</li>
          <li>MarketPlace</li>
        </ul>
      </p>
    </div>
  )
}
