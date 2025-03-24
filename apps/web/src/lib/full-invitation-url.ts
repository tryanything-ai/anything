const defaultUrl: string = process.env.NEXT_PUBLIC_APP_URL || ""; 

export default function fullInvitationUrl(token: string) {
    return `${defaultUrl}/invitation?token=${token}`;
}