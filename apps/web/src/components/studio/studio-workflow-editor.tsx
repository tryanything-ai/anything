import { useMemo, useRef } from "react";
import ReactFlow, { Background, BackgroundVariant, Controls } from "reactflow";

import 'reactflow/dist/style.css';

import { FlowProvider } from "@/context/FlowProvider";
import {
    FlowNavigationProvider

} from "@/context/FlowNavigationProvider";
const initialNodes = [
    { id: '1', position: { x: 0, y: 0 }, data: { label: '1' } },
    { id: '2', position: { x: 0, y: 100 }, data: { label: '2' } },
];

// const nodeTypes = useMemo(
//     () => ({
//       superNode: SuperNode,
//     }),
//     []
//   );


const initialEdges = [{ id: 'e1-2', source: '1', target: '2' }];
export default function StudioWorkflowEditor() {
    return (
        <FlowProvider>
            <FlowNavigationProvider>
                <div style={{ width: '100%', height: '100%' }}>
                    <ReactFlow nodes={initialNodes} edges={initialEdges} >
                        <Background
                            variant={BackgroundVariant.Dots}
                            gap={20}
                            size={1}
                            color="gray"
                        />
                        <Controls />
                    </ReactFlow>
                </div>
            </FlowNavigationProvider>
        </FlowProvider>

    )
}