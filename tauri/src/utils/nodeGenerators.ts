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
      title: "Cron Trigger",
      alt: "Cron Trigger",
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
      title: "JS Action",
      icon: "VscCode",
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
      title: "Manual Trigger",
      alt: "Manual Trigger",
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
      title: "Model Action",
      alt: "Model Action",
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
      title: "OpenAI Action",
      alt: "OpenAI Action",
      icon: "https://qcuguzlfpjtyiloqtysz.supabase.co/storage/v1/object/public/random/openai-logomark.svg",
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
      title: "Rest API Action",
      alt: "Rest API Action",
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
      title: "Python Action",
      alt: "Python Action",
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
      title: "App Chat Trigger",
      alt: "App Chat Trigger",
      icon: "VscMail",
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
      title: "Send Chat Action",
      icon: "VscSend",
      alt: "Send Chat Action",
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
      title: "Terminal Action",
      alt: "Terminal Action",
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
      title: "Vector Action",
      alt: "Vector Action",
      icon: "VscReferences",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "vector",
      trigger: false,
    },
  },
  {
    nodeType: "superNode",
    nodeConfigurationData: {
      url: "https://api.salesforce.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "Salesforce Action",
      alt: "Salesforce Action",
      icon: "https://www.vectorlogo.zone/logos/salesforce/salesforce-icon.svg",
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
      url: "https://api.gmail.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "Gmail Action",
      alt: "Gmail Action",
      icon: "https://www.vectorlogo.zone/logos/google_gmail/google_gmail-icon.svg",
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
      url: "https://api.slack.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "Slack Action",
      alt: "Slack Action",
      icon: "https://www.vectorlogo.zone/logos/slack/slack-icon.svg",
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
      url: "https://api.twitter.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      title: "Twitter Action",
      alt: "Twitter Action",
      icon: "https://www.vectorlogo.zone/logos/twitter/twitter-icon.svg",
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
      url: "https://api.github.com/",
      method: "",
      headers: "",
      body: "",
    },  
    nodePresentationData: {
      title: "GitHub Action",
      alt: "GitHub Action",
      icon: "https://www.vectorlogo.zone/logos/github/github-icon.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      trigger: false,
    },
  },
];

export const getTriggerNodes = (): Node[] => {
  return NODES.filter((node) => node.nodeProcessData.trigger === true);
};

/**
 * Function to get nodes where trigger is false
 * @returns {Node[]} Array of nodes where trigger is false
 */
export const getActionNodes = (): Node[] => {
  return NODES.filter((node) => node.nodeProcessData.trigger === false);
};
