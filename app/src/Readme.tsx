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
          {/* Todo for @eloylp */}
          <li>
            A contract management system via{' '}
            <a href="https://github.com/eigerco/nebula/discussions/11">
              <code>#import</code>
            </a>
          </li>
        </ul>
      </p>
      <h2>Common interfaces </h2>
      <p>
        Nebula aims to provide interfaces for commonly used contacts.
        <ul>
          <li>Math</li>
          <li>Voting</li>
          <li>Lottery</li>
          <li>Split Payments</li>
          <li>Auction</li>
          <li>MarketPlace</li>
          <li>Pause</li>
          <li>Payment splitter</li>
          <li>reverse billing splitting</li>
        </ul>
      </p>
    </div>
  )
}
