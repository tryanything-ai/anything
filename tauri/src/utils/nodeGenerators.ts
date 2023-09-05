import { Node } from "./nodeUtils";
import { HandleProps, Position } from "reactflow";

const BaseHandles: HandleProps[] = [
  {
    id: "a",
    position: Position.Top,
    type: "target",
  },
  {
    id: "b",
    position: Position.Bottom,
    type: "source",
  },
];

const StartHandles: HandleProps[] = [
  {
    id: "a",
    position: Position.Bottom,
    type: "source",
  },
];

const EndHandles: HandleProps[] = [
  {
    id: "a",
    position: Position.Top,
    type: "target",
  },
];

export const NODES: Node[] = [
  {
    nodeType: "cronNode",
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      title: "Cron Node",
      alt: "Cron Node",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
    },
  },
  {
    nodeType: "javascriptNode",
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      title: "JS Node",
      image_src: "/js-logo.svg",
      alt: "JS Logo",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "javascript",
    },
  },
  {
    nodeType: "manualNode",
    nodeConfigurationData: {},
    nodePresentationData: {
      title: "Manual Node",
      alt: "Manual Node",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
    },
  },
  {
    nodeType: "modelNode",
    nodeConfigurationData: {
      filename: "",
      prompt: "",
      variables: [],
    },
    nodePresentationData: {
      title: "Model Node",
      alt: "Model Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "local_model",
    },
  },
  {
    nodeType: "openAiNode",
    nodeConfigurationData: {
      url: "",
      method: "", 
      headers: "", 
      body: "",
    },
    nodePresentationData: {
      title: "OpenAI Node",
      alt: "OpenAI Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
    },
  },
  {
    nodeType: "restNode",
    nodeConfigurationData: {
      url: "",
      method: "", 
      headers: "", 
      body: "",
    },
    nodePresentationData: {
      title: "Rest API Node",
      alt: "Rest API Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
    },
  },
  {
    nodeType: "pythonNode",
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      title: "Python Node",
      alt: "Python Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "python",
    },
  },
  {
    nodeType: "receiveChatNode",
    nodeConfigurationData: {
      message: "",
    },
    nodePresentationData: {
      title: "Receive Chat Node",
      alt: "Receive Chat Node",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
    },
  },
  {
    nodeType: "sendChatNode",
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      title: "Send Chat Node",
      alt: "Send Chat Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "app_chat",
    },
  },
  {
    nodeType: "terminalNode",
    nodeConfigurationData: {
      command: "",
    },
    nodePresentationData: {
      title: "Terminal Node",
      alt: "Terminal Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "terminal",
    },
  },
  {
    nodeType: "vectorNode",
    nodeConfigurationData: {
      db: "",
      params: [],
    },
    nodePresentationData: {
      title: "Vector Node",
      alt: "Vector Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "vector",
    },
  },
];

export const getNodesForNodePanel = () => {
  //make them all "nodeType === "superNode"
  let nodes = NODES.map((node) => {
    //if its Manual Node Don't
    if (node.nodeType === "manualNode") {
      return node;
    }
    node.nodeType = "superNode";
    return node;
  });
  return nodes;
};
