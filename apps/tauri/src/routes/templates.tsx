import { TemplateGrid } from "ui";
import { BigFlow, flowJsonFromBigFlow } from "utils";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { Avatar } from "../components/avatar";
import BaseSearch from "../components/baseSearch";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
import PageLayout from "../pageLayout";

export default function Templates() {
  const [allTemplates, setAllTemplates] = useState<BigFlow>([]);
  const [results, setResults] = useState<BigFlow>();

  const { fetchTemplates } = useMarketplaceContext();

  useEffect(() => {
    async function fetchTemplatesAsync() {
      let templates = await fetchTemplates();
      console.log("templates", JSON.stringify(templates, null, 3));
      setAllTemplates([...templates]);
    }

    fetchTemplatesAsync();
  }, []);

  return (
    <PageLayout>
      <div className="flex flex-col w-full">
        <div className="items-center gap-4">
          <h1 className="md:display h2 w-full px-4 text-center md:w-[805px] md:px-0">
            Anything Templates
          </h1>
          <p className="body-xl text-slate-11 w-full px-4 text-center md:w-[572px] md:px-0">
            Automate anything with easy to customize templates
          </p>
        </div>
        {/* <div className=" w-1/3 mx-auto mt-10">
          <BaseSearch
            data={allTemplates}
            searchKey={["flow_name"]}
            onResultsChange={(results) => setResults(results)}
          />
        </div> */}
        <div className=" my-16 flex flex-col w-full items-center justify-center">
          {/* Grid of templates */}
          <TemplateGrid
            AvatarComponent={Avatar}
            LinkComponent={Link}
            templates={allTemplates}
          />
        </div>
      </div>
    </PageLayout>
  );
}
