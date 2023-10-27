import { Link } from "react-router-dom";

import { useLocalFileContext } from "../context/LocalFileProvider";
import PageLayout from "../pageLayout";
import { PageHeader } from "../components/wholePageHeader";

export default function Flows() {
  const { createNewFlow, flows } = useLocalFileContext();

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
              key={flow.flow_name}
              to={`${flow.flow_name}`}
              className="card w-full bg-base-300 shadow-xl my-2"
            >
              <div className="card-body flex-row justify-between">
                <div className="w-1/4">
                  <div className="text-2xl">{flow.flow_name}</div>
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
