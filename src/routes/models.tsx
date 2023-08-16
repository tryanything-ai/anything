import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import { useModelContext } from "../context/ModelsProvider";

export default function Models() {
  //   const { flowPaths, createNewFlow } = useLocalFileContext();
  const { models } = useModelContext();
  return (
    <div className="flex h-full w-full p-10">
      <div className="flex flex-col text-5xl text-primary-content m-5 w-full">
        <div className="flex flex-row justify-between">
          <div>Models</div>
          {/* <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              createNewFlow();
            }}
          >
            New Modwl
          </button> */}
        </div>
        <ul className="mt-4">
          {models.map((model) => {
            return (
              // <Link
              //   key={model.filename}
              //   to={`${model.name}`}
              <div
                key={model.filename}
                className="card w-full bg-base-300 shadow-xl my-2"
              >
                <div className="card-body flex-row justify-between">
                  <div className="w-1/4">
                    <div className="text-2xl">{model.name}</div>
                    <div className="text-sm">{model.description}</div>
                  </div>
                  <div className="flex text-lg">{model.parameterCount}</div>
                  <div className="flex text-lg">{model.quantization}</div>
                  <div className="btn btn-neutral text-lg">Download</div>
                  {/* <h2 className="card-title">{flow.name}</h2>
                  <div className="card-actions justify-end">
                    <div className="bg-pink-200 h-full w-full">derp</div>
                  </div> */}
                </div>
                {/* </Link> */}
              </div>
            );
          })}
        </ul>
      </div>
    </div>
  );
}
