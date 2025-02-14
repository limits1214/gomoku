'use server'

import { cookies } from "next/headers";
import setCookie from 'set-cookie-parser';
import * as jose from 'jose'

export async function RefreshToken() {
  const cookieStore = await cookies()
 
  cookieStore.set('name', 'lee')
  // or
  cookieStore.set('name', 'lee', { secure: true })
  // or
  cookieStore.set({
    name: 'name',
    value: 'lee',
    httpOnly: true,
    path: '/',
  })
}


export async function Test(a: number) {
  console.log('server', a)
  return 1
}



export const SignupGuestAction = async (nickName: string) => {
  const cookieStore = await cookies()
    // console.log('nickName', nickName)
    const res = await fetch(`${process.env.API_URL}/auth/signup/guest`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        nickName
      })
    });
    const cookieHeader = res.headers.get('set-cookie');
    if (cookieHeader) {
      const parsedCookies = setCookie.parse(cookieHeader, { map: true });
      const refreshToken = parsedCookies['refresh_token'];
      cookieStore.set('refresh_token', refreshToken.value, {
        httpOnly: true,
        maxAge: refreshToken.maxAge,
      });
    }
    const json = await res.json();
    // console.log(json)
    const accessToken = json.data.accessToken;
    // const decoded = jwt.decode(accessToken, {complete: true});
    const decoded = jose.decodeJwt(accessToken)
    // const payload = decoded!.payload as JwtPayload;
    const accMaxAge = decoded.exp! - decoded.iat!
    cookieStore.set('access_token', accessToken, {
      httpOnly: true,
      maxAge: accMaxAge,
    });
    return 1
  }