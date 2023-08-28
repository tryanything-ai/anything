import {
  Handle,
  Position
} from "reactflow";
import { AnythingNodeProps, Node} from "../../utils/nodeUtils";
import BaseNode from "./baseNode";

let node: Node = {
  nodeType: "javascriptNode",
  title: "JS Node",
  image_src: "/js-logo.svg",
  alt: "JS Logo",
  nodeData: {
    worker_type: "javascript", 
  },
  specialData: {
    code: "",
  },
};

JavascriptNode.Node = node;
//Node that acts as the beginning of a flow or one of many beginnings of a flow
export default function JavascriptNode({id,  data }: AnythingNodeProps){
  return (
    <BaseNode id={id} data={data}>
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
   </BaseNode>
  );
};

