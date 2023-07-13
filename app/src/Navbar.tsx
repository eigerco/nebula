import React from 'react'
import { Nav } from 'react-bootstrap'
import './Navbar.css'

export function Navbar ({ currentPage }: any) {
  return (
    <div className="menu">
    <Nav variant="tabs" defaultActiveKey={currentPage} className="navbar navbar-expand-md navbar-dark fixed-top bg-dark">
      <Nav.Item>
        <Nav.Link href="/">Readme</Nav.Link>
      </Nav.Item>
      <Nav.Item>
        <Nav.Link href="/wizard">Wizard</Nav.Link>
      </Nav.Item>
    </Nav>
    </div>
  )
}
