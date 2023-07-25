import React, {
  createContext,
  useState,
  useContext,
  ReactNode,
  useEffect,
} from "react";
import { useLocation, useParams } from "react-router-dom";
import { useLocalFileContext } from "./LocalFileProvider";

interface NavigationContextInterface {
  nodePanel: boolean;
  setNodePanel: (option: boolean) => void;
  tomlPanel: boolean;
  setTomlPanel: (option: boolean) => void;
  chatPanel: boolean;
  setChatPanel: (option: boolean) => void;
}

export const NavigationContext = createContext<NavigationContextInterface>({
  nodePanel: true,
  setNodePanel: () => {},
  tomlPanel: true,
  setTomlPanel: () => {},
  chatPanel: true,
  setChatPanel: () => {},
});

export const useNavigationContext = () => useContext(NavigationContext);

//TODO: keyboard shortcuts
export const NavigationProvider = ({ children }: { children: ReactNode }) => {
  const { setCurrentFlow } = useLocalFileContext();
  const location = useLocation();
  const params = useParams();
  const [nodePanel, setNodePanel] = useState<boolean>(false);
  const [tomlPanel, setTomlPanel] = useState<boolean>(false);
  const [chatPanel, setChatPanel] = useState<boolean>(false);

  useEffect(() => {
    if (params && params.id) {
      setCurrentFlow(params.id);
      // console.log("params", params);
      // if (params.editor === "toml") {
      //   setTomlPanel(true);
      // } else {
      //   setNodePanel(true);
      // }
    }
  }, [location]);

  return (
    <NavigationContext.Provider
      value={{
        nodePanel,
        setNodePanel,
        tomlPanel,
        setTomlPanel,
        chatPanel,
        setChatPanel,
      }}
    >
      {children}
    </NavigationContext.Provider>
  );
};
