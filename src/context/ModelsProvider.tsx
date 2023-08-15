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
  modelPromptTemplates: ModelPromptTemplate[];
}
  
  export const ModelContext = createContext<ModelContextInterface>({
    models: [],
    modelPromptTemplates: [],
  });
  
  export const useModelContext = () => useContext(ModelContext);
  
  export const ModelProvider = ({ children }: { children: ReactNode }) => {
    const [models, setModels] = useState<string[]>([]);
    const [modelPromptTemplates, setModelPromptTemplates] = useState<ModelPromptTemplate[]>([]);   
      
    useEffect(() => {
      invoke("plugin:rustformers|get_prompt_templates").then((result) => {
        console.log("Prompt Templates from plugin" +JSON.stringify(result));
        setModelPromptTemplates(result as ModelPromptTemplate[]);
      });
    }, []);
      
    return (
      <ModelContext.Provider value={{ models, modelPromptTemplates }}>
        {children}
      </ModelContext.Provider>
    );
  };
  