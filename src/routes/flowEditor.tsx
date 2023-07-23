import React, {
  useCallback,
  useEffect,
  useMemo,
  useState,
  useRef,
} from "react";
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
  ReactFlowProvider,
  useReactFlow,
} from "reactflow";

import "reactflow/dist/style.css";
import { useTomlFlowContext } from "../context/TomlFlowProvider";
import Header from "../components/header";
import SidePanel from "../components/sidePanel";
import { useNavigationContext } from "../context/NavigationProvider";

function findNextNodeId(nodes: any): string {
  // Initialize the maxId to 0
  let maxId = 0;

  console.log("nodes in FindNextNodeId", nodes);

  // Loop through the nodes and find the maximum numeric ID value
  nodes.forEach((node: any) => {
    const numericId = parseInt(node.id, 10);
    console.log("numericId", numericId);
    if (!isNaN(numericId) && numericId > maxId) {
      maxId = numericId;
    }
  });

  // Increment the maxId to get the next ID for the new node
  const nextId = (maxId + 1).toString();

  return nextId;
}

export default function Flows() {
  const { toml_nodes, toml_edges, set_toml } = useTomlFlowContext();
  const { sidePanel } = useNavigationContext();
  //flow state
  const reactFlowInstance = useReactFlow();
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);
  const [tomlLoaded, setTomlLoaded] = useState(false);

  const prevNodesRef = useRef<any>(); // To store the previous nodes
  const prevEdgesRef = useRef<any>(); // To store the previous edges

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

  const onClick = () => {
    const id = findNextNodeId(nodes);
    const newNode = {
      id,
      position: {
        x: Math.random() * 500,
        y: Math.random() * 500,
      },
      data: {
        label: `Node ${id}`,
      },
    };
    reactFlowInstance.addNodes(newNode);
  };

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
    // At the time this effect runs, our ref now points to the old nodes value
    const prevNodes = prevNodesRef.current;
    const prevEdges = prevEdgesRef.current;

    if (
      JSON.stringify(prevNodes) !== JSON.stringify(nodes) ||
      JSON.stringify(prevEdges) !== JSON.stringify(edges)
    ) {
      // Simple comparison - consider a deep equality check for complex state
      // If nodes are different, write new state to the file
      if (tomlLoaded) {
        //but only if the nodes are loaded. First hit is empty nodes..
        // if (JSON.stringify(nodes) !== JSON.stringify(toml_nodes)) {
        //and not same as file
        set_toml({ nodes, edges });
        // }
      }
    }

    // After our comparison, update the old value to the current one for the next effect run
    prevNodesRef.current = nodes;
    prevEdgesRef.current = edges;
  }, [nodes, edges]); // Re-run this effect every time nodes changes

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
          <CustomControls />
          <Background
            variant={BackgroundVariant.Dots}
            gap={30}
            size={1}
            color="gray"
          />
        </ReactFlow>
        <button className="absolute top-10" onClick={onClick}>
          Add node
        </button>
        {sidePanel ? (
          <div className="w-72">
            <SidePanel />
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
