import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import { useModelContext } from "../context/ModelsProvider";
import { invoke } from "@tauri-apps/api";
import { useEffect } from "react";
import { useEventLoopContext } from "../context/EventLoopProvider";

export default function Models() {
  const { subscribeToEvent } = useEventLoopContext();

  useEffect(() => {
    subscribeToEvent("model_loading", (event: any) => {
      console.log("model_download_progress event received");
      console.log(event);
    });
  }, []);

  const {
    models,
    downloadModel,
    downloadedModels,
    architectures,
    modelPromptTemplates,
  } = useModelContext();
  return (
    <div className="flex w-full p-10 h-screen overflow-y-auto">
      {/* Downloaded Models */}
      <div className="flex flex-col text-5xl text-primary-content m-5 w-full">
        <div className="flex flex-row justify-between">
          <div>Downloaded Models</div>
        </div>
        <ul className="mt-4">
          {downloadedModels.map((model) => {
            return (
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
                      invoke("start", {
                        modelFilename: model.filename,
                        architecture: architectures[0].id,
                        tokenizer: "embedded",
                        contextSize: 2048,
                        useGpu: true,
                        prompt: modelPromptTemplates[0],
                        contextFiles: [],
                      });
                    }}
                  >
                    Start
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
      {/* All Models */}
      <div className="flex flex-col text-5xl text-primary-content m-5 w-full">
        <div className="flex flex-row justify-between">
          <div>All Models</div>
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
