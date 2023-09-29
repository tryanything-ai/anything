import { useState, useEffect } from "react";
import { useParams } from "react-router-dom";
import { useMarketplaceContext } from "../context/MarketplaceProvider";

const Template = () => {
  const { author_username, template_name } = useParams<{
    author_username: string;
    template_name: string;
  }>();

  const { fetchTemplate } = useMarketplaceContext();
  const [template, setTemplate] = useState();
  const _fetchTemplate = async () => {
    if (!author_username || !template_name) return;
    let template = await fetchTemplate(author_username, template_name);
    console.log(template);
    setTemplate(template);
  };

  useEffect(() => {
    if (!author_username || !template_name) return;
    _fetchTemplate();
  }, [author_username, template_name]);

  return (
    <div>
      <h1>Template</h1>
    </div>
  );
};

export default Template;
