import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { invoke } from "@tauri-apps/api";

type ModelPromptTemplate = {
  name: string;
  warmup: string;
  template: string;
};

export type Model = {
  recommended: boolean;
  name: string;
  description: string;
  filename: string;
  url: string;
  parameterCount: string;
  quantization: string;
  labels: string[]; // Assuming the labels array contains strings
};

interface ModelContextInterface {
  models: Model[];
  downloadedModels: Model[];
  architectures: any[];
  modelPromptTemplates: ModelPromptTemplate[];
  downloadModel: (fileName: string) => Promise<any>;
  callModel: (prompt: string) => Promise<any>;
}

export const ModelContext = createContext<ModelContextInterface>({
  models: [],
  downloadedModels: [],
  architectures: [],
  modelPromptTemplates: [],
  downloadModel: async (fileName: string) => {},
  callModel: async (prompt: string) => {},
});

export const useModelContext = () => useContext(ModelContext);

export const ModelProvider = ({ children }: { children: ReactNode }) => {
  const [models, setModels] = useState<Model[]>([]);
  const [downloadedModels, setDownloadedModels] = useState<Model[]>([]);
  const [modelPromptTemplates, setModelPromptTemplates] = useState<
    ModelPromptTemplate[]
  >([]);

  const [architectures, setArchitectures] = useState<any[]>([]);

  const downloadModel = async (filename: string) => {
    const result = await invoke("plugin:local_models|download_model", {
      filename,
    });
    return result;
  };

  const callModel = async (prompt: string) => {
    const result = await invoke("plugin:local_models|call_model", {
      prompt,
    });
    return result;
  };

  useEffect(() => {
    invoke("get_prompt_templates").then((result) => {
      // console.log("Prompt Templates from plugin" + JSON.stringify(result));
      setModelPromptTemplates(result as ModelPromptTemplate[]);
    });

    invoke("get_architectures").then((result) => {
      // console.log("Architectures from plugin" + JSON.stringify(result));
      setArchitectures(result as any[]);
    });

    invoke("get_models").then((result) => {
      // console.log("Models from plugin" + JSON.stringify(result));
      setModels(result as Model[]);
    });

    invoke("get_downloaded_models").then((result) => {
      // console.log("Downloaded Models from plugin" + JSON.stringify(result));
      setDownloadedModels(result as Model[]);
    });
  }, []);

  return (
    <ModelContext.Provider
      value={{
        models,
        downloadedModels,
        modelPromptTemplates,
        architectures,
        downloadModel,
        callModel,
      }}
    >
      {children}
    </ModelContext.Provider>
  );
};
