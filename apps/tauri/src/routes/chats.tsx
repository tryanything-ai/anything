import {
  useEffect,
  useState,
} from "react";
import { Link } from "react-router-dom";
// import api from "../tauri_api/api";

export default function Chats() {
  const [chats, setChats] = useState<any[]>([]);

  // const fetchData = async () => {
  //   let res = await api.getFlowsWitChats();
  //   console.log("Chats from rust" + JSON.stringify(res));
  //   setChats(res as any[]);
  // };
  useEffect(() => {
    // fetchData();
  }, []);

  return (
    <div className="flex h-full w-full p-10">
      <div className="flex flex-col text-5xl m-5 w-full">
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
