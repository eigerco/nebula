import React from 'react'
import Image from 'next/image'
import Link from "next/link"

import Container from 'react-bootstrap/Container';
import Nav from 'react-bootstrap/Nav';
import Navbar from 'react-bootstrap/Navbar';
import NavDropdown from 'react-bootstrap/NavDropdown';

function CustomNavbar() {
  return (
    <Navbar expand="lg" className="bg-body-tertiary">
      <Container>
        <Navbar.Brand as={Link} href="/"><Image src={'/logo.svg'} width="32" height={"32"} alt="logo" /> Nebula </Navbar.Brand>
        <Navbar.Toggle aria-controls="basic-navbar-nav" />
        <Navbar.Collapse id="basic-navbar-nav">
          <Nav className="me-auto">
            <Nav.Link as={Link} href="/">Home</Nav.Link>
            <Nav.Link as={Link} href="/wizard">Wizard</Nav.Link>
            <NavDropdown title="Contracts" id="basic-nav-dropdown">
              <NavDropdown.Item as={Link} href="/contracts/raffle">
                Raffle
              </NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/contracts/voting">
                Voting
              </NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/contracts/lottery">
                Lottery
              </NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/contracts/governance">
                Governance
              </NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/contracts/marketplace">
                Marketplace
              </NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/contracts/payment-splitter">
                Payment splitter
              </NavDropdown.Item>
              <NavDropdown.Divider />
              <NavDropdown.Item href="https://github.com/eigerco/nebula">
                View in Github
              </NavDropdown.Item>
            </NavDropdown>
            <NavDropdown title="Tools" id="basic-nav-dropdown">
              <NavDropdown.Item as={Link} href="/docs/importer">Nebula Importer</NavDropdown.Item>
              <NavDropdown.Item as={Link} href="/docs/publish">Publishing Contacts</NavDropdown.Item>
              <NavDropdown.Divider />
              <NavDropdown.Item href="https://github.com/eigerco/nebula">
                View in Github
              </NavDropdown.Item>
            </NavDropdown>
          </Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
}

export default CustomNavbar;
