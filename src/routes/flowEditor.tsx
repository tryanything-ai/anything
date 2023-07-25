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

export default function Flows() {
  const { toml_nodes, toml_edges, set_toml } = useTomlFlowContext();
  const { nodePanel, chatPanel, tomlPanel } = useNavigationContext();
  //flow state
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [tomlLoaded, setTomlLoaded] = useState(false);

  const prevNodesRef = useRef<any>();
  const prevEdgesRef = useRef<any>();

  const nodeTypes = useMemo(
    () => ({
      inboxNode: InboxNode,
      vectorNode: VectorNode,
      llmNode: LLMNode,
    }),
    []
  );

  useEffect(() => {
    console.log("toml_nodes", toml_nodes);
    if (toml_nodes !== undefined) {
      setNodes(toml_nodes);
    }
    if (toml_edges !== undefined) {
      setEdges(toml_edges);
    }
    setTomlLoaded(true);
  }, [toml_nodes, toml_edges]);

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

  const onConnect = useCallback(
    (params: any) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <div className="h-full w-full pb-5">
      <Header />
      <div className="flex flex-row h-full w-full">
        <ReactFlow
          nodeTypes={nodeTypes}
          nodes={nodes}
          edges={edges}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
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

function InboxNode({ data }: { data: any }) {
  const onChange = useCallback((evt: any) => {
    console.log(evt.target.value);
  }, []);

  return (
    <div
      className={
        "bg-secondary w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <div className="text-center text-xl">{data.value}</div>

      <Handle type="source" position={Position.Bottom} id="a" />
    </div>
  );
}

function VectorNode({ data }: { data: any }) {
  const onChange = useCallback((evt: any) => {
    console.log(evt.target.value);
  }, []);

  return (
    <div
      className={
        "bg-primary-200 w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      <Handle type="target" position={Position.Right} id="b" />
      <Handle type="source" position={Position.Bottom} id="c" />
    </div>
  );
}

function LLMNode({ data }: { data: any }) {
  const onChange = useCallback((evt: any) => {
    console.log(evt.target.value);
  }, []);

  return (
    <div
      className={
        "bg-primary-200 w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      <Handle type="source" position={Position.Bottom} id="b" />
    </div>
  );
}
