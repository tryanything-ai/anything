import { Inter as FontSans } from "next/font/google"
import "./globals.css";
import { cn } from "@/lib/utils";
import { AnythingProvider } from "@/context/AnythingContext";

const defaultUrl = process.env.NEXT_PUBLIC_URL as string || "http://localhost:3000";

const fontSans = FontSans({
  subsets: ["latin"],
  variable: "--font-sans",
})

export const metadata = {
  metadataBase: new URL(defaultUrl),
  title: "Anything AI",
  description: "Use tomorrows AI in your business today",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode; 
}) {
  return (
    <html lang="en" className={cn(
      "min-h-screen bg-background font-sans antialiased",
      fontSans.variable
    )}>
      <body className="bg-background text-foreground">
        <AnythingProvider>
          <main className="min-h-screen">
            {children}
          </main>
        </AnythingProvider>
      </body>
    </html>
  );
}
