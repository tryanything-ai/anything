import React, {
  createContext,
  useContext,
  useState,
  useCallback,
  useMemo,
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

export interface VariablesExplorerInterface {
  registerCallback: (id: string, callback: (variable: string) => void) => void; // now takes an ID
  unRegisterCallback: (id: string) => void;
  insertVariable: (variable: string) => void;
  cursorPosition: number;
  setCursorPosition: (position: number) => void;
  activeFieldName: string;
  setActiveFieldName: (fieldName: string) => void;
}

export function VariablesExplorerProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  // const [registeredCallback, setRegisteredCallback] = useState<
  //   ((variable: string) => void) | null
  // >(null);
  const [registeredCallbacks, setRegisteredCallbacks] = useState<Record<string, (variable: string) => void>>({});
  const [cursorPosition, setCursorPosition] = useState(0);
  const [activeFieldName, setActiveFieldName] = useState("");

  const registerCallback = useCallback((id: string, callback: (variable: string) => void) => {
    setRegisteredCallbacks(callbacks => ({
      ...callbacks,
      [id]: callback
    }));
  }, []);

  const unRegisterCallback = useCallback((id: string) => {
    setRegisteredCallbacks(callbacks => {
      const { [id]: _, ...rest } = callbacks;
      return rest;
    });
  }, []);

  const insertVariable = useCallback((variable: string) => {
    Object.values(registeredCallbacks).forEach(callback => {
      callback(variable);
    });
  }, [registeredCallbacks]);

  const handleSetCursorPosition = useCallback((position: number) => {
    console.log("[VARIABLES EXPLORER] Setting cursor position:", position);
    setCursorPosition(position);
  }, []);

  const handleSetActiveFieldName = useCallback((fieldName: string) => {
    console.log("[VARIABLES EXPLORER] Setting active field name:", fieldName);
    setActiveFieldName(fieldName);
  }, []);

  const contextValue = useMemo(
    () => ({
      registerCallback,
      unRegisterCallback,
      insertVariable,
      cursorPosition,
      setCursorPosition: handleSetCursorPosition,
      activeFieldName,
      setActiveFieldName: handleSetActiveFieldName,
    }),
    [
      registerCallback,
      unRegisterCallback,
      insertVariable,
      cursorPosition,
      handleSetCursorPosition,
      activeFieldName,
      handleSetActiveFieldName,
    ],
  );

  return (
    <VariablesExplorerContext.Provider value={contextValue}>
      {children}
    </VariablesExplorerContext.Provider>
  );
}
