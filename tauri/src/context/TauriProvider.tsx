import React, { useState, useEffect, useContext, ReactNode } from "react";
import * as tauriPath from "@tauri-apps/api/path";
import * as fs from "@tauri-apps/api/fs";
import * as os from "@tauri-apps/api/os";
import tauriConfJson from "../../src-tauri/tauri.conf.json";

declare global {
  interface Window {
    __TAURI__?: any;
  }
}

export const APP_NAME = tauriConfJson.package.productName;
export const RUNNING_IN_TAURI = window.__TAURI__ !== undefined;

// NOTE: Add cacheable Tauri calls in this file
interface TauriContextInterface {
  loading: boolean;
  downloads: string;
  documents: string;
  appDocuments: string;
  osType: string;
  fileSep: string;
  currentVault: string,
}

const TauriContext = React.createContext<TauriContextInterface>({
  loading: true,
  downloads: "",
  documents: "",
  appDocuments: "",
  osType: "",
  fileSep: "/",
  currentVault: "",
});

export const useTauriContext = () => useContext(TauriContext);

export function TauriProvider({ children }: { children: ReactNode }) {
  const [loading, setLoading] = useState<boolean>(true);
  const [downloads, setDownloadDir] = useState<string>("");
  const [documents, setDocumentDir] = useState<string>("");
  const [osType, setOsType] = useState<string>("");
  const [fileSep, setFileSep] = useState<string>("/");
  const [appDocuments, setAppDocuments] = useState<string>("");
  //TODO: implement Vualts ( but after backend is done in ) 
  const [currentVault, setCurrentVault] = useState<string>("");

  //TODO: fix why this is running twice on startup
  useEffect(() => {
    if (RUNNING_IN_TAURI) {
      const callTauriAPIs = async () => {
        setDownloadDir(await tauriPath.downloadDir());
        const _documents = await tauriPath.documentDir();
        console.log("documents", _documents);
        setDocumentDir(_documents);
        const _osType = await os.type();
        setOsType(_osType);
        const _fileSep = _osType === "Windows_NT" ? "\\" : "/";
        setFileSep(_fileSep);
        //Create App Folder
        await fs.createDir(APP_NAME, {
          dir: fs.BaseDirectory.Document,
          recursive: true,
        });
        //Create Flows Folder
        await fs.createDir(APP_NAME + "/flows", {
          dir: fs.BaseDirectory.Document,
          recursive: true,
        });
        //Create Nodes Folder
        await fs.createDir(APP_NAME + "/nodes", {
          dir: fs.BaseDirectory.Document,
          recursive: true,
        });
        //Create Settings Folder
        await fs.createDir(APP_NAME + "/settings", {
          dir: fs.BaseDirectory.Document,
          recursive: true,
        });
        //Create Assets Folder
        await fs.createDir(APP_NAME + "/assets", {
          dir: fs.BaseDirectory.Document,
          recursive: true,
        });
        setAppDocuments(`${_documents}${APP_NAME}`);
        console.log("appDocuments", `${_documents}${APP_NAME}`);
        setLoading(false);
      };
      callTauriAPIs().catch(console.error);
    }
  }, []);

  return (
    <TauriContext.Provider
      value={{ loading, fileSep, downloads, documents, osType, appDocuments, currentVault }}
    >
      {children}
    </TauriContext.Provider>
  );
}
