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

import { Allotment, LayoutPriority } from "allotment";
import "allotment/dist/style.css";

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
    currentProcessingStatus,
  } = useFlowContext();

  const {
    nodePanel,
    debugPanel,
    tomlPanel,
    settingsPanel,
    nodeConfigPanel,
    nodeId,
  } = useNavigationContext();
  const reactFlowWrapper = useRef(null);
  const { flow_name } = useParams();

  const nodeTypes = useMemo(
    () => ({
      manualNode: ManualNode,
      superNode: SuperNode,
    }),
    []
  );

  return (
    <div className="h-full w-full pb-5 overscroll-none">
      <Header />
      <div className="flex flex-row h-full w-full">
        <Allotment proportionalLayout={false}>
          {/* Left Side */}
          {nodePanel ? (
            <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <NodePanel />
            </Allotment.Pane>
          ) : null}
          {/* Editor */}
          <Allotment.Pane priority={LayoutPriority.High}>
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
          </Allotment.Pane>
          {/* Right Side */}
          {debugPanel ? (
            <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <DebugPanel key={flow_name} />
            </Allotment.Pane>
          ) : null}
          {settingsPanel ? (
            <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <SettingsPanel />
            </Allotment.Pane>
          ) : null}
          {tomlPanel ? (
            <Allotment.Pane preferredSize={300} minSize={200}>
              <TomlPanel />
            </Allotment.Pane>
          ) : null}
          {nodeConfigPanel ? (
            <Allotment.Pane preferredSize={300} maxSize={600} minSize={200}>
              <NodeConfigPanel key={nodeId} />
            </Allotment.Pane>
          ) : null}
        </Allotment>
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
