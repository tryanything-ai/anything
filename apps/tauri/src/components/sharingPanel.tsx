import { useEffect, useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";

import { useFlowContext } from "../context/FlowProvider";
import RequireAuth from "./RequireAuth";
import slugify from "slugify";

import { BigFlow, MockNewFlows } from "utils";
import { useMarketplaceContext } from "../context/MarketplaceProvider";

type Inputs = {
  flow_name: string;
};

const FlowSharingPanel = () => {
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [publishedFlow, setPublishedFlow] = useState<BigFlow>(null);
  // does a template and a version already exist.
  // const [publishedTemplate, pub] = useState(false);
  // const { session } = useAuthenticationContext();
  const { flowFrontmatter } = useFlowContext();
  const { saveTemplate, fetchTemplateById } = useMarketplaceContext();
  const { flow_name } = useParams();
  const navigate = useNavigate();

  type Inputs = {
    description: string;
  };

  const {
    register,
    handleSubmit,
    control,
    formState: { errors, isDirty, touchedFields },
  } = useForm<Inputs>();

  //TODO: create flow
  //TODO: create flow version
  //TODO: publish new flow_version if flow exist
  //TODO: force user login / signup and/or username creation if new
  //TODO: check if flow name is unique

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    // const _saveTemplate = async () => {
    try {
      setSaving(true);

      //TODO: validate inputs
      //TODO: show errors
      if (flow_name && data.description) {
        console.log("Saving");
        let res = await saveTemplate(
          flow_name,
          data.description,
          MockNewFlows[0]
        );
        if (res) {
          setPublishedFlow(res);
        }
      }
    } catch (error) {
      console.log(error);
    } finally {
      console.log("Done");
      setSaving(false);
    }
  };

  const _fetchTemplateById = async () => {
    if (flowFrontmatter.flow_id) {
      let res = fetchTemplateById(flowFrontmatter.flow_id);
      console.log(res);
    }
  };

  //TODO:we shoudl update flowFrontmatter when we update description so we might need to change this
  useEffect(() => {
    //TODO: check if already "published"
    _fetchTemplateById();
  }, [flowFrontmatter]);

  return (
    <RequireAuth>
      <div className="flex flex-col h-full gap-5 p-4">
        <h1 className="text-2xl font-bold">Flow Sharing</h1>
        {/* View Published */}
        {publishedFlow ? (
          <div className="btn btn-link text-sm">View Published Template</div>
        ) : null}
        <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-5">
          <div>Template Name:</div>
          <div>{flow_name}</div>
          <div>Template Description:</div>
          <textarea
            // type="text"
            placeholder="Type here"
            className="w-full h-32 textarea textarea-bordered textarea-md"
            defaultValue={""}
            // value={description}
            // onChange={(e) => setDescription(e.target.value)}
            //TODO: wire in flowFrontmatter when its live
            // defaultValue={flowFrontmatter ? flowFrontmatter.description || ""}
            {...register("description", { required: true })}
          />
          {errors.description?.type === "required" && (
            <div className="alert alert-error   ">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="stroke-current shrink-0 h-6 w-6"
                fill="none"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <span>Description is required</span>
            </div>
            // <p className="flex mb-1 h-full justify-center items-center pl-4">
            //   Description is required
            // </p>
          )}
          <div>URL Preview:</div>
          <div>{`tryanything.xyz/templates/${slugify(flow_name)}`}</div>
          {saving ? (
            "Publishing..."
          ) : (
            <button
              className="btn btn-primary"
              type="submit"
              // type="submit"
              // disabled={!}
            >
              Publish Template
            </button>
          )}
        </form>
        {/* <button
          className="btn btn-secondary"
          onClick={_saveTemplate}
          disabled={saving}
        >
          {saving ? "Publishing..." : "Publish Template"}
        </button> */}
        {/* TODO: */}
        {/* <div>Choose Tags</div> */}
      </div>
    </RequireAuth>
  );
};

export default FlowSharingPanel;
