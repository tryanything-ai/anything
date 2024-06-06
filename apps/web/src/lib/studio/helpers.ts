import {
    Node
} from "reactflow";

export function findConflictFreeId(nodes: Node[], planned_node_name: string): string {
    const nodesWithSubstring = nodes.filter((node: Node) => node.id.startsWith(planned_node_name));
    let suffix = nodesWithSubstring.length;
    if (suffix === 0) {
        return planned_node_name;
    } else {
        let highestSuffixedNode = 0;
        nodesWithSubstring.forEach((node: Node) => {
            const lastChar = node.id.slice(-1);
            const lastCharIsInt = !isNaN(parseInt(lastChar));
            if (lastCharIsInt) {
                let suffix = parseInt(lastChar);
                if (suffix > highestSuffixedNode) {
                    highestSuffixedNode = suffix;
                }
            }
        });

        return `${planned_node_name}_${highestSuffixedNode + 1}`;
    }
}