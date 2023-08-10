import { useMemo } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";
import Header from "../components/header";
import NodePanel from "../components/nodePanel";
import TomlPanel from "../components/tomlPanel";
import ChatPanel from "../components/chatPanel";
import { useNavigationContext } from "../context/NavigationProvider";
import { FlowProvider, useFlowContext } from "../context/FlowProvider";
import VectorNode from "../components/nodes/vectorNode";
import PythonNode from "../components/nodes/pythonNode";
import JavascriptNode from "../components/nodes/javascriptNode";

import "reactflow/dist/style.css";

function Flows() {
  const { nodes, edges, onConnect, onNodesChange, onEdgesChange } =
    useFlowContext();
  const { nodePanel, chatPanel, tomlPanel } = useNavigationContext();

  const nodeTypes = useMemo(
    () => ({
      vectorNode: VectorNode,
      pythonNode: PythonNode,
      javascriptNode: JavascriptNode,
    }),
    []
  );

  return (
    <div className="h-full w-full pb-5">
      <Header />
      <div className="flex flex-row h-full w-full">
        <ReactFlow
          nodeTypes={nodeTypes}
          nodes={nodes} //new
          edges={edges} //new
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
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
        {nodePanel ? (
          <div className="w-1/4">
            <NodePanel />
          </div>
        ) : null}
        {chatPanel ? (
          <div className="w-1/4">
            <ChatPanel />
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
