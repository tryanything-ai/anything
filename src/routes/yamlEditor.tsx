export default function YamlEditor() {
  return (
    <div className="p-3 whitespace-pre">
      {`
    nodes:
    - id: "1"
      type: "inboxNode"
      position:
        x: 100
        y: 50
      data:
        value: "B2B Leads ✉️"
  
    - id: "2"
      type: "vectorNode"
      position:
        x: 200
        y: 300
      data:
        value: "SQLite Vector"
  
    - id: "3"
      type: "pushNode"
      position:
        x: 600
        y: 200
      data:
        value: "Gmail Inbox"
  
    - id: "4"
      type: "polyNode"
      position:
        x: 600
        y: 400
      data:
        value: "Local Obsidian Notes"
  
    - id: "5"
      type: "llmNode"
      position:
        x: 200
        y: 550
      data:
        value: "OpenAI GPT LLM"
  
    - id: "6"
      type: "outboxNode"
      position:
        x: 100
        y: 700
      data:
        value: "Outbound Sales ⌲"
  
  edges:
    - id: "1"
      source: "1"
      target: "2"
      sourceHandle: "b"
      targetHandle: "a"
  
    - id: "2"
      source: "3"
      target: "2"
      sourceHandle: "a"
      targetHandle: "b"
      animated: true
  
    - id: "3"
      source: "4"
      target: "2"
      sourceHandle: "a"
      targetHandle: "b"
      animated: true
  
    - id: "4"
      source: "2"
      target: "5"
      sourceHandle: "c"
      targetHandle: "a"
  
    - id: "5"
      source: "5"
      target: "6"
      sourceHandle: "b"
      targetHandle: "a"
  
  `}
    </div>
  );
}
