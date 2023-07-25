import { useLocalFileContext } from "../context/LocalFileProvider";
import Editor from "@monaco-editor/react";

const TomlPanel = () => {
  const { toml, writeToml } = useLocalFileContext();
  const handleEditorChange = (value: any, event: any) => {
    console.log("EditorChanged", value);
    writeToml(value);
    // setUserInteraction(true);
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
