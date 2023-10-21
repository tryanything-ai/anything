import { useLocalFileContext } from "../context/LocalFileProvider";
import { Link } from "react-router-dom";
import BaseCard from "../components/baseCard";

export default function Home() {
  const { flows, createNewFlow } = useLocalFileContext();

  return (
    <div className="flex flex-row h-full w-full m-10">
      {/* FLows */}

      <div className="flex flex-col text-5xl m-5">
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

      <div className="flex flex-col text-5xl m- w-96 m-5">
        <div className="flex flex-row justify-between">
          <div>Templates</div>

          <Link className="btn btn-primary m-1 ml-4" to="/templates">
            Explore
          </Link>
        </div>

        <ul></ul>
      </div>
    </div>
  );
}
