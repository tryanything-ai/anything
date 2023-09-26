import { Link } from "react-router-dom";
import BaseCard from "../components/baseCard";
import { MockFlowDefinitions } from "../utils/mocks";

export default function Templates() {
  return (
    <div className="min-h-screen flex flex-col">
      <div className="flex-grow flex flex-col items-center">
        <div className="flex flex-col items-center justify-center h-72 ">
          <div className="my-10">
            <h1 className="text-5xl">Choose a Template</h1>
          </div>

          <div className="form-control w-full">
            <div className="input-group w-96">
              <input
                type="text"
                placeholder="Search…"
                className="input input-bordered w-full"
              />
              <button className="btn btn-square">
                <svg
                  xmlns="http:www.w3.org/2000/svg"
                  className="h-6 w-6"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth="2"
                    d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                  />
                </svg>
              </button>
            </div>
          </div>
        </div>

        <div className="flex w-full items-center justify-center">
          {/* <input
            type="text"
            placeholder="Search templates..."
            className="w-full p-2 rounded border shadow-sm"
          /> */}
        </div>
        {/* Grid of templates */}
        <div className="grid grid-cols-3 gap-6 w-full max-w-5xl pt-10">
        {MockFlowDefinitions.map(
            (template, index) => (
              <BaseCard
                as={Link}
                to={`/templates/${index}`}
                //TODO: change to id
              >
                {template.flow_name}
              </BaseCard>
            )
          )}
        </div>
      </div>
    </div>
    // <div className="flex flex-col h-full w-full m-10">
    //   <div className=" items-center justify-center text-5xl h-32 bg-pink-200">
    //     <div className="flex flex-row justify-center">
    //       <div>Templates</div>
    //     </div>
    //   </div>

    // </div>
  );
}
