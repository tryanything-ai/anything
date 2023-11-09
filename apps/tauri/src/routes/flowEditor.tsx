import "allotment/dist/style.css";
import "reactflow/dist/style.css";

//Sliding Panels
import { Allotment } from "allotment";
import { useMemo, useRef } from "react";

//Drag and Drop Editor
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import FlowName from "../components/flowName";
import NodePanel from "../components/nodePanel";
import SuperNode from "../components/nodes/superNode";
import RightPanel from "../components/RightPanel";
import { useFlowNavigationContext } from "../context/FlowNavigationProvider";
import { useFlowContext } from "../context/FlowProvider";

export default function Flows() {
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

  const reactFlowWrapper = useRef(null);

  const nodeTypes = useMemo(
    () => ({
      superNode: SuperNode,
    }),
    []
  );

  return (
    <div className="h-full w-full overscroll-none">
      <div className="flex flex-row h-full w-full">
        <Allotment defaultSizes={[100, 500, 100]}>
          {/* Left Side */}
          <Allotment.Pane preferredSize={300} maxSize={600} minSize={250}>
            <NodePanel />
          </Allotment.Pane>
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
          <Allotment.Pane preferredSize={350} maxSize={600} minSize={300}>
            <RightPanel />
          </Allotment.Pane>
        </Allotment>
      </div>
    </div>
  );
}
