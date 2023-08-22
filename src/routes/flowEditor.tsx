import { useMemo, useRef, useState } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls, ReactFlowActions, ReactFlowInstance } from "reactflow";
import Header from "../components/header";
import NodePanel from "../components/nodePanel";
import TomlPanel from "../components/tomlPanel";
import DebugPanel from "../components/debugPanel";
import { useNavigationContext } from "../context/NavigationProvider";
import { FlowProvider, useFlowContext } from "../context/FlowProvider";
import VectorNode from "../components/nodes/vectorNode";
import PythonNode from "../components/nodes/pythonNode";
import JavascriptNode from "../components/nodes/javascriptNode";
import SettingsPanel from "../components/settingsPanel";

import "reactflow/dist/style.css";
import CronNode from "../components/nodes/cronNode";
import TerminalNode from "../components/nodes/terminalNode";
import ModelNode from "../components/nodes/modelNode";
import ManualNode from "../components/nodes/manualNode";

function Flows() {
  const {
    nodes,
    edges,
    onConnect,
    onNodesChange,
    onEdgesChange,
    onDragOver,
    onDrop,
    setReactFlowInstance
  } = useFlowContext();
  const { nodePanel, debugPanel, tomlPanel, settingsPanel } =
    useNavigationContext();
  const reactFlowWrapper = useRef(null);

  const nodeTypes = useMemo(
    () => ({
      vectorNode: VectorNode,
      pythonNode: PythonNode,
      javascriptNode: JavascriptNode,
      cronNode: CronNode,
      terminalNode: TerminalNode,
      modelNode: ModelNode,
      manualNode: ManualNode,
    }),
    []
  );

  return (
    <div className="h-full w-full pb-5">
      <Header />
      <div className="flex flex-row h-full w-full">
        {/* <div className="flex flex-row h-full w-full" > */}
          <ReactFlow
            ref={reactFlowWrapper}
            nodeTypes={nodeTypes}
            nodes={nodes} //new
            edges={edges} //new
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
        {/* </div> */}
        {nodePanel ? (
          <div className="w-1/4">
            <NodePanel />
          </div>
        ) : null}
        {debugPanel ? (
          <div className="w-1/4">
            <DebugPanel />
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
