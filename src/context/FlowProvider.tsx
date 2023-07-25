import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import {
  Connection,
  Edge,
  EdgeChange,
  Node,
  NodeChange,
  addEdge,
  OnNodesChange,
  OnEdgesChange,
  OnConnect,
  applyNodeChanges,
  applyEdgeChanges,
} from "reactflow";

interface FlowContextInterface {
  nodes: Node[];
  edges: Edge[];
  onNodesChange: OnNodesChange;
  onEdgesChange: OnEdgesChange;
  onConnect: OnConnect;
}

export const FlowContext = createContext<FlowContextInterface>({
  nodes: [],
  edges: [],
  onNodesChange: () => {},
  onEdgesChange: () => {},
  onConnect: () => {},
});

const initalNodes: Node[] = [
  {
    id: "1",
    // type: "colorChooser",
    data: { color: "#4FD1C5" },
    position: { x: 250, y: 25 },
  },

  {
    id: "2",
    // type: "colorChooser",
    data: { color: "#F6E05E" },
    position: { x: 100, y: 125 },
  },
  {
    id: "3",
    // type: "colorChooser",
    data: { color: "#B794F4" },
    position: { x: 250, y: 250 },
  },
];

const initialEdges: Edge[] = [
  { id: "e1-2", source: "1", target: "2" },
  { id: "e2-3", source: "2", target: "3" },
];

export const useFlowContext = () => useContext(FlowContext);

export const FlowProvider = ({ children }: { children: ReactNode }) => {
  const [nodes, setNodes] = useState<Node[]>(initalNodes);
  const [edges, setEdges] = useState<Edge[]>(initialEdges);

  const onNodesChange: OnNodesChange = (nodeChanges: any) => {
    console.log("nodeChanges", nodeChanges);
    //TODO: write TOML
    setNodes((nodes) => applyNodeChanges(nodeChanges, nodes));
  };

  //When the edge is changed
  const onEdgesChange: OnEdgesChange = (edgeChanges: any) => {
    console.log("edgeChanges", edgeChanges);
    setEdges((edges) => applyEdgeChanges(edgeChanges, edges));
  };

  //When a node is connected to an edge etc
  const onConnect: OnConnect = (params: any) => {
    console.log("params on Connect", params);
    setEdges((edges) => addEdge(params, edges));
  };

  return (
    <FlowContext.Provider
      value={{ nodes, edges, onConnect, onNodesChange, onEdgesChange }}
    >
      {children}
    </FlowContext.Provider>
  );
};
