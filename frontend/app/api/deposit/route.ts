// app/api/deposit/route.ts
import { NextResponse } from 'next/server';

export async function POST(req: Request) {
  const data = await req.json();
  // Connect to your Rust backend here
  return NextResponse.json({ success: true, data });
}

// app/api/deposits/route.ts
export async function GET() {
  // Get deposits from your Rust backend
  return NextResponse.json({ deposits: [] });
}