import React, { useState } from "react";
import { Link } from "react-router-dom";
import BaseCard from "../components/baseCard";
import { MockFlowDefinitions } from "../utils/mocks";
import BaseSearch from "../components/baseSearch";

export default function Templates() {
  const [searchValue, setSearchValue] = useState("");

  const handleSearchChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setSearchValue(event.target.value);
  };

  const handleSearchClick = () => {
    console.log(`Searching for: ${searchValue}`);
    // Here, you can implement whatever logic you want when the search button is clicked.
    // For instance, you might want to call an API to perform a search using the searchValue.
  };

  return (
    <div className="min-h-screen flex flex-col">
      <div className="flex-grow flex flex-col items-center">
        <div className="flex flex-col items-center justify-center h-72 ">
          <div className="my-10">
            <h1 className="text-7xl">Choose a Template</h1>
          </div>
          <BaseSearch
            value={searchValue}
            onClick={handleSearchClick}
            onChange={handleSearchChange}
          />
        </div>
        <div className="flex w-full items-center justify-center"></div>
        {/* Grid of templates */}
        <div className="grid grid-cols-3 gap-6 w-full max-w-5xl pt-10">
          {MockFlowDefinitions.map((template, index) => (
            <BaseCard as={Link} to={`/templates/${template.flow_id}`}>
              {template.flow_name}
            </BaseCard>
          ))}
        </div>
      </div>
    </div>
  );
}
