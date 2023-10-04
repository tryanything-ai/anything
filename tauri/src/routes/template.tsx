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
    if (!author_username || !template_name) {
      console.log("Author username or template name not found.");
      return;
    }
    let template = await fetchTemplate(author_username, template_name);
    console.log(template);
    setTemplate(template);
  };

  useEffect(() => {
    if (!author_username || !template_name) {
      console.log("Author username or template name not found.");
      return;
    }
    _fetchTemplate();
  }, [author_username, template_name]);

  return (
    <div>
      <h1>{template_name}</h1>
      <h2>Author: {author_username}</h2>
      <h3>Triggers</h3>
      {/* {template?.triggers.map((trigger, index) => (
        <div key={index}>
          <p>{trigger.name}</p>
          <p>{trigger.description}</p>
        </div>
      ))} */}
      <h3>Actions</h3>
      {/* {template?.actions.map((action, index) => (
        <div key={index}>
          <p>{action.name}</p>
          <p>{action.description}</p>
        </div>
      ))} */}
    </div>
  );
};

export default Template;
