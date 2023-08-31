import { Link } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Tables() {
  const { tables } = useSqlContext();

  return (
    <div className="flex flex-col h-full w-full m-10">
      <div className="text-5xl text-white mx-5">Tables</div>
      <div className="text-2xl my-2 mx-5">A local sqlite database</div>
      <ul>
        {tables.map((table) => {
          return (
            <li key={table.name} className="text-2xl my-2 mx-5">
              <Link to={`/tables/${table.name}`}>{table.name}</Link>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
