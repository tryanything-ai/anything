import { Inter as FontSans } from "next/font/google";
import "@repo/ui/globals.css";
import { cn } from "@/lib/utils";
import { PostHogProvider } from "../posthog/provider";

const defaultUrl: string = process.env.NEXT_PUBLIC_HOSTED_URL!;

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
});

export const metadata = {
  metadataBase: new URL(defaultUrl),
  title: "Anything AI",
  description: "Use tomorrows AI in your business today",
};

interface RootLayoutProps {
  children: React.ReactNode;
}

export default function RootLayout({ children }: RootLayoutProps): JSX.Element {
  return (
    <html
      lang="en"
      className={cn(
        "min-h-screen bg-background font-sans antialiased",
        fontSans.variable,
      )}
    >
      <body className="bg-background text-foreground">
        <main className="min-h-screen">
          <PostHogProvider>{children}</PostHogProvider>
        </main>
      </body>
    </html>
  );
}
