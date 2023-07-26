import { useParams } from "react-router-dom";
import { useFlowContext } from "../context/FlowProvider";
import Editor from "@monaco-editor/react";
import { writeTextFile } from "@tauri-apps/api/fs";
import { useTauriContext } from "../context/TauriProvider";
import { stringify, parse } from "iarna-toml-esm";

const TomlPanel = () => {
  const { toml } = useFlowContext();
  const { appDocuments } = useTauriContext();
  const { flow_name } = useParams();

  const handleEditorChange = async (value: any, event: any) => {
    try {
      let parseable = parse(value);
      if (parseable) {
        console.log(
          "writing toml to",
          appDocuments + "/flows/" + flow_name + "/flow.toml"
        );
        await writeTextFile(
          appDocuments + "/flows/" + flow_name + "/flow.toml",
          value
        );
      }
    } catch (error) {
      console.log("error parsing toml", error);
    }
  };

  return (
    <div className="flex flex-col h-full pt-2 border-l border-gray-500">
      <Editor
        language="toml"
        height="100vh"
        theme="vs-dark"
        className=""
        defaultLanguage="markdown"
        value={toml}
        onChange={handleEditorChange}
        options={{
          fontSize: 15,
        }}
      />
    </div>
  );
};

export default TomlPanel;
