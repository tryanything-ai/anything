import React, {
  useCallback,
  useEffect,
  useMemo,
  useState,
  useRef,
  ReactNode,
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

//Node that acts as the beginning of a flow or one of many beginnings of a flow
const JavascriptNode = ({ data }: any) => {
  return (
    <div
      className={
        "bg-primary w-40 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle
        type="target"
        position={Position.Top}
        id="a"
        onConnect={(params) => {
          console.log("onConnect params in JavscriptNode", params);
        }}
      />

      <div className="h-10 w-40">
        <img
          src={"/js-logo.svg"}
          alt="Javascript Logo"
          className="max-w-full max-h-full"
        />
      </div>
      <Handle
        type="source"
        position={Position.Bottom}
        id="b"
        onConnect={(params) => {
          console.log("onConnect params in JavscriptNode", params);
        }}
      />
    </div>
  );
};

export default JavascriptNode;
