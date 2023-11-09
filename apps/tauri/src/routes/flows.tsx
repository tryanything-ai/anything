import { Link } from "react-router-dom";

import { useFlowsContext } from "../context/FlowsProvider";
import PageLayout from "../pageLayout";
import { PageHeader } from "../components/wholePageHeader";

export default function Flows() {
  const { createNewFlow, flows } = useFlowsContext();

  return (
    <PageLayout>
      <PageHeader
        callback={createNewFlow}
        title="Flows"
        buttonLabel="New Flow"
      />

      <ul className="mt-4">
        {flows.map((flow) => {
          return (
            <Link
              key={flow.name}
              to={`${flow.name}`}
              className="card w-full bg-base-300 shadow-xl my-2"
            >
              <div className="card-body flex-row justify-between">
                <div className="w-1/4">
                  <div className="text-2xl">{flow.name}</div>
                </div>
                <div className="flex text-lg">Stats</div>
                <div className="flex text-lg">Live</div>
                {/* <h2 className="card-title">{flow.flow_name}</h2>
                  <div className="card-actions justify-end">
                    <div className="bg-pink-200 h-full w-full">derp</div>
                  </div> */}
              </div>
            </Link>
          );
        })}
      </ul>
    </PageLayout>
  );
}
