import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import 'reactflow/dist/style.css';

import BaseNode
    from "./nodes/workflow-node";
import { useAnything } from "@/context/AnythingContext";

// const edgeTypes = {
//     'custom-edge': CustomEdge
//   }

  const nodeTypes = {
        "anything": BaseNode,
}; 
export default function StudioWorkflowEditor() {

    const { workflow } = useAnything();

    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ReactFlow
                nodeTypes={nodeTypes}
                nodes={workflow.nodes}
                edges={workflow.edges}
                onNodesChange={workflow.onNodesChange}
                onEdgesChange={workflow.onEdgesChange}
                onConnect={workflow.onConnect}
                // onDragOver={workflow.onDragOver}
                // onDrop={(e) => onDrop(e, reactFlowWrapper)}
                onInit={workflow.setReactFlowInstance}
            >
                <Background
                    variant={BackgroundVariant.Dots}
                    gap={20}
                    size={1}
                />
                <Controls />
            </ReactFlow>
        </div>
    )
}