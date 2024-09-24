import { Suspense } from "react"; //https://nextjs.org/docs/messages/missing-suspense-with-csr-bailout
import LoginPage from "@/components/auth/login-component";

const SignupPage = () => {
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <LoginPage />
    </Suspense>
  );
};

export default SignupPage;
