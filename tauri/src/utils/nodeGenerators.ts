import { Node } from "./nodeUtils";
import { HandleProps, Position } from "reactflow";
import { TRIGGERS } from "./mocks";

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
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      node_label: "Cron Trigger",
      alt: "Cron Trigger",
      icon: "VscWatch",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      worker_name: "cron",
      trigger: true,
    },
  },
  {
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      node_label: "JS Action",
      icon: "VscCode",
      alt: "JS Logo",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "javascript",
      worker_name: "js_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {},
    nodePresentationData: {
      nodeType: "manualNode",
      node_label: "Manual Trigger",
      alt: "Manual Trigger",
      icon: "VscPerson",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      worker_name: "manual_trigger",
      trigger: true,
    },
  },
  {
    nodeConfigurationData: {
      filename: "",
      prompt: "",
      variables: [],
    },
    nodePresentationData: {
      node_label: "Model Action",
      alt: "Model Action",
      icon: "VscWand",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "local_model",
      worker_name: "local_model",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "https://api.openai.com/v1/chat/completions",
      method: "POST",
      headers:
        '{"Authorization":"Bearer OPEN_AI_API_KEY", "Content-Type":"application/json"}',
      body: '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Act like Hermione Granger and be pithy. She just tried a spell and it mostly worked."}], "temperature": 0.7 }',
    },
    nodePresentationData: {
      node_label: "OpenAI Action",
      alt: "OpenAI Action",
      icon: "https://qcuguzlfpjtyiloqtysz.supabase.co/storage/v1/object/public/random/openai-logomark.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "openai_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      node_label: "Rest API Action",
      alt: "Rest API Action",
      icon: "VscRadioTower",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "rest_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      code: "",
    },
    nodePresentationData: {
      node_label: "Python Action",
      alt: "Python Action",
      icon: "VscCode",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "python",
      worker_name: "python_action",
      trigger: false,
    },
  },
  {
  
    nodeConfigurationData: {
      message: "",
    },
    nodePresentationData: {
      node_label: "App Chat Trigger",
      alt: "App Chat Trigger",
      icon: "VscMail",
      handles: StartHandles,
    },
    nodeProcessData: {
      worker_type: "start",
      worker_name: "app_chat_trigger",
      trigger: true,
    },
  },
  {
    nodeConfigurationData: {
      pattern: "",
    },
    nodePresentationData: {
      node_label: "Send Chat Action",
      icon: "VscSend",
      alt: "Send Chat Action",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "app_chat",
      worker_name: "send_chat_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      command: "",
    },
    nodePresentationData: {
      node_label: "Terminal Action",
      alt: "Terminal Action",
      icon: "VscTerminal",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "terminal",
      worker_name: "terminal_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      db: "",
      params: [],
    },
    nodePresentationData: {
      node_label: "Vector Action",
      alt: "Vector Action",
      icon: "VscReferences",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "vector",
      worker_name: "vector_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "https://api.salesforce.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      node_label: "Salesforce Action",
      alt: "Salesforce Action",
      icon: "https://www.vectorlogo.zone/logos/salesforce/salesforce-icon.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "salesforce_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "https://api.slack.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      node_label: "Slack Action",
      alt: "Slack Action",
      icon: "https://www.vectorlogo.zone/logos/slack/slack-icon.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "slack_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "https://api.twitter.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      node_label: "Twitter Action",
      alt: "Twitter Action",
      icon: "https://www.vectorlogo.zone/logos/twitter/twitter-icon.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "twitter_action",
      trigger: false,
    },
  },
  {
    nodeConfigurationData: {
      url: "https://api.github.com/",
      method: "",
      headers: "",
      body: "",
    },
    nodePresentationData: {
      node_label: "GitHub Action",
      alt: "GitHub Action",
      icon: "https://www.vectorlogo.zone/logos/github/github-icon.svg",
      handles: BaseHandles,
    },
    nodeProcessData: {
      worker_type: "rest",
      worker_name: "github_action",
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
