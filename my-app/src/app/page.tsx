// app/page.tsx
'use client';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function Home() {
  return (
    <div>
      <WalletMultiButton />
      {/* Your dApp content */}
    </div>
  );
}
