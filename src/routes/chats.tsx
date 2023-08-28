import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";

import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import { invoke } from "@tauri-apps/api";

export default function Chats() {
  const [chats, setChats] = useState<any[]>([]);

  useEffect(() => {
    invoke("get_chat_flows").then((result) => {
      console.log("Chats from rust" + JSON.stringify(result));
      setChats(result as any[]);
    });
  }, []);

  return (
    <div className="flex h-full w-full p-10">
    <div className="flex flex-col text-5xl text-primary-content m-5 w-full">
      <div className="flex flex-row justify-between">
        <div>Chats</div>
        {/* <button
          className="btn btn-primary m-1 ml-4"
          onClick={() => {
            // createNewFlow();
          }}
        >
          New Flow
        </button> */}
      </div>
      <ul className="mt-4">
        {chats.map((chat) => {
          return (
            <Link
              key={chat.flow_name}
              to={`${chat.flow_name}`}
              className="card w-full bg-base-300 shadow-xl my-2"
            >
              <div className="card-body flex-row justify-between">
                <div className="w-1/4">
                  <div className="text-2xl">{chat.flow_name}</div>
                </div>
                <div className="flex text-lg">Stats</div>
                <div className="flex text-lg">Live</div>
                {/* <h2 className="card-title">{chat.name}</h2>
                <div className="card-actions justify-end">
                  <div className="bg-pink-200 h-full w-full">derp</div>
                </div> */}
              </div>
            </Link>
          );
        })}
      </ul>
    </div>
  </div>
  );
}
