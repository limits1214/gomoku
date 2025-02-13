
import { cookies } from "next/headers";
import {  NextResponse } from "next/server"
import jwt, { JwtPayload } from 'jsonwebtoken'

export async function middleware() {
  // console.log(request.url)
  // console.log('middleware')
  
  const cookieStore = await cookies();
  const refreshTokenCookie = cookieStore.get('refresh_token');
  if (refreshTokenCookie) {
    let isCallRefresh = false;

    const accessTokenCookie = cookieStore.get('access_token');
    if (accessTokenCookie) {
      const accessToken = accessTokenCookie.value
      const decoded = jwt.decode(accessToken, {complete: true});
      if (decoded) {
        const payload = decoded.payload as JwtPayload;
        const exp = payload.exp!;
        const now = Math.floor(Date.now() / 1000);
        // console.log('now: ', now, 'exp', payload.exp)
        // 10초 넉넉히
        const paddingTime = 10;
        if (exp - now < paddingTime) {
          isCallRefresh = true;
        }
      } else {
        isCallRefresh = true;
      }
    } else {
      isCallRefresh = true;
    }

    if (isCallRefresh) {
      console.log('refresh')
      const res = await fetch(`${process.env.API_URL}/auth/access/refresh`, {
        method: 'POST',
        credentials: 'include',
        headers: {
          "Content-Type": "application/json"
        },
        body: JSON.stringify({
          refreshToken: refreshTokenCookie.value
        })
      });

      const json = await res.json();
      const accessToken = json.data.accessToken;
      const decoded = jwt.decode(accessToken, {complete: true});
      const payload = decoded!.payload as JwtPayload;
      const accMaxAge = payload.exp! - payload.iat!
      cookieStore.set('access_token', accessToken, {
        httpOnly: true,
        maxAge: accMaxAge,
      });
    }
  }
  const response = NextResponse.next()
  return response
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - api (API routes)
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico, sitemap.xml, robots.txt (metadata files)
     */
    '/((?!api|_next/static|_next/image|favicon.ico|sitemap.xml|robots.txt).*)',
  ],
}