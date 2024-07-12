import {
    Node
} from "reactflow";

// export function findConflictFreeId(nodes: Node[], planned_node_name: string): string {
//     console.log("Finding conflict free id for", planned_node_name);
//     const nodesWithSubstring = nodes.filter((node: Node) => node.id.startsWith(planned_node_name));
//     console.log("Nodes with substring", nodesWithSubstring);
//     let suffix = nodesWithSubstring.length;
//     console.log("Suffix", suffix);
//     if (suffix === 0) {
//         console.log("No conflict found for", planned_node_name);
//         console.log("Returning", planned_node_name);
//         return planned_node_name;
//     } else {
//         let highestSuffixedNode = 0;
//         nodesWithSubstring.forEach((node: Node) => {
//             const lastChar = node.id.slice(-1);
//             const lastCharIsInt = !isNaN(parseInt(lastChar));
//             if (lastCharIsInt) {
//                 let suffix = parseInt(lastChar);
//                 if (suffix > highestSuffixedNode) {
//                     highestSuffixedNode = suffix;
//                 }
//             }
//         });
//         console.log("Highest suffixed node", highestSuffixedNode);

//         return `${planned_node_name}_${highestSuffixedNode + 1}`;
//     }
// }

export function findConflictFreeId(nodes: Node[], planned_node_name: string): string {
    console.log("Finding conflict free id for", planned_node_name);
    let suffix = 0;
    let newId = planned_node_name;

    // If the id is not used ever just use it
    if (!nodes.some((node: Node) => node.id === newId)) {
        console.log("No conflict found for", planned_node_name);
        console.log("Returning", planned_node_name);
        return newId;
    }

    // Generate a new ID with an incrementing suffix until we find one that doesn't exist
    while (nodes.some((node: Node) => node.id === newId)) {
        suffix++;
        newId = `${planned_node_name}_${suffix}`;
    }

    console.log("Returning conflict free id", newId);
    return newId;
}