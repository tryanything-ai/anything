import React, { useCallback, useEffect, useMemo } from "react";
import CustomControls from "../components/customControls";
import ReactFlow, {
  MiniMap,
  Background,
  useNodesState,
  useEdgesState,
  addEdge,
  Handle,
  Position,
  BackgroundVariant,
} from "reactflow";

import "reactflow/dist/style.css";
import { useTomlFlowContext } from "../context/TomlFlowProvider";

const initialNodes: any = [
  // {
  //   id: "1",
  //   type: "inboxNode",
  //   position: { x: 100, y: 50 },
  //   data: { value: "B2B Leads ✉️" },
  // },
  // {
  //   id: "2",
  //   position: { x: 200, y: 300 },
  //   type: "vectorNode",
  //   data: { value: "Memory VectorStore" },
  // },
  // {
  //   id: "3",
  //   position: { x: 600, y: 200 },
  //   type: "pushNode",
  //   data: { value: "Gmail Inbox" },
  // },
  // {
  //   id: "4",
  //   position: { x: 600, y: 400 },
  //   type: "polyNode",
  //   data: { value: "Obsidian Notes" },
  // },
  // {
  //   id: "5",
  //   position: { x: 200, y: 550 },
  //   type: "llmNode",
  //   data: { value: "OpenAI GPT LLM" },
  // },
  // {
  //   id: "6",
  //   type: "outboxNode",
  //   position: { x: 100, y: 700 },
  //   data: { value: "Outbound Sales ⌲" },
  // },
];

const initialEdges: any = [
  // { id: "1", source: "1", target: "2", sourceHandle: "b", targetHandle: "a" },
  // {
  //   id: "2",
  //   source: "3",
  //   target: "2",
  //   sourceHandle: "a",
  //   targetHandle: "b",
  //   animated: true,
  // },
  // {
  //   id: "3",
  //   source: "4",
  //   target: "2",
  //   sourceHandle: "a",
  //   targetHandle: "b",
  //   animated: true,
  // },
  // { id: "4", source: "2", target: "5", sourceHandle: "c", targetHandle: "a" },
  // { id: "5", source: "5", target: "6", sourceHandle: "b", targetHandle: "a" },
];

export default function Flows() {
  const { toml_nodes } = useTomlFlowContext();
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const nodeTypes = useMemo(
    () => ({
      inboxNode: InboxNode,
      outboxNode: OutBoxNode,
      vectorNode: VectorNode,
      llmNode: LLMNode,
      pushNode: PushNode,
      polyNode: PolyNode,
    }),
    []
  );

  useEffect(() => {
    console.log("toml_nodes", toml_nodes);
    if (toml_nodes !== undefined) {
      setNodes(toml_nodes);
    }
  }, [toml_nodes]);

  const _onNodesChange = useCallback(
    (node: any) => {
      console.log("node changed", node);
      onNodesChange(node);
    },
    [setNodes]
  );

  const onConnect = useCallback(
    (params: any) => setEdges((eds) => addEdge(params, eds)),
    [setEdges]
  );

  return (
    <div style={{ width: "96vw", height: "100vh" }}>
      {/* <button className="btn absolute bg-pink-200 z-10">Edit Text</button> */}
      <ReactFlow
        nodeTypes={nodeTypes}
        nodes={nodes}
        edges={edges}
        onNodesChange={_onNodesChange}
        onEdgesChange={onEdgesChange}
        onConnect={onConnect}
      >
        <CustomControls />
        {/* <Controls style={{ backgroundColor: "darkgray" }} /> */}
        <MiniMap style={{ backgroundColor: "darkgray" }} />
        <Background
          variant={BackgroundVariant.Dots}
          gap={30}
          size={1}
          color="gray"
        />
      </ReactFlow>
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
      {/* <Handle type="target" position={Position.Top} /> */}
      <div className="text-center text-xl">{data.value}</div>
      {/* <div>
        <value htmlFor="text">Text:</label>
        <input id="text" name="text" onChange={onChange} className="nodrag" />
      </div> */}
      {/* <Handle type="source" position={Position.Bottom} id="a" /> */}
      <Handle
        type="source"
        position={Position.Bottom}
        id="a"
        // style={handleStyle}
      />
    </div>
  );
}

function OutBoxNode({ data }: { data: any }) {
  const onChange = useCallback((evt: any) => {
    console.log(evt.target.value);
  }, []);

  return (
    <div
      className={
        "bg-purple-700 w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      {/* <div>
        <label htmlFor="text">Text:</label>
        <input id="text" name="text" onChange={onChange} className="nodrag" />
      </div> */}
      {/* <Handle type="source" position={Position.Bottom} id="a" />
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
        // style={handleStyle}
      /> */}
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

      {/* <Handle type="source" position={Position.Bottom} id="a" />
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
      /> */}
    </div>
  );
}

function PushNode({ data }: { data: any }) {
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
      <Handle type="source" position={Position.Left} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      {/* <div>
        <label htmlFor="text">Text:</label>
        <input id="text" name="text" onChange={onChange} className="nodrag" />
      </div> */}
      {/* <Handle type="source" position={Position.Bottom} id="a" />
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
        // style={handleStyle}
      /> */}
    </div>
  );
}

function PolyNode({ data }: { data: any }) {
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
      <Handle type="source" position={Position.Left} id="a" />
      <div className="text-center text-xl">{data.value}</div>
      {/* <div>
        <label htmlFor="text">Text:</label>
        <input id="text" name="text" onChange={onChange} className="nodrag" />
      </div> */}
      {/* <Handle type="source" position={Position.Bottom} id="a" />
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
        // style={handleStyle}
      /> */}
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
      {/* <Handle type="target" position={Position.Bottom} id="c" /> */}

      {/* <Handle type="source" position={Position.Bottom} id="a" />
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
      /> */}
    </div>
  );
}
