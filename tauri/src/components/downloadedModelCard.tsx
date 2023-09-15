import { useModelContext } from "../context/ModelsProvider";
import api from '../tauri_api/api'; 
import { useEffect, useState } from "react";
import { useEventLoopContext } from "../context/EventLoopProvider";

import { Model } from "../context/ModelsProvider";

const DownloadedModelCard = ({ model }: { model: Model }) => {
  const [loading, setLoading] = useState<boolean>(false);
  const [progress, setProgress] = useState<number>(0);
  const [message, setMessage] = useState<string>("");

  const start = () => {
    setLoading(true);
    api.startModel({
      modelFilename: model.filename,
      architecture: architectures[0].id,
      tokenizer: "embedded",
      contextSize: 1048,
      useGpu: false,
      prompt: modelPromptTemplates[0],
      contextFiles: [],
    }); 
  };
  const {
    architectures,
    modelPromptTemplates,
  } = useModelContext();
  const { subscribeToEvent } = useEventLoopContext();

  useEffect(() => {
   let unlisten =  subscribeToEvent("model_loading", (event: any) => {
      setLoading(true);
      setProgress(event.progress);
      setMessage(event.message);
    });

    return () => {
      unlisten.then((unlisten) => unlisten()); 
    }
  }, []);

  return (
    <div
      key={model.filename}
      className="card w-full bg-base-300 shadow-xl my-2 text-5xl"
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
          disabled={loading}
          onClick={start}
        >
          {loading ? "Loading" : "Start"}
        </button>
      </div>
      <div></div>
      <div className="p-5 justify-normal">
        {loading ? (
          <>
            <div className="text-sm -my-5">{message}</div>
            <progress
              className="progress progress-secondary"
              value={progress * 100}
              max="100"
            />
          </>
        ) : null}
      </div>
    </div>
  );
};
export default DownloadedModelCard;
