import { Button } from "@anything/ui";
import { Link } from "react-router-dom";

import BaseCard from "../components/baseCard";
import { useLocalFileContext } from "../context/LocalFileProvider";
 
export default function Home() {
  const { flows, createNewFlow } = useLocalFileContext();

  return (
    <div className="m-10 flex h-full w-full flex-row">
      {/* FLows */}

      <div className="m-5 flex flex-col text-5xl">
        <div className="flex flex-row justify-between">
          <div>Flows</div>

          <button
            className="btn btn-primary m-1 ml-4"
            onClick={() => {
              createNewFlow();
            }}
          >
            New Flow
          </button>
        </div>

        <ul>
          {flows.map((flow) => {
            return (
              <BaseCard
                as={Link}
                key={flow.flow_name}
                to={`flows/${flow.flow_name}`}
                className="w-96"
              >
                <h2 className="card-title">{flow.flow_name}</h2>
              </BaseCard>
            );
          })}
        </ul>
      </div>

      {/* Tables */}

      <div className="m- m-5 flex w-96 flex-col text-5xl">
        <div className="flex flex-row justify-between">
          <div>Templates</div>

          <Link className="btn btn-primary m-1 ml-4" to="/templates">
            Explore
          </Link>
        </div>
        <Button />
        <ul></ul>
      </div>
    </div>
  );
}
