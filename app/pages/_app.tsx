import * as React from 'react'
import { AppProps } from 'next/app'
import Head from 'next/head'

import 'bootstrap/dist/css/bootstrap.min.css'
import 'bootstrap-icons/font/bootstrap-icons.css'

import '../src/index.css'

export default function MyApp({ Component, pageProps }: AppProps): JSX.Element {
  return (
    <html data-bs-theme="dark">
      <Head>
        <title>Nebula</title>
        <meta
          name="description"
          content="Nebula - A platform for building Soroban smart contracts"
        />
        <meta charSet="utf-8" />
        <link rel="icon" href="/favicon.ico" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <meta name="theme-color" content="#000000" />
        <meta
          name="description"
          content="Nebula - A platform for building Soroban smart contracts"
        />
        <link rel="apple-touch-icon" href="/logo192.png" />
        <link rel="manifest" href="/manifest.json" />
      </Head>
      <Component {...pageProps} />
    </html>
  )
}
