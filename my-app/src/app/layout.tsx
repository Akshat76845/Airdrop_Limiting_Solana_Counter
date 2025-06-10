// app/layout.tsx
'use client'
import './globals.css';
import '@solana/wallet-adapter-react-ui/styles.css';
import { WalletConnectionProvider } from '../app/components/WalletConnectionProvider';

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <body>
        <WalletConnectionProvider>
          {children}
        </WalletConnectionProvider>
      </body>
    </html>
  );
}
