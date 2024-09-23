import React from "react";
import { AvatarAndUsername } from "./avatarAndUsername";
import { BaseNodeIcon } from "./baseNodeIcons";
import { DBFlowTemplate } from "@repo/anything-api";
import { ArrowBigRight } from "lucide-react";

export interface TemplateCardProps {
  slug: string;
  description: string;
  profileName: string;
  profile: boolean;
  username: string;
  flowName: DBFlowTemplate;
  template: any;
  Link: React.ComponentType<any>;
  AvatarComponent: React.ComponentType;
}

const TemplateCard = ({
  template,
  username,
  profileName,
  profile,
  slug,
  description,
  flowName,
  Link,
  AvatarComponent,
}: TemplateCardProps) => {
  const getFlowDetails = (template: DBFlowTemplate) => {
    const latestVersion = template.flow_template_versions[0];
    if (!latestVersion || !latestVersion.flow_definition) {
      return { trigger: null, actions: [] };
    }

    const { actions } = latestVersion.flow_definition;
    const trigger = actions.find((action) => action.type === "trigger");
    const nonTriggerActions = actions.filter(
      (action) => action.type !== "trigger",
    );

    return { trigger, actions: nonTriggerActions };
  };

  const { trigger, actions } = getFlowDetails(template);

  return (
    <Link
      data-ph-capture-attribute-flow-template-name={flowName}
      data-ph-capture-attribute-flow-template-slug={slug}
      href={`/templates/${slug}`}
      to={`/templates/${slug}`}
    >
      <div className="bg-white rounded-lg border border-gray-300 hover:bg-gray-50 transition-all duration-300 overflow-hidden">
        <div className="p-6">
          <h2 className="text-2xl font-bold mb-2 text-gray-800 truncate">
            {template.flow_template_name}
          </h2>
          <p className="text-gray-600 text-sm mb-4 line-clamp-2 font-light">
            {description}
          </p>
          <div className="h-px w-full bg-gray-200 mb-4" />
          {trigger?.icon && <NodeArray trigger={trigger} actions={actions} />}
        </div>
      </div>
    </Link>
  );
};

export default TemplateCard;

const NodeArray = ({ actions, trigger }: { actions: any[]; trigger: any }) => {
  const visibleActions = actions.slice(0, 4);
  const hiddenIconsCount = actions.length - visibleActions.length;

  return (
    <div className="flex items-center space-x-2">
      <div className="bg-white border border-gray-600 text-primary-content flex h-14 w-14 flex-row rounded-md text-xl hover:bg-gray-50">
        <BaseNodeIcon icon={trigger.icon} />
      </div>
      <ArrowBigRight className="text-gray-600" />
      {visibleActions.map((action, index) => (
        <div
          key={index}
          className="bg-white border border-gray-600 text-primary-content flex h-14 w-14 flex-row rounded-md text-xl hover:bg-gray-50"
        >
          <BaseNodeIcon icon={action.icon} />
        </div>
      ))}
      {hiddenIconsCount > 0 && (
        <div className="flex h-14 w-14 items-center justify-center rounded-md bg-white bg-opacity-30 border border-gray-600 text-gray-600 font-medium">
          +{hiddenIconsCount}
        </div>
      )}
    </div>
  );
};
