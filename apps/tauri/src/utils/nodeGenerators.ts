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

let raw_action = {
  trigger: false,
  node_name: "cli_action",
  node_label: "CLI Action",
  icon: `<svg width="24" height="24" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 3L3 1.5H21L22.5 3V21L21 22.5H3L1.5 21V3ZM3 3V21H21V3H3Z"/><path d="M7.06078 7.49988L6.00012 8.56054L10.2427 12.8032L6 17.0459L7.06066 18.1066L12 13.1673V12.4391L7.06078 7.49988Z"/><rect x="12" y="16.5" width="6" height="1.5"/></svg>`,
  description: "CLI Action",
  handles: [
    {
      id: "a",
      position: "top",
      type: "target",
    },
    {
      id: "b",
      position: "bottom",
      type: "source",
    },
  ],
  variables: [],
  config: {
    command: "",
    run_folder: "",
  },
  extension_id: "system-shell",
};

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
      pattern: "0 */5 * * * *", //every 5 minutes
    },
    trigger_type: "cron",
    mockData: {
      pattern: "0 */5 * * * *"
    },
  },
  {
    trigger: false,
    node_name: "deno_action",
    node_label: "JS Action",
    icon: rawIcons.Deno,
    description: "Deno Logo",
    handles: BaseHandles,
    variables: [],
    config: {
      code: "",
    },
    extension_id: "deno"
  },
  // {
  //   trigger: false,
  //   node_name: "js_action",
  //   node_label: "JS Action",
  //   icon: rawIcons.VscCode,
  //   description: "JS Logo",
  //   handles: BaseHandles,
  //   variables: [],
  //   config: {
  //     code: "",
  //   },
  //   extension_id: "javascript",
  // },
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
    extension_id: "local_model",
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
    extension_id: "rest",
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
    extension_id: "rest",
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
    extension_id: "python",
  },
  // {
  //   trigger: true,
  //   node_name: "app_chat_trigger",
  //   node_label: "App Chat Trigger",
  //   icon: rawIcons.VscMail,
  //   description: "App Chat Trigger",
  //   handles: StartHandles,
  //   variables: [],
  //   config: {
  //     message: "",
  //   },
  //   trigger_type: "chat",
  //   mockData: {
  //     message: "Hello, World!"
  //   },
  // },
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
    extension_id: "app_chat",
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
    extension_id: "system-shell",
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
    extension_id: "vector",
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
