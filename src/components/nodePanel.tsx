const NodePanel = () => {
  function findNextNodeId(nodes: any): string {
    // Initialize the maxId to 0
    let maxId = 0;

    console.log("nodes in FindNextNodeId", nodes);

    // Loop through the nodes and find the maximum numeric ID value
    nodes.forEach((node: any) => {
      const numericId = parseInt(node.id, 10);
      console.log("numericId", numericId);
      if (!isNaN(numericId) && numericId > maxId) {
        maxId = numericId;
      }
    });

    // Increment the maxId to get the next ID for the new node
    const nextId = (maxId + 1).toString();

    return nextId;
  }

  const addNode = () => {
    // const id = findNextNodeId(nodes);
    // const newNode = {
    //   id,
    //   position: {
    //     x: Math.random() * 500,
    //     y: Math.random() * 500,
    //   },
    //   data: {
    //     label: `Node ${id}`,
    //   },
    // };
    // reactFlowInstance.addNodes(newNode); //TODO: fix this
    console.log("addNode");
  };

  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <button onClick={addNode} className="btn btn-neutral">
        Add Node
      </button>
    </div>
  );
};

export default NodePanel;
