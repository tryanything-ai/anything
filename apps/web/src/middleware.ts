import { type NextRequest } from "next/server";
// import { validateSession } from "@/lib/supabase/middleware";
import { updateSession } from "./lib/supabase/middleware";
import { NextResponse } from 'next/server'

export async function middleware(request: NextRequest) {
  //Skip if the request is for the oauth integrations callbacks
  //TODO BRING BACK
  // if (request.nextUrl.pathname.startsWith('/auth/') && request.nextUrl.pathname.endsWith('/callback')) {
  //   console.log('Skipping session update for auth callback route');
  //   return NextResponse.next();
  // }

  return await updateSession(request);
}

export const config = {
  matcher: [
    /*
     * Match all request paths except for the ones starting with:
     * - _next/static (static files)
     * - _next/image (image optimization files)
     * - favicon.ico (favicon file)
     * Feel free to modify this pattern to include more paths.
     */
    '/((?!_next/static|_next/image|favicon.ico|.*\\.(?:svg|png|jpg|jpeg|gif|webp)$).*)',
  ],
};
