"use client";

import { useState } from 'react';
import { LaunchpadForm } from '@/components/LaunchpadForm';
import { DepositList } from '@/components/DepositList';
import { Header } from '@/components/Header';

export default function Home() {
  const [activeDeposits, setActiveDeposits] = useState<any[]>([]);

  return (
    <main className="min-h-screen bg-gray-100">
      <Header />
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <LaunchpadForm 
            onDeposit={(deposit) => setActiveDeposits([...activeDeposits, deposit])} 
          />
          <DepositList deposits={activeDeposits} />
        </div>
      </div>
    </main>
  );
}