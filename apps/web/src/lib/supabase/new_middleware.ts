import { createServerClient } from '@supabase/ssr'
import { NextResponse, type NextRequest } from 'next/server'

export async function updateSession(request: NextRequest) {
  console.log('updateSession called with request:', request)
  
  let supabaseResponse = NextResponse.next({
    request,
  })

  console.log('Initial supabaseResponse:', supabaseResponse)

  const supabase = createServerClient(
    process.env.NEXT_PUBLIC_SUPABASE_URL!,
    process.env.NEXT_PUBLIC_SUPABASE_ANON_KEY!,
    {
      cookies: {
        getAll() {
          const allCookies = request.cookies.getAll()
          console.log('Cookies getAll:', allCookies)
          return allCookies
        },
        setAll(cookiesToSet) {
          console.log('Cookies setAll called with:', cookiesToSet)
          cookiesToSet.forEach(({ name, value, options }) => {
            console.log(`Setting cookie: ${name}=${value}`, options)
            request.cookies.set(name, value)
          })
          supabaseResponse = NextResponse.next({
            request,
          })
          console.log('Updated supabaseResponse after setting cookies:', supabaseResponse)
          cookiesToSet.forEach(({ name, value, options }) => {
            console.log(`Setting cookie in supabaseResponse: ${name}=${value}`, options)
            supabaseResponse.cookies.set(name, value, options)
          })
        },
      },
    }
  )
  console.log('Supabase client created:', supabase)

  // IMPORTANT: Avoid writing any logic between createServerClient and
  // supabase.auth.getUser(). A simple mistake could make it very hard to debug
  // issues with users being randomly logged out.

  // IMPORTANT: Avoid writing any logic between createServerClient and
  // supabase.auth.getUser(). A simple mistake could make it very hard to debug
  // issues with users being randomly logged out.

  const {
    data: { user },
  } = await supabase.auth.getUser()
  console.log('User fetched from supabase:', user)
  if (user) {
    if (request.nextUrl.pathname.startsWith('/login') || request.nextUrl.pathname.startsWith('/signup')) {
      console.log('User found on login/signup page, redirecting to /')
      const url = request.nextUrl.clone()
      url.pathname = '/'
      return NextResponse.redirect(url)
    }
  } else if (
    !request.nextUrl.pathname.startsWith('/login') &&
    !request.nextUrl.pathname.startsWith('/signup')
    // &&
    // !request.nextUrl.pathname.startsWith('/auth') 
  ) {
    console.log('No user found and not on login/signup page, redirecting to /login')
    const url = request.nextUrl.clone()
    url.pathname = '/login'
    return NextResponse.redirect(url)
  }

  // IMPORTANT: You *must* return the supabaseResponse object as it is. If you're
  // creating a new response object with NextResponse.next() make sure to:
  // 1. Pass the request in it, like so:
  //    const myNewResponse = NextResponse.next({ request })
  // 2. Copy over the cookies, like so:
  //    myNewResponse.cookies.setAll(supabaseResponse.cookies.getAll())
  // 3. Change the myNewResponse object to fit your needs, but avoid changing
  //    the cookies!
  // 4. Finally:
  //    return myNewResponse
  // If this is not done, you may be causing the browser and server to go out
  // of sync and terminate the user's session prematurely!

  console.log('Returning final supabaseResponse:', supabaseResponse)
  return supabaseResponse
}