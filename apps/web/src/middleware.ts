import { type NextRequest } from "next/server";
// import { validateSession } from "@/lib/supabase/middleware";
import { updateSession } from "./lib/supabase/new_middleware";
import { NextResponse } from 'next/server'

export async function middleware(request: NextRequest) {
  // return await validateSession(request);
  //Skip if the request is for the oauth integrations callbacks
  // if (request.nextUrl.pathname.match(/^\/auth\/[^\/]+\/callback$/)) {
  //   console.log('Skipping middleware for oauth callback for integrations')
  //   const res = NextResponse.next()
  //   return res; 
  // }
  if (request.nextUrl.pathname.startsWith('/auth/') && request.nextUrl.pathname.endsWith('/callback')) {
    console.log('Skipping session update for auth callback route');
    return NextResponse.next();
  }

  return await updateSession(request);
}

export const config = {
  matcher: [
    /*
     * Match all request paths except:
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * - images - .svg, .png, .jpg, .jpeg, .gif, .webp
     * Feel free to modify this pattern to include more paths.
     */
    // '/auth/:path*/callback',
    "/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp)$).*)",
  ],
};
