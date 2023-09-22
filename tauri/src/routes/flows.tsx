import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";

export default function Flows() {
  const { createNewFlow, flows } = useLocalFileContext();

  return (
    <div className="flex h-full w-full p-10">
      <div className="flex flex-col text-5xl m-5 w-full">
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
        <ul className="mt-4">
          {flows.map((flow) => {
            return (
              <Link
                key={flow.name}
                to={`${flow.name}`}
                className="card w-full bg-base-300 shadow-xl my-2"
              >
                <div className="card-body flex-row justify-between">
                  <div className="w-1/4">
                    <div className="text-2xl">{flow.name}</div>
                  </div>
                  <div className="flex text-lg">Stats</div>
                  <div className="flex text-lg">Live</div>
                  {/* <h2 className="card-title">{flow.name}</h2>
                  <div className="card-actions justify-end">
                    <div className="bg-pink-200 h-full w-full">derp</div>
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
