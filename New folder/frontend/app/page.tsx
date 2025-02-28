"use client";

import { useState, useEffect } from "react";
import { LaunchpadForm } from "@/components/LaunchpadForm";
import { DepositList } from "@/components/DepositList";
import { Header } from "@/components/Header";
import { createDeposit, getDeposits } from "@/lib/api";

export default function Home() {
  const [activeDeposits, setActiveDeposits] = useState<any[]>([]);

  // On mount, fetch existing deposits from Rust backend
  useEffect(() => {
    async function loadDeposits() {
      const deposits = await getDeposits();
      // deposits will be an array of [id, Deposit] tuples
      // Convert them to a simpler array of objects
      const normalized = deposits.map(([id, deposit]: [string, any]) => ({
        id,
        chain: deposit.token.chain_id,
        tokenAddress: deposit.token.contract_address,
        amount: deposit.token.amount,
        status: deposit.status,
        createdAt: new Date().toISOString(), // or store real timestamps
        releaseDate: deposit.lock_until,      // from deposit.lock_until
      }));
      setActiveDeposits(normalized);
    }
    loadDeposits();
  }, []);

  const handleDeposit = async (depositData: any) => {
    // depositData is the form input from LaunchpadForm
    // e.g. { chain, tokenAddress, amount, lockDuration, ... }

    // Build the JSON for the Rust server
    const body = {
      chain: depositData.chain,
      token_address: depositData.tokenAddress,
      amount: depositData.amount,
      lock_duration_days: parseInt(depositData.lockDuration),
      user_address: "0xUSERWALLETADDRESS", // or pass from form
    };

    // Call Rust to create deposit
    const result = await createDeposit(body);

    // Optionally re-fetch deposits or push the new deposit
    const newDeposits = await getDeposits();
    const normalized = newDeposits.map(([id, deposit]: [string, any]) => ({
      id,
      chain: deposit.token.chain_id,
      tokenAddress: deposit.token.contract_address,
      amount: deposit.token.amount,
      status: deposit.status,
      createdAt: new Date().toISOString(),
      releaseDate: deposit.lock_until,
    }));
    setActiveDeposits(normalized);
  };

  return (
    <main className="min-h-screen bg-gray-100">
      <Header />
      <div className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <LaunchpadForm onDeposit={handleDeposit} />
          <DepositList deposits={activeDeposits} />
        </div>
      </div>
    </main>
  );
}
