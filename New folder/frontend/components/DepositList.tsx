import { Card } from '@/components/ui/card';

interface Deposit {
  id: number;
  chain: string;
  amount: string;
  status: string;
  createdAt: string;
  releaseDate: string;
  tokenAddress: string;
}

interface DepositListProps {
  deposits: Deposit[];
}

export function DepositList({ deposits }: DepositListProps) {
  return (
    <Card className="p-6">
      <h2 className="text-2xl font-semibold mb-4">Active Deposits</h2>
      <div className="space-y-4">
        {deposits.map((deposit) => (
          <div
            key={deposit.id}
            className="border rounded-lg p-4 hover:shadow-md transition-shadow"
          >
            <div className="flex justify-between items-start">
              <div>
                <p className="font-semibold">{deposit.chain}</p>
                <p className="text-sm text-gray-500">
                  Amount: {deposit.amount}
                </p>
                <p className="text-sm text-gray-500">
                  Token: {deposit.tokenAddress.slice(0, 6)}...
                  {deposit.tokenAddress.slice(-4)}
                </p>
              </div>
              <span className="px-2 py-1 text-xs rounded-full bg-green-100 text-green-800">
                {deposit.status}
              </span>
            </div>
            <div className="mt-2 text-sm text-gray-500">
              <p>Created: {new Date(deposit.createdAt).toLocaleDateString()}</p>
              <p>Release: {new Date(deposit.releaseDate).toLocaleDateString()}</p>
            </div>
          </div>
        ))}
        {deposits.length === 0 && (
          <p className="text-gray-500 text-center py-4">No active deposits</p>
        )}
      </div>
    </Card>
  );
}