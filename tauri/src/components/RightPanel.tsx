import clsx from "clsx";
import { useFlowNavigationContext } from "../context/FlowNavigationProvider"; // replace with your actual path
import TomlPanel from "../components/tomlPanel";
import DebugPanel from "../components/debugPanel";
import SettingsPanel from "../components/settingsPanel";
import NodeConfigPanel from "./nodeConfigPanel";

const RightPanel = () => {
  const { debugPanel, settingsPanel, tomlPanel, nodeConfigPanel, closeAllPanelsOpenOne, nodeId } =
    useFlowNavigationContext();

  return (
    <div className="w-full">
      <div className="tabs tabs-boxed">
        <a
          className={clsx("tab", { "tab-active": debugPanel })}
          onClick={() => closeAllPanelsOpenOne("debug")}
        >
          Debug
        </a>
        <a
          className={clsx("tab", { "tab-active": nodeConfigPanel })}
          onClick={() => closeAllPanelsOpenOne("nodeConfig")}
        >
          Node Config
        </a> 
        <a
          className={clsx("tab", { "tab-active": settingsPanel })}
          onClick={() => closeAllPanelsOpenOne("settings")}
        >
          Settings
        </a>
        <a
          className={clsx("tab", { "tab-active": tomlPanel })}
          onClick={() => closeAllPanelsOpenOne("toml")}
        >
          Editor
        </a>
        {/* You can also control this one similarly */}
          </div>
           {debugPanel ? (
            // <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <DebugPanel />
            // </Allotment.Pane>
          ) : null}
          {settingsPanel ? (
            // <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <SettingsPanel />
            // </Allotment.Pane>
          ) : null}
          {tomlPanel ? (
            // <Allotment.Pane preferredSize={300} minSize={200}>
              <TomlPanel />
            // </Allotment.Pane>
          ) : null}
           {nodeConfigPanel ? (
            // <Allotment.Pane preferredSize={700} minSize={200}>
              <NodeConfigPanel key={nodeId} />
            // </Allotment.Pane>
          ) : null}
    </div>
  );
};

export default RightPanel;
