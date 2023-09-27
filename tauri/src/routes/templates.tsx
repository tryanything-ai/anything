import { useState } from "react";
import { Link } from "react-router-dom";
import BaseCard from "../components/baseCard";
import { MockFlowDefinitions } from "../utils/mocks";
import BaseSearch from "../components/baseSearch";
import { RustFlow } from "../utils/flowConversion";

export default function Templates() {
  const [results, setResults] = useState<RustFlow[]>(MockFlowDefinitions);

  return (
    <div className="min-h-screen flex flex-col">
      <div className="flex-grow flex flex-col items-center">
        <div className="flex flex-col items-center justify-center h-72 ">
          <div className="my-10">
            <h1 className="text-7xl">Choose a Template</h1>
          </div>
          <BaseSearch
            data={MockFlowDefinitions}
            searchKey={["flow_name"]}
            onResultsChange={(results) => setResults(results)}
          />
        </div>
        <div className="flex w-full items-center justify-center"></div>
        {/* Grid of templates */}
        <div className="grid grid-cols-3 gap-6 w-full max-w-5xl pt-10">
          {results.map((template, index) => (
            <BaseCard
              key={template.flow_id}
              as={Link}
              to={`/templates/${template.flow_id}`}
            >
              {template.flow_name}
            </BaseCard>
          ))}
        </div>
      </div>
    </div>
  );
}
