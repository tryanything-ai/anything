export const AUTH_EVENTS = {
  LOGIN_ATTEMPT: "login_attempt",
  LOGIN_SUCCESS: "login_success",
  LOGIN_ERROR: "login_error",
  SIGNUP_ATTEMPT: "signup_attempt",
  SIGNUP_SUCCESS: "signup_success",
  SIGNUP_ERROR: "signup_error",
  EMAIL_VERIFICATION_SENT: "email_verification_sent",
  MARKETING_LOGIN_CLICK: "marketing_login_click",
  MARKETING_SIGNUP_CLICK: "marketing_signup_click",
  MARKETING_TEMPLATE_VIEW: "marketing_template_view",
  MARKETING_INTEGRATION_VIEW: "marketing_integration_view",
  MARKETING_DISCORD_CLICK: "marketing_discord_click"
} as const;