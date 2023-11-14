import { Link } from "react-router-dom";

import { useFlowsContext } from "../context/FlowsProvider";
import PageLayout from "../pageLayout";
import { HeaderButton, PageHeader } from "../components/wholePageHeader";

export default function Flows() {
  //TODO: need a way to fetch if flows are stopped etc
  const { createNewFlow, flows, stopExecution, updateFlow } = useFlowsContext();

  return (
    <PageLayout>
      <PageHeader
        title="Flows"
        ActionComponent={() => {
          return (
            <div>
              <HeaderButton
                className="btn-error btn-outline"
                callback={stopExecution}
              >
                Pause System
              </HeaderButton>
              <HeaderButton callback={createNewFlow}>New Flow</HeaderButton>
            </div>
          );
        }}
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
                {/* <div className="flex text-lg">Live</div> */}
                <label
                  className="flex justify-center items-center gap-2 text-lg"
                  onClick={(e) => e.stopPropagation()}
                >
                  {flow.active ? "Live" : "Paused"}
                  <input
                    className="toggle toggle-success"
                    type="checkbox"
                    onChange={(e) => {
                      e.stopPropagation();
                      updateFlow(flow.flow_id, {
                        active: !flow.active,
                        flow_name: flow.name,
                        version: flow.latest_version_id,
                      });
                    }}
                    checked={flow.active}
                  />
                </label>
              </div>
            </Link>
          );
        })}
      </ul>
    </PageLayout>
  );
}
