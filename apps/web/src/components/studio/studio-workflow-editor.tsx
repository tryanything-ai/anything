import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import "reactflow/dist/style.css";

import BaseNode from "./nodes/workflow-node";
import { useAnything } from "@/context/AnythingContext";
import CustomEdge from "./edges/workflow-edge";
import { Button } from "@repo/ui/components/ui/button";

const nodeTypes = {
  anything: BaseNode,
};

const edgeTypes = {
  anything: CustomEdge,
};

export default function StudioWorkflowEditor() {
  const { workflow } = useAnything();

  return (
    <div style={{ width: "100%", height: "100%" }}>
      <ReactFlow
        nodeTypes={nodeTypes}
        edgeTypes={edgeTypes}
        nodes={workflow.nodes}
        edges={workflow.edges}
        onNodesChange={workflow.onNodesChange}
        onEdgesChange={workflow.onEdgesChange}
        onConnect={workflow.onConnect}
        nodeDragThreshold={1}
        // onDragOver={workflow.onDragOver}
        // onDrop={(e) => onDrop(e, reactFlowWrapper)}
        onInit={workflow.setReactFlowInstance}
      >
        <Background variant={BackgroundVariant.Dots} gap={20} size={1} />

        <Controls />
        <Button
          style={{
            position: "absolute",
            bottom: "15px",
            left: "55px",
            margin: 0,
            zIndex: "100",
            cursor: "pointer",
          }}
          onClick={workflow.showActionSheet}
          className=""
        >
          Add Node
        </Button>
      </ReactFlow>
    </div>
  );
}
