import { ReactNode } from "react";
import { useAuthenticationContext } from "./context/AuthenticaionProvider";
import { Link } from "react-router-dom";

export default function PageLayout({
  children,
  requireAuth = false,
}: {
  children: ReactNode;
  requireAuth?: boolean;
}) {
  const { session } = useAuthenticationContext();

  if (!session && requireAuth) {
    return (
      <PageLayout>
        <Link to="/login" className="btn btn-primary m-1 ml-4">
          Login
        </Link>
      </PageLayout>
    );
  }

  return (
    <div className="flex flex-col p-14 h-full w-full hide-scrollbar overflow-scroll">
      {children}
    </div>
  );
}
