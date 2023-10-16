import { RustFlow, RustTrigger } from "./flowConversion";

export const MockFlowDefinitions: RustFlow[] = [
  {
    flow_name: "Simple Cron Flow",
    author: "Author 1",
    author_id: "1",
    flow_id: "1",
    version: "0.1",
    description: "A simple flow that echos holiday cheer",
    variables: [],
    trigger: {
      name: "file_change",
      settings: {
        file: "/Users/auser/Desktop/watch-directory",
      },
    },
    nodes: [
      {
        name: "echocheer",
        label: "Holiday cheers",
        depends_on: [],
        variables: [
          {
            cheers: "Jingle Bells",
          },
        ],
        action: {
          action_type: "Shell",
          config: {
            command: "echo 'Jingle Bells'",
          },
        },
      },
      {
        name: "say-cheers",
        label: "say holiday cheer",
        depends_on: ["echocheer"],
        variables: [
          {
            cheer: "{{echocheer.stdout}}",
          },
        ],
        action: {
          action_type: "Shell",
          config: {
            command: "echo 'Heres my cheers: {{cheer}}'",
            executor: "/bin/bash",
            args: [],
          },
        },
      },
    ],
    environment: {
      NODE_ENV: "development",
    },
  },
  {
    flow_name: "Simple AI Flow",
    author: "Author 1",
    author_id: "1",
    flow_id: "2",
    version: "0.1",
    description: "A simple flow that echos holiday cheer",
    variables: [],
    trigger: {
      name: "file_change",
      settings: {
        file: "/Users/auser/Desktop/watch-directory",
      },
    },
    nodes: [
      {
        name: "echocheer",
        label: "Holiday cheers",
        depends_on: [],
        variables: [
          {
            cheers: "Jingle Bells",
          },
        ],
        action: {
          action_type: "Shell",
          config: {
            command: "echo 'Jingle Bells'",
          },
        },
      },
      {
        name: "say-cheers",
        label: "say holiday cheer",
        depends_on: ["echocheer"],
        variables: [
          {
            cheer: "{{echocheer.stdout}}",
          },
        ],
        action: {
          action_type: "Shell",
          config: {
            command: "echo 'Heres my cheers: {{cheer}}'",
            executor: "/bin/bash",
            args: [],
          },
        },
      },
    ],
    environment: {
      NODE_ENV: "development",
    },
  },
];

export const TRIGGERS: RustTrigger[] = [
  {
    name: "file_change",
    settings: {
      file: "",
    },
  },
  {
    name: "webhook",
    settings: {},
  },
  {
    name: "schedule",
    settings: {},
  },
];

// export const MockTemplates = [
//   {
//     name: "Template 1",
//     description: "This is a template",
//     author: "Author 1",
//     author_id: "1",
//     template_id: "1",
//     template_name: "Template 1",
//     definitions: MockFlowDefinitions[0],
//   },
//   {
//     name: "Template 2",
//     description: "This is a another templates",
//     author: "Author 2",
//     author_id: "1",
//     template_id: "1",
//     template_name: "Template 2",
//     definitions: MockFlowDefinitions[0],
//   },
// ];