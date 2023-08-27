import React, { useCallback } from "react";
import { Handle, Position, NodeProps } from "reactflow";

import { Node } from "../nodePanel";
import { useModelContext } from "../../context/ModelsProvider";
import BaseNode from "./baseNode";

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

type NodeData = {
  value: number;
};

export default function ModelNode({ id }: NodeProps<NodeData>) {
  const { callModel } = useModelContext();

  // const onChange = useCallback((evt: any) => {
  //   console.log(evt.target.value);
  // }, []);

  return (
    <BaseNode id={id} flow_id="flow_id">
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
    </BaseNode>
  );
}
