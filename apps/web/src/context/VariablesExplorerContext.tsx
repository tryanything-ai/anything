import React, {
  createContext,
  useContext,
  useRef,
  useState,
  useCallback,
} from "react";

const VariablesExplorerContext = createContext<any>(null);

export const useVariablesExplorer = () => useContext(VariablesExplorerContext);

export interface VariablesExplorerInterface {
  ediorRefs: Map<string, React.RefObject<HTMLDivElement>>;
  // insertAtCursor: (text: string) => void;
  // setTemplateValue: (value: string) => void;
  insertAtCursor: (text: string) => void;
  registerEditorRef: (
    name: string,
    ref: React.RefObject<HTMLDivElement>,
  ) => void;
  unregisterEditorRef: (name: string) => void;
  setActiveEditor: (name: string) => void;
}

export function VariablesExplorerProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  // const editorRef = useRef<HTMLDivElement>(null); // Reference to the editable input

  // Map to store refs by name
  const [editorRefs] = useState<Map<string, React.RefObject<HTMLDivElement>>>(
    new Map(),
  );

  const registerEditorRef = useCallback(
    (name: string, ref: React.RefObject<HTMLDivElement>) => {
      editorRefs.set(name, ref);
    },
    [],
  );

  const unregisterEditorRef = useCallback((name: string) => {
    editorRefs.delete(name);
  }, []);

  // Function to access a specific editor
  const getEditorRef = useCallback((name: string) => {
    return editorRefs.get(name);
  }, []);

  const [activeEditorName, setActiveEditorName] = useState<string | null>(null);

  const setActiveEditor = useCallback((name: string) => {
    console.log("[SET_ACTIVE_EDITOR] Setting active editor to:", name);
    setActiveEditorName(name);
    console.log("[SET_ACTIVE_EDITOR] Active editor set successfully");
  }, []);

  // Simplified insertAtCursor that uses activeEditorName
  const insertAtCursor = useCallback(
    (text: string) => {
      console.log("[INSERT_AT_CURSOR] Called with text:", text);
      if (!activeEditorName) {
        console.log("[INSERT_AT_CURSOR] No active editor name, returning");
        return;
      }
      const editor = getEditorRef(activeEditorName)?.current;
      if (!editor) {
        console.log("[INSERT_AT_CURSOR] No editor ref found for:", activeEditorName);
        return;
      }

      const selection = window.getSelection();
      if (!selection?.rangeCount) {
        console.log("[INSERT_AT_CURSOR] No selection range found");
        return;
      }

      console.log("[INSERT_AT_CURSOR] Inserting text at cursor position");
      const range = selection.getRangeAt(0);
      range.deleteContents();

      const textNode = document.createTextNode(text);
      range.insertNode(textNode);

      // Move the cursor to after the inserted text
      range.setStartAfter(textNode);
      selection.removeAllRanges();
      selection.addRange(range);
      console.log("[INSERT_AT_CURSOR] Text inserted successfully");
    },
    [activeEditorName, getEditorRef],
  );

  return (
    <VariablesExplorerContext.Provider
      value={{
        registerEditorRef,
        unregisterEditorRef,
        getEditorRef,
        insertAtCursor,
        setActiveEditor,
        // templateValue,
        // setTemplateValue,
      }}
    >
      {children}
    </VariablesExplorerContext.Provider>
  );
}
