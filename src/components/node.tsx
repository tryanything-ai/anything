// import React, { useCallback, useMemo } from "react";

// import ReactFlow, { Handle, Position } from "reactflow";

// import toml from "toml";

// const Node = ({ data }: { data: any }) => {

//   const filePath = "path/to/your/file.toml";

//   // Step 1: Read the TOML data from the file
// //   const tomlData = fs.readFileSync(filePath, "utf-8");

//   // Step 2: Parse the TOML data into a JavaScript object
//   const parsedData = toml.parse(tomlData);

//   // Step 3: Find and update the node with ID "2" in the JavaScript object
//   const nodeIdToUpdate = "2";
//   const nodeToUpdate = parsedData.nodes.find(
//     (node: any) => node.id === nodeIdToUpdate
//   );
//   if (nodeToUpdate) {
//     // Modify the properties of the node here
//     nodeToUpdate.type = "updatedType";
//     nodeToUpdate.position.x = 300;
//     nodeToUpdate.position.y = 400;
//     nodeToUpdate.data.value = "Updated Value";
//   }

//   // Step 4: Serialize the updated JavaScript object back to TOML
//   const updatedTomlData = toml.stringify(parsedData);

//   // Step 5: Write the updated TOML data back to the file
//   fs.writeFileSync(filePath, updatedTomlData, "utf-8");

//   const onChange = useCallback((evt: any) => {
//     console.log(evt.target.value);
//   }, []);

//   return (
//     <div
//       className={
//         "bg-primary-200 w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
//         data.classNames
//       }
//     >
//       <Handle type="target" position={Position.Top} id="a" />
//       <div className="text-center text-xl">{data.value}</div>
//       <Handle type="target" position={Position.Right} id="b" />
//       <Handle type="source" position={Position.Bottom} id="c" />

//       {/* <Handle type="source" position={Position.Bottom} id="a" />
//         <Handle
//           type="source"
//           position={Position.Bottom}
//           id="b"
//         /> */}
//     </div>
//   );
// };

// export default Node;
