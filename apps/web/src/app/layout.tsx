import { Inter as FontSans } from "next/font/google";
import "./globals.css";
import "@repo/ui/globals.css";
import { cn } from "@/lib/utils";
import { AnythingProvider } from "@/context/AnythingContext";

const defaultUrl: string =
  process.env.NODE_ENV === "production"
    ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`
    : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

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
        <AnythingProvider>
          <main className="min-h-screen">{children}</main>
        </AnythingProvider>
      </body>
    </html>
  );
}
