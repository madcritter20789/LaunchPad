import { NextResponse } from 'next/server';

export async function GET() {
  // Forward to the Rust server
  const response = await fetch('http://localhost:3001/deposits');
  const depositData = await response.json();

  // depositData is an array of [id, Deposit] tuples
  return NextResponse.json(depositData);
}
