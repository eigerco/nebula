import React from 'react'
import { Navbar } from './Navbar'

export default function Readme({ currentPage }: any) {
  return (
    <div className="p-3">
      <Navbar currentPage="#/" />
      <h2>What is it?</h2>
      <p>
        Nebula is a tool for easing development of Soroban smart contracts being
        highly cohesive with the existing ecosystem.
      </p>
      <h2>Development</h2>
      <p>
        Nebula aims to provide the following functionality:
        <ul>
          <li>
            A contract wizard UI, that will help developers to graphically
            compose smart contracts from templates which are ready for
            production use and audited.
          </li>
          <li>
            A highly cohesive Soroban Rust contract library, that will contain
            the contracts that the wizard will use. Developers can also make use
            of this library interfaces and implementations directly.
          </li>
          <li>
            A contract management system via{' '}
            <a href="https://github.com/eigerco/nebula/discussions/11">
              <code>#import</code>
            </a>
          </li>
        </ul>
        For an updated status of current developments visit our{' '}
        <a href="https://github.com/eigerco/nebula">github repository</a> issues
        and discussions.
      </p>
      <h2>Common interfaces </h2>
      <p>
        Nebula aims to provide interfaces for commonly used contacts. This is
        not an exhaustive list.
        <ul>
          <li>Math</li>
          <li>Voting</li>
          <li>Raffle</li>
          <li>Lottery</li>
          <li>Split Payments</li>
          <li>Auction</li>
          <li>MarketPlace</li>
          <li>Pause</li>
          <li>Payment splitter</li>
          <li>Reverse billing splitting</li>
        </ul>
      </p>
    </div>
  )
}
