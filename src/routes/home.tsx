import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useLocalFileContext } from "../context/LocalFileProvider";

export default function Home() {
  const { flowPaths } = useLocalFileContext();
  return (
    <div className="flex flex-col h-full w-full m-10">
      <div className="text-5xl text-white m-5">Flows</div>
      <ul>
        {flowPaths.map((flow) => {
          return (
            <li className="card w-96 bg-base-300 shadow-xl">
              <div className="card-body">
                <h2 className="card-title">{flow.name}</h2>
                {/* <p>Flow Description</p> */}
                {/* <div className="card-actions justify-end">
                  <button className="btn btn-primary">Buy Now</button>
                </div> */}
              </div>
            </li>
          );
        })}
      </ul>
    </div>
  );
}
