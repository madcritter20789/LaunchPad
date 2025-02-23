  "use client";
  
  import { useState } from 'react';
  import { Card } from '@/components/ui/card';
  
  interface LaunchpadFormProps {
    onDeposit: (deposit: any) => void;
  }
  
  export function LaunchpadForm({ onDeposit }: LaunchpadFormProps) {
    const [formData, setFormData] = useState({
      chain: '',
      amount: '',
      lockDuration: '',
      tokenAddress: ''
    });
  
    const handleSubmit = async (e: React.FormEvent) => {
      e.preventDefault();
      // Here you would integrate with your Rust backend
      onDeposit({
        ...formData,
        id: Date.now(),
        status: 'Active',
        createdAt: new Date().toISOString(),
        releaseDate: new Date(Date.now() + parseInt(formData.lockDuration) * 86400000).toISOString()
      });
      
      setFormData({
        chain: '',
        amount: '',
        lockDuration: '',
        tokenAddress: ''
      });
    };
  
    return (
      <Card className="p-6">
        <h2 className="text-2xl font-semibold mb-4">Create New Deposit</h2>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Chain
            </label>
            <select
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
              value={formData.chain}
              onChange={(e) => setFormData({ ...formData, chain: e.target.value })}
              required
            >
              <option value="">Select Chain</option>
              <option value="ethereum">Ethereum</option>
              <option value="binance">Binance Smart Chain</option>
              <option value="polygon">Polygon</option>
            </select>
          </div>
  
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Token Address
            </label>
            <input
              type="text"
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
              value={formData.tokenAddress}
              onChange={(e) => setFormData({ ...formData, tokenAddress: e.target.value })}
              required
            />
          </div>
  
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Amount
            </label>
            <input
              type="number"
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
              value={formData.amount}
              onChange={(e) => setFormData({ ...formData, amount: e.target.value })}
              required
            />
          </div>
  
          <div>
            <label className="block text-sm font-medium text-gray-700">
              Lock Duration (days)
            </label>
            <input
              type="number"
              className="mt-1 block w-full rounded-md border-gray-300 shadow-sm focus:border-indigo-500 focus:ring-indigo-500"
              value={formData.lockDuration}
              onChange={(e) => setFormData({ ...formData, lockDuration: e.target.value })}
              required
            />
          </div>
  
          <button
            type="submit"
            className="w-full bg-indigo-600 text-white py-2 px-4 rounded-md hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
          >
            Create Deposit
          </button>
        </form>
      </Card>
    );
  }
  