import { useEffect, useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";

import { useFlowContext } from "../context/FlowProvider";
import RequireAuth from "./RequireAuth";
import slugify from "slugify";

type Inputs = {
  flow_name: string;
};

const FlowSharingPanel = () => {
  const [loading, setLoading] = useState(false);

  const { updateFlowFrontmatter } = useFlowContext();
  const { flow_name } = useParams();
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  //TODO: create flow
  //TODO: create flow version
  //TODO: publish new flow_version if flow exist
  //TODO: force user login / signup and/or username creation if new

  const saveTemplate: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      // TODO: validate inputs
      //TODO: show errors
      if (flow_name && data.flow_name != flow_name) {
       
      }
    } catch (error) {
      console.log(error);
    } finally {
      console.log(data);
      setLoading(false);
    }
  };

  return (
    <RequireAuth>
      <div className="flex flex-col h-full gap-5 p-4">
        <h1 className="text-2xl font-bold">Flow Sharing</h1>
        {/* View Published */}
        <div className="btn btn-link">View Published Template</div>
        <div>Template Name:</div>
        <div>{flow_name}</div>
        <div>Sharing URL:</div>
        <div>{`tryanything.xyz/templates/${slugify(flow_name)}`}</div>
        <div className="btn btn-secondary">Publish Template</div>
        {/* TODO: */}
        <div>Choose Tags</div>
      </div>
    </RequireAuth>
  );
};

export default FlowSharingPanel;
