import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const data = await req.json();

  // Forward this request to the Rust server
  const response = await fetch('http://localhost:3001/deposits', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(data),
  });

  // Get the deposit ID from the Rust server
  const depositId = await response.json();
  return NextResponse.json({ depositId });
}
