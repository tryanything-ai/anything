import { Link, useRouteError } from "react-router-dom";
import PageLayout from "./pageLayout";

export default function ErrorPage() {
  const error: any = useRouteError();
  console.error(error);

  return (
    <PageLayout>
      <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh' }}>
        <img src="/404.svg" alt="Error 404" style={{ maxWidth: '100%', maxHeight: '100%' }} />
      </div>
      <p>
        Go{" "}
        <Link className="btn btn-primary" to="/">
          home
        </Link>
      </p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </PageLayout>
  );
}
