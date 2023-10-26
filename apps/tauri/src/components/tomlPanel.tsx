import Editor from "@monaco-editor/react";
import { parse } from "iarna-toml-esm";
import { useParams } from "react-router-dom";

import { useFlowContext } from "../context/FlowProvider";
import { useTauriContext } from "../context/TauriProvider";
import api from "../tauri_api/api";

const TomlPanel = () => {
  const { toml } = useFlowContext();
  const { appDocuments } = useTauriContext();
  const { flow_name } = useParams();

  const handleEditorChange = async (value: any, event: any) => {
    try {
      //TODO: manage coding errors differently. Can't add random stuff right now
      let parseable = parse(value);
      if (parseable) {
        // console.log(
        //   "writing toml to",
        //   appDocuments + "/flows/" + flow_name + "/flow.toml"
        // );
        await api.fs.writeTextFile(
          appDocuments + "/flows/" + flow_name + "/flow.toml",
          value
        );
      }
    } catch (error) {
      console.log("error parsing toml", error);
    }
  };

  return (
    <div className="flex flex-col h-full">
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
