import React from 'react'
import { Nav } from 'react-bootstrap'
import './Navbar.css'
import logo from './logo.png'

export function Navbar({ currentPage }: any) {
  return (
    <div className="menu">
      <Nav
        variant="tabs"
        defaultActiveKey={currentPage}
        className="navbar navbar-expand-md navbar-dark fixed-top bg-dark"
      >
        <h1
          className="mx-3 my-auto"
          style={{ fontSize: '1.8rem', textAlign: 'justify' }}
        >
          <img src={logo} width="32" alt="logo" /> Nebula
        </h1>
        <Nav.Item>
          <Nav.Link href="#/">About</Nav.Link>
        </Nav.Item>
        <Nav.Item>
          <Nav.Link href="#/wizard">Wizard</Nav.Link>
        </Nav.Item>
      </Nav>
    </div>
  )
}
