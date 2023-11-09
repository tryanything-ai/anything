import { ReactNode } from "react";
import { useAuthenticationContext } from "../context/AuthenticaionProvider";
import { Link } from "react-router-dom";

export default function RequireAuth({ children }: { children: ReactNode }) {
  const { session } = useAuthenticationContext();

  if (!session) {
    return (
      <>
        <p className="text-sm w-full text-center pt-6">
          Login Required for this feature
        </p>

        <div className="flex justify-center mt-6">
          <Link to="/login" className="btn btn-primary">
            Login
          </Link>
        </div>
      </>
    );
  }

  return <>{children}</>;
}
