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

type Model = {
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
  architectures: any[];
  modelPromptTemplates: ModelPromptTemplate[];
  downloadModel: (fileName: string) => Promise<any>;
}

export const ModelContext = createContext<ModelContextInterface>({
  models: [],
  architectures: [],
  modelPromptTemplates: [],
  downloadModel: async (fileName: string) => {},
});

export const useModelContext = () => useContext(ModelContext);

export const ModelProvider = ({ children }: { children: ReactNode }) => {
  const [models, setModels] = useState<Model[]>([]);
  const [modelPromptTemplates, setModelPromptTemplates] = useState<
    ModelPromptTemplate[]
  >([]);

  const [architectures, setArchitectures] = useState<any[]>([]);

  const downloadModel = async (filename: string) => {
    const result = await invoke("plugin:local_models|download_model", {
      filename,
    });
    return result;
  }

  useEffect(() => {
    invoke("plugin:local_models|get_prompt_templates").then((result) => {
      console.log("Prompt Templates from plugin" + JSON.stringify(result));
      setModelPromptTemplates(result as ModelPromptTemplate[]);
    });

    invoke("plugin:local_models|get_architectures").then((result) => {
      console.log("Architectures from plugin" + JSON.stringify(result));
      setArchitectures(result as any[]);
    });

    invoke("plugin:local_models|get_models").then((result) => {
      console.log("Models from plugin" + JSON.stringify(result));
      setModels(result as Model[]);
    });
  }, []);

  return (
    <ModelContext.Provider
      value={{ models, modelPromptTemplates, architectures, downloadModel }}
    >
      {children}
    </ModelContext.Provider>
  );
};
