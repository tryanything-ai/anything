const defaultUrl: string = process.env.NODE_ENV === 'production' 
  ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}` 
  : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

export default function fullInvitationUrl(token: string) {
    return `${defaultUrl}/invitation?token=${token}`;
}