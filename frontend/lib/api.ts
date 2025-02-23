// lib/api.ts
export async function createDeposit(depositData: any) {
    const response = await fetch('/api/deposit', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(depositData),
    });
    return response.json();
  }
  
  export async function getDeposits() {
    const response = await fetch('/api/deposits');
    return response.json();
  }