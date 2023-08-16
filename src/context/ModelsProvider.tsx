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

interface ModelContextInterface {
  models: any[];
  architectures: any[];
  modelPromptTemplates: ModelPromptTemplate[];
}

export const ModelContext = createContext<ModelContextInterface>({
  models: [],
  architectures: [],
  modelPromptTemplates: [],
});

export const useModelContext = () => useContext(ModelContext);

export const ModelProvider = ({ children }: { children: ReactNode }) => {
  const [models, setModels] = useState<string[]>([]);
  const [modelPromptTemplates, setModelPromptTemplates] = useState<
    ModelPromptTemplate[]
  >([]);

  const [architectures, setArchitectures] = useState<any[]>([]);

  useEffect(() => {
    invoke("plugin:rustformers|get_prompt_templates").then((result) => {
      console.log("Prompt Templates from plugin" + JSON.stringify(result));
      setModelPromptTemplates(result as ModelPromptTemplate[]);
    });

    invoke("plugin:rustformers|get_architectures").then((result) => {
      console.log("Architectures from plugin" + JSON.stringify(result));
      setArchitectures(result as any[]);
    });

    invoke("plugin:rustformers|get_models").then((result) => {
      console.log("Models from plugin" + JSON.stringify(result));
      setModels(result as string[]);
    });
  }, []);

  return (
    <ModelContext.Provider
      value={{ models, modelPromptTemplates, architectures }}
    >
      {children}
    </ModelContext.Provider>
  );
};
