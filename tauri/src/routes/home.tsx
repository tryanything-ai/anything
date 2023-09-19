import { useLocalFileContext } from "../context/LocalFileProvider";

import { Link } from "react-router-dom";

import { useSqlContext } from "../context/SqlProvider";

import BaseCard from "../components/baseCard";

export default function Home() {
  const { flowPaths, createNewFlow } = useLocalFileContext();

  const { tables } = useSqlContext();

  return (
    <div className="flex flex-row h-full w-full m-10">
      {/* FLows */}

      <div className="flex flex-col text-5xl m-5">
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
              <BaseCard
                as={Link}
                key={flow.name}
                to={`flows/${flow.name}`}
                className="w-96"
              >
                <h2 className="card-title">{flow.name}</h2>
              </BaseCard>
            );
          })}
        </ul>
      </div>

      {/* Tables */}

      <div className="flex flex-col text-5xl m- w-96 m-5">
        <div className="flex flex-row justify-between">
          <div>Vectors</div>

          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              // createNewVector();
            }}
          >
            New Vector
          </button>
        </div>

        <ul></ul>
      </div>

      {/* Tables */}

      <div className="flex flex-col text-5xl m-5">
        <div className="flex flex-row justify-between">
          <div>Tables</div>

          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              // createNewTable();
            }}
          >
            New Table
          </button>
        </div>

        <ul>
          {tables.map((table) => {
            return (
              <BaseCard
                as={Link}
                key={table.name}
                to={`tables/${table.name}`}
                className="w-96"
              >
                <h2 className="card-title">{table.name}</h2>
              </BaseCard>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
