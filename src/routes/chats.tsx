import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";

export default function Chats() { 
    //TODO: Get a list of flows that have receive_chage and send_chat nodes in them. 
  //   const { flowPaths, createNewFlow } = useLocalFileContext();
  return (
    <div className="flex h-full w-full p-10">
      <div className="flex flex-col text-5xl text-primary-content m-5 w-full">
        <div className="flex flex-row justify-between">
          <div>Chats</div>
          {/* <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              //   createNewFlow();
            }}
          >
            New Chats
          </button> */}
        </div>
      </div>
    </div>
  );
}
