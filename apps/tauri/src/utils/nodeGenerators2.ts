import { Node, Trigger, Action } from "./flowTypes";
import { HandleProps, Position } from "reactflow";
import * as rawIcons from "./rawIcons";

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

const NODES: (Action | Trigger)[] = [
  {
    trigger: true,
    node_name: "Cron Trigger",
    node_label: "Cron Trigger",
    icon: rawIcons.VscWatch,
    description: "Fire at a given time",
    handles: StartHandles,
    variables: [],
    config: {
      pattern: "*/5 * * * *", //every 5 minutes
    },
    trigger_type: "cron",
    mockData: {},
  },
  {
    trigger: false,
    node_name: "JS Action",
    node_label: "JS Action",
    icon: rawIcons.VscCode,
    description: "JS Logo",
    handles: BaseHandles,
    variables: [],
    config: {
      code: "",
    },
    engine: "javascript",
    depends_on: [],
  },
  {
    trigger: true,
    node_name: "Manual Trigger",
    node_label: "Manual Trigger",
    icon: rawIcons.VscPerson,
    description: "Manual Trigger",
    handles: StartHandles,
    variables: [],
    config: {},
    trigger_type: "manual",
    mockData: {},
  },
  {
    trigger: false,
    node_name: "Local Model Action",
    node_label: "Local Model Action",
    icon: rawIcons.Llama,
    description: "Local Model Action",
    handles: BaseHandles,
    variables: [],
    config: {
      filename: "",
      prompt: "",
    },
    engine: "local_model",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "OpenAI Action",
    node_label: "OpenAI Action",
    icon: rawIcons.OpenAi,
    description: "OpenAI Action",
    handles: BaseHandles,
    variables: [],
    config: {
      url: "https://api.openai.com/v1/chat/completions",
      method: "POST",
      headers:
        '{"Authorization":"Bearer OPEN_AI_API_KEY", "Content-Type":"application/json"}',
      body: '{"model": "gpt-3.5-turbo", "messages": [{"role": "user", "content": "Act like Hermione Granger and be pithy. She just tried a spell and it mostly worked."}], "temperature": 0.7 }',
    },
    engine: "rest",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "Rest API Action",
    node_label: "Rest API Action",
    icon: rawIcons.VscRadioTower,
    description: "Rest API Action",
    handles: BaseHandles,
    variables: [],
    config: {
      url: "",
      method: "",
      headers: "",
      body: "",
    },
    engine: "rest",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "Python Action",
    node_label: "Python Action",
    icon: rawIcons.VscCode,
    description: "Python Action",
    handles: BaseHandles,
    variables: [],
    config: {
      code: "",
    },
    engine: "python",
    depends_on: [],
  },
  {
    trigger: true,
    node_name: "App Chat Trigger",
    node_label: "App Chat Trigger",
    icon: rawIcons.VscMail,
    description: "App Chat Trigger",
    handles: StartHandles,
    variables: [],
    config: {
      message: "",
    },
    trigger_type: "chat",
    mockData: {},
  },
  {
    trigger: false,
    node_name: "Send Chat Action",
    node_label: "Send Chat Action",
    icon: rawIcons.VscSend,
    description: "Send Chat Action",
    handles: BaseHandles,
    variables: [],
    config: {
      pattern: "",
    },
    engine: "app_chat",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "Terminal Action",
    node_label: "Terminal Action",
    icon: rawIcons.VscTerminal,
    description: "Terminal Action",
    handles: BaseHandles,
    variables: [],
    config: {
      command: "",
    },
    engine: "terminal",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "Vector Action",
    node_label: "Vector Action",
    icon: rawIcons.VscReferences,
    description: "Vector Action",
    handles: BaseHandles,
    variables: [],
    config: {
      db: "",
      params: "",
    },
    engine: "vector",
    depends_on: [],
  },
];

export const getTriggerNodes = (): Trigger[] => {
  return NODES.filter((node): node is Trigger => node.trigger === true);
};

/**
 * Function to get nodes where trigger is false
 * @returns {Node[]} Array of nodes where trigger is false
 */
export const getActionNodes = (): Action[] => {
  return NODES.filter((node): node is Action => node.trigger === false);
};
