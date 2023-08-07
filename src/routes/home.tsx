import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Home() {
  const { flowPaths } = useLocalFileContext();
  const { tables } = useSqlContext();
  return (
    <div className="flex flex-row h-full w-full m-10">
      {/* FLows */}
      <div className="flex flex-col text-5xl text-white m-5 ">
        <div className="m-2">Flows</div>
        <ul>
          {flowPaths.map((flow) => {
            return (
              <Link
                key={flow.name}
                to={`flows/${flow.name}`}
                className="card w-96 bg-base-300 shadow-xl my-2"
              >
                <div className="card-body">
                  <h2 className="card-title">{flow.name}</h2>
                  {/* <p>Flow Description</p> */}
                  {/* <div className="card-actions justify-end">
                  <button className="btn btn-primary">Buy Now</button>
                </div> */}
                </div>
              </Link>
            );
          })}
        </ul>
      </div>
      {/* Tables */}
      <div className="flex flex-col text-5xl text-white m- w-96">
        <div className="m-2">Vectors</div>
        <ul></ul>
      </div>
      {/* Tables */}
      <div className="flex flex-col text-5xl text-white m-5">
        <div className="m-2">Tables</div>
        <ul>
          {tables.map((table) => {
            return (
              <Link
                key={table.name}
                to={`tables/${table.name}`}
                className="card w-96 bg-base-300 shadow-xl my-2"
              >
                <div className="card-body">
                  <h2 className="card-title">{table.name}</h2>
                  {/* <p>Flow Description</p> */}
                  {/* <div className="card-actions justify-end">
                  <button className="btn btn-primary">Buy Now</button>
                </div> */}
                </div>
              </Link>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
