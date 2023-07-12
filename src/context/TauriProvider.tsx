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
  downloads: string | undefined;
  documents: string | undefined;
  appDocuments: string | undefined;
  osType: string | undefined;
  fileSep: string;
}

const TauriContext = React.createContext<TauriContextInterface | undefined>(
  undefined
);

export const useTauriContext = () => useContext(TauriContext);

export function TauriProvider({ children }: { children: ReactNode }) {
  const [loading, setLoading] = useState<boolean>(true);
  const [downloads, setDownloadDir] = useState<string | undefined>();
  const [documents, setDocumentDir] = useState<string | undefined>();
  const [osType, setOsType] = useState<string | undefined>();
  const [fileSep, setFileSep] = useState<string>("/");
  const [appDocuments, setAppDocuments] = useState<string | undefined>();

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
        await fs.createDir(APP_NAME, {
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
      value={{ loading, fileSep, downloads, documents, osType, appDocuments }}
    >
      {children}
    </TauriContext.Provider>
  );
}
