import { cookies } from 'next/headers';
export async function GET() {
  const cookieStore = await cookies();
  cookieStore.set({
    name:"TEST_COOKIE", 
    value: "asdf",
    httpOnly: true,
  });
  const r = Response.json({hi: "hi"});
  // r.headers.set(
  //   "Set-Cookie",
  //   "auth_token=your_secret_token; HttpOnly; Secure; Path=/; Max-Age=86400"
  // );

  return r;
}