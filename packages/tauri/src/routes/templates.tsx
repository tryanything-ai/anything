import { TemplateCard,TemplateGrid } from "@anything/ui";
import {
  BigFlow, 
  flowJsonFromBigFLow
} from "@anything/utils";
import { useEffect, useState } from "react";
import { Link } from "react-router-dom";

import { Avatar } from "../components/avatar";
import BaseSearch from "../components/baseSearch";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
// import { Flow } from "../utils/newNodes";

export default function Templates() {
  const [allTemplates, setAllTemplates] = useState<BigFlow[]>([]);
  const [results, setResults] = useState<BigFlow[]>();

  const { fetchTemplates } = useMarketplaceContext();

  useEffect(() => {
    async function fetchTemplatesAsync() {
      let templates = await fetchTemplates();

      setAllTemplates([...templates]);
    }

    fetchTemplatesAsync();
  }, []);

  return (
    <div className="min-h-screen flex flex-col">
      <div className="flex-grow flex flex-col items-center">
        <div className="flex flex-col items-center justify-center h-72 ">
          <div className="my-10">
            <h1 className="text-7xl">Choose a Template</h1>
          </div>
          <BaseSearch
            data={allTemplates}
            searchKey={["flow_name"]}
            onResultsChange={(results) => setResults(results)}
          />
        </div>
        <div className="flex w-full items-center justify-center"></div>
        {/* Grid of templates */}
        <TemplateGrid>
          {results?.map((template, index) => {
            let flow_json = flowJsonFromBigFLow(template);
            return (
              <TemplateCard
                AvatarComponent={() => (
                  <Avatar
                    avatar_url={template?.profiles?.avatar_url || ""}
                    profile_name={template?.profiles?.full_name || ""}
                  />
                )}
                Link={Link}
                key={index}
                profile={true}
                // tags={template.tags}
                // avatar_url={template?.profiles?.avatar_url || ""}
                username={template?.profiles?.username || ""}
                profile_name={template?.profiles?.full_name || ""}
                description={
                  template.flow_template_description
                    ? template.flow_template_description
                    : ""
                }
                flow_template_json={flow_json}
                slug={template.slug}
                flow_name={template.flow_template_name}
              />
            )
          })}
        </TemplateGrid>
        
        {/* <div className="grid grid-cols-3 gap-6 w-full max-w-5xl pt-10">
          {results.map((template, index) => (
            <TemplateCard key={template.flow_id} template={template} />
          ))}
        </div> */}
      </div>
    </div>
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
