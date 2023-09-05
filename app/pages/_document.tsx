import { Html, Head, Main, NextScript } from 'next/document'
import React from 'react'

export default function Document() {
  return (
    <Html lang="en" data-bs-theme="dark">
      <Head>
        <meta
          name="description"
          content="Nebula - A platform for building Soroban smart contracts"
        />
        <meta charSet="utf-8" />
        <link rel="icon" href="/favicon.ico" />

        <meta name="theme-color" content="#000000" />
        <link rel="apple-touch-icon" href="/logo192.png" />
        <link rel="manifest" href="/manifest.json" />
      </Head>
      <body>
        <Main />
        <NextScript />
      </body>
    </Html>
  )
}
