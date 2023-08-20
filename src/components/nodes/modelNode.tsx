import React, { useCallback } from "react";
import ReactFlow, { Handle, Position } from "reactflow";

import { Node } from "../nodePanel";
import { useModelContext } from "../../context/ModelsProvider";

let node: Node = {
  nodeType: "modelNode",
  title: "Model Node",
  alt: "Model Node",
  nodeData: {
    worker_type: "local_model", 
  },
  specialData: {
    filename: "",
    prompt: "",
    variables: [],
  },
};

ModelNode.Node = node;

export default function ModelNode({ data }: { data: any }) {
  const { callModel } = useModelContext();

  // const onChange = useCallback((evt: any) => {
  //   console.log(evt.target.value);
  // }, []);

  return (
    <div
      className={
        "bg-primary w-40 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle" +
        data.classNames
      }
    >
      <Handle type="target" position={Position.Top} id="a" />
      <div className="text-center text-xl">Local Model</div>
      {/* <button
        className="btn btn-secondary"
        onClick={() =>
          callModel("Tell me to have a wonderful day in a random language! ( then describe in english )")
        }
      >
        Call
      </button> */}
      <Handle type="target" position={Position.Right} id="b" />
      <Handle type="source" position={Position.Bottom} id="c" />
    </div>
  );
}
