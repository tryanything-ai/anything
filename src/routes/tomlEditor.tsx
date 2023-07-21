import React, { useState, useEffect } from "react";
import Editor from "@monaco-editor/react";
import { useLocalFileContext } from "../context/LocalFileProvider";
// import { useFileContext } from "../context/FileProvider";

export default function TomlEditor() {
  const { toml, setToml } = useLocalFileContext();
  // const { filePath, fileContent, updateFile } = useFileContext();

  // const [markdown, setMarkdown] = useState("");
  // const [userInteraction, setUserInteraction] = useState(false);

  const updateContext = () => {
    // console.log("updateContext");
    // updateFile(filePath, markdown);
  };

  //from GPT
  // useEffect(() => {
  //   if (userInteraction) {
  //     const delay = 1000; // Delay in milliseconds
  //     let timeoutId;

  //     const debounceInput = () => {
  //       clearTimeout(timeoutId);
  //       timeoutId = setTimeout(() => {
  //         updateContext();
  //         setUserInteraction(false);
  //       }, delay);
  //     };

  //     debounceInput();

  //     return () => {
  //       clearTimeout(timeoutId); // Cleanup timeout on component unmount
  //     };
  //   }
  // }, [markdown]);

  const handleEditorChange = (value: any, event: any) => {
    console.log("EditorChanged", value);
    setToml(value);
    // setUserInteraction(true);
  };

  //end GPT

  // useEffect(() => {
  // console.log("filePath", filePath);
  // console.log("fileContent", fileContent);
  // setMarkdown(fileContent);
  // }, [filePath, fileContent]);

  return (
    <div className="flex flex-row h-full w-full mt-6">
      <div className="flex flex-col w-full h-full">
        <Editor
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
    </div>
  );
}
