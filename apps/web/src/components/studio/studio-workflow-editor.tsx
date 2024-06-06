import { useMemo, useRef } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import 'reactflow/dist/style.css';

import { FlowProvider } from "@/context/FlowProvider";
import {
    FlowNavigationProvider

} from "@/context/FlowNavigationProvider";
import BaseNode
    from "./nodes/workflow-node";

const initialNodes = [
    { id: '1', type: "anything", position: { x: 0, y: 0 }, data: { label: '1', trigger: true } },
    { id: '2', type: "anything", position: { x: 0, y: 100 }, data: { label: '2' } },
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
        <FlowProvider>
            <FlowNavigationProvider>
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
            </FlowNavigationProvider>
        </FlowProvider>

    )
}