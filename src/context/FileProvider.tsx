import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { invoke } from "@tauri-apps/api/tauri";
import { useTauriContext } from "./TauriProvider";

interface LocalFileContextInterface {
  theme: string;
  setTheme: (theme: string) => void;
}

export const LocalFileContext = createContext<LocalFileContextInterface>({
  theme: localStorage.getItem("theme") || "dark",
  setTheme: () => {},
});

export const useLocalFileContext = () => useContext(LocalFileContext);

export const LocalFileProvider = ({ children }: { children: ReactNode }) => {
  const { documents } = useTauriContext();
  // Define the path to the directory you want to monitor
  // const directoryPath = "";

  // Define the event handler for file changes
  const handleFileChange = (filePath: string) => {
    console.log(`File changed: ${filePath}`);
    // Perform any necessary actions based on the file change
  };

  // Start monitoring the directory for changes
  invoke("tauri.fs.watch", { documents })
    .then(() => {
      console.log(`Started monitoring directory: ${documents}`);
    })
    .catch((error) => {
      console.error("Error starting filesystem monitoring:", error);
    });

  // Listen for file change events
  tauri.promisified.listen("tauri.fs.file_changed", ({ filePath }) => {
    handleFileChange(filePath);
  });

  return (
    <LocalFileContext.Provider value={{ theme, setTheme }}>
      {children}
    </LocalFileContext.Provider>
  );
};
