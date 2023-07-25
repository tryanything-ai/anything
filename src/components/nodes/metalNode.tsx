import React, { useCallback, useEffect } from "react";
import ReactFlow, { Handle, Position, OnNodesChange } from "reactflow";
import { stringify, parse } from "iarna-toml-esm";

//Node that acts as the beginning of a flow or one of many beginnings of a flow
function MetalNode({ data }: { data: any }) {
  const readNodeStateFromToml = () => {};

  const watchNodeStateFromToml = () => {};

  const updateNodeStateToToml = () => {};

  // const onNodesChange: OnNodesChange = useCallback(
  //   (nodes) => {
  //     console.log("nodes", nodes);
  //     // setNodes(nodes);
  //     // setTomlNodes(nodes);
  //     // setTomlEdges(edges);

  // async function updateNode(id: string, newNodeData: any) {
  //   const filePath = '/path/to/your/file.toml';

  //   // Read the TOML file
  //   const fileContent = fs.readFileSync(filePath, 'utf-8');

  //   // Parse the TOML file
  //   let data = toml.parse(fileContent);

  //   // Find and update the node
  //   for (let node of data.nodes) {
  //     if (node.id === id) {
  //       // Merge existing node data with new data
  //       Object.assign(node, newNodeData);
  //       break;
  //     }
  //   }

  //   // Write the updated data back to the file
  //   const newContent = toml.stringify(data);
  //   fs.writeFileSync(filePath, newContent);
  // }

  //Needs to fetch or receivein some way the data to be worked on.
  //Base case. Receive a message.
  //It should also give some context of whats going on
  //IDEA:: slow mode?
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

export default MetalNode;
