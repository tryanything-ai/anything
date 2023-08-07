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
const PythonNode = ({ data }: any) => {
  return (
    <div
      className={
        "bg-primary w-42 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <img
        src={"/python-logo.svg"}
        alt="Python Logo"
        className="max-w-full max-h-full mt-2 ml-4"
      />
      <Handle type="source" position={Position.Bottom} id="b" />
    </div>
  );
};

export default PythonNode;
