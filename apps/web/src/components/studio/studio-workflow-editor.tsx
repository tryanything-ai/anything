import { useMemo } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import 'reactflow/dist/style.css';

import BaseNode
    from "./nodes/workflow-node";

const initialNodes = [
    { id: '1', type: "anything", position: { x: 0, y: 0 }, },
    { id: '2', type: "anything", position: { x: 0, y: 100 },  },
];

const initialEdges = [{ id: 'e1-2', source: '1', target: '2' }];

export default function StudioWorkflowEditor() {

    const nodeTypes = useMemo(
        () => ({
            anything: BaseNode,
        }),
        []
    );

    return (
        <div style={{ width: '100%', height: '100%' }}>
            <ReactFlow
                nodeTypes={nodeTypes}
                nodes={initialNodes}
                edges={initialEdges}
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