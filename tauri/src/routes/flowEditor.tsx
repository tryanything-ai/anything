import { useMemo, useRef } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";
import Header from "../components/header";
import NodePanel from "../components/nodePanel";
import TomlPanel from "../components/tomlPanel";
import DebugPanel from "../components/debugPanel";
import { useNavigationContext } from "../context/NavigationProvider";
import { FlowProvider, useFlowContext } from "../context/FlowProvider";
import SettingsPanel from "../components/settingsPanel";
import ManualNode from "../components/nodes/manualNode";
import { useParams } from "react-router-dom";

import NodeConfigPanel from "../components/nodeConfigPanel";
import SuperNode from "../components/nodes/superNode";
import "reactflow/dist/style.css";

function Flows() {
  const {
    nodes,
    edges,
    onConnect,
    onNodesChange,
    onEdgesChange,
    onDragOver,
    onDrop,
    setReactFlowInstance,
  } = useFlowContext();

  const { nodePanel, debugPanel, tomlPanel, settingsPanel, nodeConfigPanel } =
    useNavigationContext();
  const reactFlowWrapper = useRef(null);
  const { flow_name } = useParams();

  const nodeTypes = useMemo(
    () => ({
      manualNode: ManualNode,
      superNode: SuperNode
    }),
    []
  );

  return (
    <div className="h-full w-full pb-5 overscroll-none">
      <Header />
      <div className="flex flex-row h-full w-full">
        <div className="flex flex-row h-full w-full" ref={reactFlowWrapper}>
          <ReactFlow
            nodeTypes={nodeTypes}
            nodes={nodes} 
            edges={edges} 
            onNodesChange={onNodesChange}
            onEdgesChange={onEdgesChange}
            onDragOver={onDragOver}
            onInit={setReactFlowInstance}
            onDrop={(e) => onDrop(e, reactFlowWrapper)}
            onConnect={onConnect}
            fitView
          >
            <Controls style={{ background: "darkgray" }} />
            <Background
              variant={BackgroundVariant.Dots}
              gap={30}
              size={1}
              color="gray"
            />
          </ReactFlow>
        </div>
        {nodePanel ? (
          <div className="w-1/4">
            <NodePanel />
          </div>
        ) : null}
        {debugPanel ? (
          <div className="w-1/4">
            {/* If you don't provide this key the debug pannel doesnt rerender and flow_name is stale in useParams */}
            <DebugPanel key={flow_name} />
          </div>
        ) : null}
        {settingsPanel ? (
          <div className="w-1/4">
            <SettingsPanel />
          </div>
        ) : null}
        {tomlPanel ? (
          <div className="w-1/2">
            <TomlPanel />
          </div>
        ) : null}
        {nodeConfigPanel ? (
          <div className="w-1/2">
            <NodeConfigPanel />{" "}
          </div>
        ) : null}
      </div>
    </div>
  );
}

export default function FlowEditor() {
  return (
    <FlowProvider>
      <Flows />
    </FlowProvider>
  );
}
