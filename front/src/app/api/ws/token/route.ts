import { cookies } from 'next/headers';
export async function GET() {
  const cookieStore = await cookies();
  

  const header: Record<string, string> = {
    "Content-Type": "application/json"
  };

  const access_token = cookieStore.get('access_token');
  if (access_token) {
    header["Authorization"] = `Bearer ${access_token.value}`
  }
  
  const res = await fetch(`${process.env.API_URL}/ws/temptoken/issue`, {
    method: 'GET',
    credentials: 'include',
    headers: header,
  });

  const j = await res.json();
  // const data = j.data;
  // cookieStore.set({
  //   name:"TEST_COOKIE", 
  //   value: "asdf",
  //   httpOnly: true,
  // });
  const r = Response.json(j.data);


  return r;
}