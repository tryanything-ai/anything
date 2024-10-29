import React, {
  createContext,
  useContext,
  useRef,
  useState,
  useCallback,
} from "react";

const VariablesExplorerContext = createContext<any>(null);

export const useVariablesExplorer = () => {
  const context = useContext(VariablesExplorerContext);
  if (!context) {
    throw new Error(
      "useVariablesExplorer must be used within a VariablesExplorerProvider",
    );
  }
  console.log("[VARIABLES EXPLORER CONTEXT] Using variables explorer context");
  return context;
};

interface EditorInfo {
  ref: React.RefObject<
    HTMLTextAreaElement | { session: { history: History } } | null
  >;
  getValue: () => string;
  setValue: (value: string) => void;
}

export interface VariablesExplorerInterface {
  ediorRefs: Map<string, React.RefObject<HTMLTextAreaElement>>;
  insertAtCursor: (text: string) => void;
  registerEditor: (name: string, editorInfo: EditorInfo) => void;
  unregisterEditor: (name: string) => void;
  setActiveEditor: (name: string) => void;
}

export function VariablesExplorerProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  console.log("[VARIABLES EXPLORER CONTEXT] Initializing provider");

  const [editorMap] = useState<Map<string, EditorInfo>>(new Map());
  const registerEditor = useCallback(
    (name: string, editorInfo: EditorInfo) => {
      console.log("[VARIABLES EXPLORER CONTEXT] Registering editor:", name);
      editorMap.set(name, editorInfo);
    },
    [editorMap],
  );

  const unregisterEditor = useCallback(
    (name: string) => {
      console.log("[VARIABLES EXPLORER CONTEXT] Unregistering editor:", name);
      editorMap.delete(name);
    },
    [editorMap],
  );

  const getEditorInfo = useCallback(
    (name: string) => {
      console.log(
        "[VARIABLES EXPLORER CONTEXT] Getting editor info for:",
        name,
      );
      return editorMap.get(name);
    },
    [editorMap],
  );

  const [activeEditorName, setActiveEditorName] = useState<string | null>(null);

  const setActiveEditor = useCallback((name: string) => {
    console.log("[VARIABLES EXPLORER CONTEXT] Setting active editor to:", name);
    setActiveEditorName(name);
    console.log("[VARIABLES EXPLORER CONTEXT] Active editor set successfully");
  }, []);

  const insertAtCursor = useCallback(
    (text: string) => {
      console.log(
        "[VARIABLES EXPLORER CONTEXT] Insert at cursor called with text:",
        text,
      );
      if (!activeEditorName) {
        console.log(
          "[VARIABLES EXPLORER CONTEXT] No active editor name, returning",
        );
        return;
      }
      const editorInfo = getEditorInfo(activeEditorName);
      if (!editorInfo) {
        console.log(
          "[VARIABLES EXPLORER CONTEXT] No editor info found for:",
          activeEditorName,
        );
        return;
      }
      const textarea = editorInfo.ref.current;
      const { getValue, setValue } = editorInfo;

      if (!textarea) {
        console.log(
          "[VARIABLES EXPLORER CONTEXT] No element found for:",
          activeEditorName,
        );
        return;
      }
      // If it's a textarea element
      if (textarea instanceof HTMLTextAreaElement) {
        const value = getValue();
        const startPos = textarea.selectionStart ?? 0;
        const endPos = textarea.selectionEnd ?? 0;

        const newValue =
          value.substring(0, startPos) + text + value.substring(endPos);
        setValue(newValue);

        setTimeout(() => {
          if (textarea) {
            const newCursorPos = startPos + text.length;
            textarea.selectionStart = textarea.selectionEnd = newCursorPos;
            textarea.focus();
          }
        }, 0);
      }
      // If it's the editor component
      else {
        const value = getValue();
        setValue(value + text); // For now, just append the text since we can't easily access cursor position
      }

      console.log("[VARIABLES EXPLORER CONTEXT] Text inserted successfully");
    },
    [activeEditorName, getEditorInfo],
  );

  console.log("[VARIABLES EXPLORER CONTEXT] Rendering provider");
  return (
    <VariablesExplorerContext.Provider
      value={{
        registerEditor,
        unregisterEditor,
        insertAtCursor,
        setActiveEditor,
      }}
    >
      {children}
    </VariablesExplorerContext.Provider>
  );
}
