import { NextRequest, NextResponse } from "next/server";

export async function GET(req: NextRequest) {
  const { searchParams } = new URL(req.url);
  const channel = searchParams.get("channel");
  const roomNum = searchParams.get("roomNum");
  if (channel == null || roomNum == null) {
    return NextResponse.json({ error: "query string issue" },{ status: 400 });
  }
  
  const params = new URLSearchParams({
    channel, roomNum
  });
  const queryString = params.toString();
  const res = await fetch(`${process.env.API_URL}/room/info/channelroom?${queryString}`, {
    method: 'GET',
    credentials: 'include',
  });
  const j = await res.json();

  return NextResponse.json(j.data);
}