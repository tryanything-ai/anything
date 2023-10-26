import {
  createContext,
  ReactNode,
  useContext,
  useState,
} from "react";

// import api from "../tauri_api/api";

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

  const downloadModel = async (filename: string) => {};
  // const downloadModel = async (filename: string) => {
  //   const result = await api.downloadModel({
  //     filename,
  //   });
  //   return result;
  // };
  const callModel = async (prompt: string) => {}; 
  // const callModel = async (prompt: string) => {
  //   const result = await api.callModel({prompt});
  //   return result;
  // };  

  // const getModelData = async () => {
  //   const promptTemplates = await api.getPromptTemplates();
  //   setModelPromptTemplates(promptTemplates as ModelPromptTemplate[]);

  //   const architectures = await api.getArchitectures();
  //   setArchitectures(architectures as any[]);

  //   const models = await api.getModels();
  //   setModels(models as Model[]);

  //   const downloadedModels = await api.getDownloadedModels();
  //   setDownloadedModels(downloadedModels as Model[]);
  // }

  // useEffect(() => {
  //   getModelData(); 
  // }, []);

  return (
    <ModelContext.Provider
      value={{
        models,
        downloadedModels,
        modelPromptTemplates,
        architectures,
        downloadModel,
        callModel
      }}
    >
      {children}
    </ModelContext.Provider>
  );
};
