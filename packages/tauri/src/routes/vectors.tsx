import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";

export default function Vectors() {

  return (
    <div className="flex h-full w-full p-10">
      <div className="flex flex-col text-5xl m-5 w-full">
        <div className="flex flex-row justify-between">
          <div>Vectors</div>
          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              //   createNewFlow();
            }}
          >
            New Vector
          </button>
        </div>
      </div>
    </div>
  );
}
