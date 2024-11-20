const defaultUrl: string = process.env.NEXT_PUBLIC_HOSTED_URL || ""; 

export default function fullInvitationUrl(token: string) {
    return `${defaultUrl}/invitation?token=${token}`;
}