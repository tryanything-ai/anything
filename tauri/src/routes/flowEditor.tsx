import { useMemo, useRef } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";
import NodePanel from "../components/nodePanel";
import {
  FlowNavigationProvider,
  useFlowNavigationContext,
} from "../context/FlowNavigationProvider";
import { FlowProvider, useFlowContext } from "../context/FlowProvider";
import ManualNode from "../components/nodes/manualNode";
import NodeConfigPanel from "../components/nodeConfigPanel";
import SuperNode from "../components/nodes/superNode";

import { Allotment } from "allotment";
import "allotment/dist/style.css";

import "reactflow/dist/style.css";
import FlowName from "../components/flowName";
import RightPanel from "../components/RightPanel";

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

  const { nodePanel, nodeConfigPanel, nodeId } = useFlowNavigationContext();
  const reactFlowWrapper = useRef(null);

  const nodeTypes = useMemo(
    () => ({
      manualNode: ManualNode,
      superNode: SuperNode,
    }),
    []
  );

  return (
    <div className="h-full w-full overscroll-none">
      {/* <Header /> */}
      <div className="flex flex-row h-full w-full">
        <Allotment defaultSizes={[100, 500, 100]}>
          {/* Left Side */}
          {/* {nodePanel ? ( */}
          <Allotment.Pane preferredSize={300} maxSize={600} minSize={250}>
            <NodePanel />
          </Allotment.Pane>
          {/* ) : null} */}
          {/* Editor */}
          <Allotment.Pane preferredSize={800}>
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
                <FlowName />
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
          <Allotment.Pane preferredSize={300} maxSize={600} minSize={250}>
            <RightPanel />
          </Allotment.Pane>
         
        </Allotment>
      </div>
    </div>
  );
}

export default function FlowEditor() {
  return (
    <FlowProvider>
      <FlowNavigationProvider>
        <Flows />
      </FlowNavigationProvider>
    </FlowProvider>
  );
}
