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
      <div className="flex flex-grow flex-col items-center">
        <div className="flex h-72 flex-col items-center justify-center">
          <div className="my-10">
            <h1 className="text-7xl">Choose a Template</h1>
          </div>
          {/* <BaseSearch
            data={allTemplates}
            searchKey={["flow_name"]}
            onResultsChange={(results) => setResults(results)}
          /> */}
        </div>
        <div className="flex w-full items-center justify-center"></div>
        {/* Grid of templates */}
        <TemplateGrid
          AvatarComponent={Avatar}
          LinkComponent={Link}
          templates={allTemplates}
        />
      </div>
    </PageLayout>
  );
}

// const TemplateCard = ({ template }: { template: any }) => {
//   //TODO: make the icons for the trigger and the actions
//   return (
//     <BaseCard
//       key={template.flow_id}
//       as={Link}
//       to={`/templates/${template.author}/${template.flow_id}`}
//     >
//       {template.flow_name}
//     </BaseCard>
//   );
// };
