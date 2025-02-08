import { PostHog } from 'posthog-node'

export default function PostHogClient() {
  const posthogClient = new PostHog(process.env.NEXT_PUBLIC_POSTHOG_KEY!, {
    host: process.env.NEXT_PUBLIC_POSTHOG_HOST,
    flushAt: 1,
    flushInterval: 0
  })
  return posthogClient
}

export const MARKETING_EVENTS = {
  HOME_VIEW: "marketing_home_view",
  LOGIN_CLICK: "marketing_login_click",
  SIGNUP_CLICK: "marketing_signup_click",
  TEMPLATE_VIEW: "marketing_template_view",
  INTEGRATION_VIEW: "marketing_integration_view",
  DISCORD_CLICK: "marketing_discord_click"
} as const;