import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Home() {
  const { flowPaths, createNewFlow } = useLocalFileContext();
  const { tables } = useSqlContext();

  return (
    <div className="flex flex-row h-full w-full m-10">
      {/* FLows */}
      <div className="flex flex-col text-5xl text-primary-content m-5 ">
        <div className="flex flex-row justify-between">
          <div>Flows</div>
          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              createNewFlow();
            }}
          >
            New Flow
          </button>
        </div>

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
      <div className="flex flex-col text-5xl text-primary-content m- w-96 m-5">
        <div className="flex flex-row justify-between">
          <div>Vectors</div>
          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              // createNewFlow();
            }}
          >
            New Vector
          </button>
        </div>

        <ul></ul>
      </div>
      {/* Tables */}
      <div className="flex flex-col text-5xl text-primary-content m-5">
        <div className="flex flex-row justify-between">
          <div>Tables</div>
          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              // createNewFlow();
            }}
          >
            New Table
          </button>
        </div>
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
