import { Link, useRouteError } from "react-router-dom";
import PageLayout from "./pageLayout";

export default function ErrorPage() {
  const error: any = useRouteError();
  console.error(error);

  return (
    <PageLayout>
      <div
        style={{
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          height: "70vh",
        }}
      >
        <img
          src="/404.svg"
          alt="Error 404"
          style={{ maxWidth: "100%", maxHeight: "100%" }}
        />
      </div>
      <p className="flex flex-col justify-center items-center pt-5">
        <Link className="btn btn-primary" to="/">
          Go Home
        </Link>
        <p className="pt-10">
          <i>{error.statusText || error.message}</i>
        </p>
      </p>
    </PageLayout>
  );
}
