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
    nodeType: "superNode",
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      title: "Cron Node",
      alt: "Cron Node",
      icon: "VscWatch",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      trigger: true,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      title: "JS Node",
      icon: "/js-logo.svg",
      alt: "JS Logo",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "javascript",
      trigger: false,
    },
  },
  {
    nodeType: "manualNode",
    nodeConfigurationData: {},
    nodePresentationData: {
      title: "Manual Node",
      alt: "Manual Node",
      icon: "VscPerson",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      trigger: true,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      filename: "",
      prompt: "",
      variables: [],
    },
    nodePresentationData: {
      title: "Model Node",
      alt: "Model Node",
      icon: "VscWand",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "local_model",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      url: "",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "OpenAI Node",
      alt: "OpenAI Node",
      icon: "VscRadioTower",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      url: "",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "Rest API Node",
      alt: "Rest API Node",
      icon: "VscRadioTower",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      title: "Python Node",
      alt: "Python Node",
      icon: "VscCode",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "python",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      message: "",
    },
    nodePresentationData: {
      title: "Receive Chat Node",
      icon: "VscMail",
      alt: "Receive Chat Node",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      trigger: true,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      title: "Send Chat Node",
      icon: "VscSend",
      alt: "Send Chat Node",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "app_chat",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      command: "",
    },
    nodePresentationData: {
      title: "Terminal Node",
      alt: "Terminal Node",
      icon: "VscTerminal",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "terminal",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      db: "",
      params: [],
    },
    nodePresentationData: {
      title: "Vector Node",
      alt: "Vector Node",
      icon: "VscReferences",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "vector",
      trigger: false,
    },
  },
];

export const getTriggerNodes = (): Node[] => {
  return NODES.filter(node => node.nodeProcessData.trigger === true);
};

/**
 * Function to get nodes where trigger is false
 * @returns {Node[]} Array of nodes where trigger is false
 */
export const getActionNodes = (): Node[] => {
  return NODES.filter(node => node.nodeProcessData.trigger === false);
};
