// lib/api.ts
// If using Next.js routes as proxy:
export async function createDeposit(depositData: any) {
  const response = await fetch('/api/deposit', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(depositData),
  });
  return response.json();
}

export async function getDeposits() {
  const response = await fetch('/api/deposits');
  return response.json();
}

/*
// If calling Rust server directly (bypassing Next.js):
export async function createDeposit(depositData: any) {
  const response = await fetch('http://localhost:3001/deposits', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(depositData),
  });
  return response.json();
}

export async function getDeposits() {
  const response = await fetch('http://localhost:3001/deposits');
  return response.json();
}
*/
