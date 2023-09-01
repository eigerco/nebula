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
              <NavDropdown.Item href="/contracts/raffle">Raffle</NavDropdown.Item>
              <NavDropdown.Item href="#action/3.2">
                Voting
              </NavDropdown.Item>
              <NavDropdown.Item href="#action/3.3">Something</NavDropdown.Item>
              <NavDropdown.Divider />
              <NavDropdown.Item href="#action/3.4">
                Separated link
              </NavDropdown.Item>
            </NavDropdown>
          </Nav>
        </Navbar.Collapse>
      </Container>
    </Navbar>
  );
}

export default CustomNavbar;

// export function Navbar({ currentPage }: any) {
//   return (
//     <div className="menu">
//       <Nav
//         variant="tabs"
//         defaultActiveKey={currentPage}
//         className="navbar navbar-expand-md navbar-dark fixed-top bg-dark"
//       >
//         <h1
//           className="mx-3 my-auto"
//           style={{ fontSize: '1.8rem', textAlign: 'justify' }}
//         >
//           <Image src={'/logo.svg'} width="32" height={"32"} alt="logo" /> Nebula 
//         </h1>
//         <Nav.Item>
//           <Nav.Link as={Link} href="/">About</Nav.Link>
//         </Nav.Item>
//         <Nav.Item>
//           <Nav.Link as={Link} href="/wizard">Wizard</Nav.Link>
//         </Nav.Item>
//       </Nav>
//     </div>
//   )
// }
