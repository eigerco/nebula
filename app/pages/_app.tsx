import * as React from 'react'
import { AppProps } from 'next/app'
import Head from 'next/head'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap-icons/font/bootstrap-icons.css'

import '../src/index.css'
import CustomNavbar from '../src/Navbar'

export default function MyApp({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <>
      <Head>
        <title>Nebula</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta
          name="description"
          content="Nebula - A platform for building Soroban smart contracts"
        />
      </Head>
      <CustomNavbar />
      <div className="container">
        <Component {...pageProps} />
      </div>
    </>
  )
}
