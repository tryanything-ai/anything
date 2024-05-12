import { Link } from "react-router-dom";

import PageLayout from "../pageLayout";
import { PageHeader } from "../components/wholePageHeader";

export default function Tables() {
  let tables = []
  return (
    <PageLayout>
      <PageHeader callback={() => { }} title="Tables" buttonLabel="New Table" />
      <ul>
        {tables.map((table) => {
          return (
            <li key={table.name} className="text-2xl my-2 mx-5">
              <Link to={`/tables/${table.name}`}>{table.name}</Link>
            </li>
          );
        })}
      </ul>
    </PageLayout>
  );
}
