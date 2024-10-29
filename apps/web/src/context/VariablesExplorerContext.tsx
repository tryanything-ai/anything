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
  registerCallback: (callback: (variable: string) => void) => void;
  unRegisterCallback: () => void;
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
  const [registeredCallback, setRegisteredCallback] = useState<
    ((variable: string) => void) | null
  >(null);
  const [cursorPosition, setCursorPosition] = useState(0);
  const [activeFieldName, setActiveFieldName] = useState("");

  const registerCallback = useCallback(
    (callback: (variable: string) => void) => {
      setRegisteredCallback(() => callback);
    },
    [],
  );

  const unRegisterCallback = useCallback(() => {
    setRegisteredCallback(null);
  }, []);

  const insertVariable = useCallback(
    (variable: string) => {
      if (registeredCallback) {
        registeredCallback(variable);
      }
    },
    [registeredCallback],
  );

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
