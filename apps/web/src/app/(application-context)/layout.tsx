import { AnythingProvider } from "@/context/AnythingContext";

interface ProtectedLayoutProps {
  children: React.ReactNode;
}

export default function ApplicationContextLayout({ children }: ProtectedLayoutProps): JSX.Element {
  return (
    <AnythingProvider>
      {children}
    </AnythingProvider>
  );
}