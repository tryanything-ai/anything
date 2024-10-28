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
  registerEditorRef: (
    name: string,
    ref: React.RefObject<HTMLDivElement>,
  ) => void;
  unregisterEditorRef: (name: string) => void;
}

export function VariablesExplorerProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const editorRef = useRef<HTMLDivElement>(null); // Reference to the editable input

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

  // // Function to insert text at the cursor position
  // const insertAtCursor = (text: string) => {
  //   const editor = editorRef.current;
  //   if (!editor) return;

  //   const selection = window.getSelection();
  //   if (!selection?.rangeCount) return;

  //   const range = selection.getRangeAt(0);
  //   range.deleteContents();

  //   const textNode = document.createTextNode(text);
  //   range.insertNode(textNode);

  //   // Move the cursor to after the inserted text
  //   range.setStartAfter(textNode);
  //   selection.removeAllRanges();
  //   selection.addRange(range);

  //   // Update the template value state
  //   setTemplateValue(editor.innerText);
  // };

  return (
    <VariablesExplorerContext.Provider
      value={{
        registerEditorRef,
        unregisterEditorRef,
        getEditorRef,
        // insertAtCursor,
        // templateValue,
        // setTemplateValue,
      }}
    >
      {children}
    </VariablesExplorerContext.Provider>
  );
}
