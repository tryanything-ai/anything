import React, {
  useCallback,
  useEffect,
  useMemo,
  useState,
  useRef,
} from "react";
import ReactFlow, {
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Handle,
  Position,
  BackgroundVariant,
  Controls,
} from "reactflow";

import "reactflow/dist/style.css";
import { useTomlFlowContext } from "../context/TomlFlowProvider";
import Header from "../components/header";
import NodePanel from "../components/nodePanel";
import { useNavigationContext } from "../context/NavigationProvider";
import TomlPanel from "../components/tomlPanel";
import ChatPanel from "../components/chatPanel";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { FlowProvider, useFlowContext } from "../context/FlowProvider";

function Flows() {
  const { nodes, edges, onConnect, onNodesChange, onEdgesChange } =
    useFlowContext();
  const { toml_nodes, toml_edges, set_toml } = useTomlFlowContext();
  const { nodePanel, chatPanel, tomlPanel } = useNavigationContext();
  // const { setCurrentFlow } = useLocalFileContext();
  //flow state
  // const [nodes, setNodes, onNodesChange] = useNodesState([]);
  // const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [tomlLoaded, setTomlLoaded] = useState(false);

  const prevNodesRef = useRef<any>();
  const prevEdgesRef = useRef<any>();

  // const nodeTypes = useMemo(
  //   () => ({
  //     inboxNode: InboxNode,
  //     vectorNode: VectorNode,
  //     llmNode: LLMNode,
  //   }),
  //   []
  // );

  // useEffect(() => {
  //   console.log("toml_nodes", toml_nodes);
  //   if (toml_nodes !== undefined) {
  //     setNodes(toml_nodes);
  //   }
  //   if (toml_edges !== undefined) {
  //     setEdges(toml_edges);
  //   }
  //   setTomlLoaded(true);
  // }, [toml_nodes, toml_edges]);

  //wysiwyg to toml
  useEffect(() => {
    const prevNodes = prevNodesRef.current;
    const prevEdges = prevEdgesRef.current;
    if (
      JSON.stringify(prevNodes) !== JSON.stringify(nodes) ||
      JSON.stringify(prevEdges) !== JSON.stringify(edges)
    ) {
      if (tomlLoaded) {
        set_toml({ nodes, edges });
      }
    }

    prevNodesRef.current = nodes;
    prevEdgesRef.current = edges;
  }, [nodes, edges]);

  // useEffect(() => {
  //   // This is where you can do something when the component is mounted

  //   return () => {
  //     // This function will be called when the component is about to be unmounted
  //     // console.log('The component is about to be unmounted');
  //     // setCurrentFlow("");
  //   };
  // }, []);

  // const onConnect = useCallback(
  //   (params: any) => setEdges((eds) => addEdge(params, eds)),
  //   [setEdges]
  // );

  return (
    <div className="h-full w-full pb-5">
      <Header />
      <div className="flex flex-row h-full w-full">
        <ReactFlow
          // nodeTypes={nodeTypes}
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
          <div className="w-1/4">
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
