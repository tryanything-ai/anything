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

const NODES: Node[] = [
  {
    trigger: true,
    node_name: "cron_trigger",
    node_label: "Cron Trigger",
    icon: rawIcons.VscWatch,
    description: "Fire at a given time",
    handles: StartHandles,
    variables: [],
    config: {
      pattern: "*/5 * * * *", //every 5 minutes
    },
    trigger_type: "cron",
    mockData: {
      pattern: "*/5 * * * *"
    },
  },
  {
    trigger: false,
    node_name: "js_action",
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
    node_name: "manual_trigger",
    node_label: "Manual Trigger",
    icon: rawIcons.VscPerson,
    description: "Manual Trigger",
    handles: StartHandles,
    variables: [],
    config: {},
    trigger_type: "manual",
    mockData: {
      click: true
    },
  },
  {
    trigger: false,
    node_name: "local_model_action",
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
    node_name: "openai_action",
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
    node_name: "rest_api_action",
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
    node_name: "python_action",
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
    node_name: "app_chat_trigger",
    node_label: "App Chat Trigger",
    icon: rawIcons.VscMail,
    description: "App Chat Trigger",
    handles: StartHandles,
    variables: [],
    config: {
      message: "",
    },
    trigger_type: "chat",
    mockData: {
      message: "Hello, World!"
    },
  },
  {
    trigger: false,
    node_name: "send_chat_action",
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
    node_name: "cli_action",
    node_label: "CLI Action",
    icon: rawIcons.VscTerminal,
    description: "CLI Action",
    handles: BaseHandles,
    variables: [],
    config: {
      command: "",
      run_folder: "",
    }, 
    engine: "system-shell",
    depends_on: [],
  },
  {
    trigger: false,
    node_name: "vector_action",
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
