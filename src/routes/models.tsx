import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import { useModelContext } from "../context/ModelsProvider";
import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import { useEventLoopContext } from "../context/EventLoopProvider";
import ModelCard from "../components/downloadedModelCard";

export default function Models() {
  const { models, downloadModel, downloadedModels } = useModelContext();

  return (
    <div className="flex flex-col w-full p-10 h-screen overflow-y-auto">
      {/* Downloaded Models */}
      <div className="flex flex-col text-5xl text-primary-content w-full">
        <div className="flex flex-col">
          <div className="pl-4">Downloaded Models</div>
        </div>
        <ul className="mt-4">
          {downloadedModels.map((model) => {
            return <ModelCard model={model} />;
          })}
        </ul>
      </div>
      {/* All Models */}
      <div className="flex flex-col text-5xl text-primary-content w-full mt-4">
        <div className="flex flex-col justify-between">
          <div className="pl-4">All Models</div>
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
                  <button
                    className="btn btn-neutral text-lg"
                    onClick={() => {
                      downloadModel(model.filename);
                    }}
                  >
                    Download
                  </button>
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
