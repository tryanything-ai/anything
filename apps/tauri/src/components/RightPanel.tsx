import clsx from "clsx";

import { useFlowNavigationContext } from "../context/FlowNavigationProvider"; // replace with your actual path
import DebugPanel from "./debugPanel";
import NodeConfigPanel from "./nodeConfigPanel";
import SettingsPanel from "./settingsPanel";
import TomlPanel from "./tomlPanel";
import SharingPanel from "./sharingPanel";

const RightPanel = () => {
  const {
    debugPanel,
    settingsPanel,
    tomlPanel,
    sharingPanel,
    nodeConfigPanel,
    closeAllPanelsOpenOne,
    nodeId,
  } = useFlowNavigationContext();

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
          className={clsx("tab", { "tab-active": sharingPanel })}
          onClick={() => closeAllPanelsOpenOne("sharing")}
        >
          Sharing
        </a>
        {/* <a
          className={clsx("tab", { "tab-active": tomlPanel })}
          onClick={() => closeAllPanelsOpenOne("toml")}
        >
          Editor
        </a> */}
        {/* You can also control this one similarly */}
      </div>
      {debugPanel ? <DebugPanel /> : null}
      {settingsPanel ? <SettingsPanel /> : null}
      {/* {tomlPanel ? <TomlPanel /> : null} */}
      {sharingPanel ? <SharingPanel /> : null}
      {nodeConfigPanel ? <NodeConfigPanel key={nodeId} /> : null}
    </div>
  );
};

export default RightPanel;
