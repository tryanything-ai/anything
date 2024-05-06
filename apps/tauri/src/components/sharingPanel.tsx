import { useEffect, useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { Link, useNavigate, useParams } from "react-router-dom";

import { useFlowContext } from "../context/FlowProvider";
import RequireAuth from "./RequireAuth";
import slugify from "slugify";

import { BigFlow } from "utils";
import { useMarketplaceContext } from "../context/MarketplaceProvider";
import { FormError } from "./formError";

type Inputs = {
  flow_name: string;
};



const FlowSharingPanel = () => {
  const [loading, setLoading] = useState(false);
  const [saving, setSaving] = useState(false);
  const [publishedFlow, setPublishedFlow] = useState<BigFlow>(null);
  // does a template and a version already exist.
  // const [publishedTemplate, setPublishedTemplate] = useState(false);

  const { flowFrontmatter, getFlowDefinitionsFromReactFlowState } =
    useFlowContext();
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

  //TODO: publish new flow_version if flow exist

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    // const _saveTemplate = async () => {
    try {
      setSaving(true);

      let flowDefinition = getFlowDefinitionsFromReactFlowState();
      //TODO: validate inputs
      //TODO: show errors
      if (flow_name && data.description) {
        console.log("Saving");
        let res = await saveTemplate(
          flowFrontmatter.flow_id,
          flowFrontmatter.flow_version_id,
          flow_name,
          data.description,
          flowDefinition
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
      console.log("Fetching Template by ID:", flowFrontmatter.flow_id);
      let res = await fetchTemplateById(flowFrontmatter.flow_id);
      if (res && res[0]) {
        console.log("Fetched Template by ID Response:", res);
        setPublishedFlow(res);
      }
    }
  };

  //TODO:we shoudl update flowFrontmatter when we update description so we might need to change this
  useEffect(() => {
    _fetchTemplateById();
  }, []);

  return (
    <RequireAuth>
      <div className="flex flex-col h-full gap-5 p-4">
        <h1 className="text-2xl font-bold">Flow Sharing</h1>
        {/* View Published */}
        {publishedFlow ? (
          // <Link className="btn btn-link text-sm"  target="_blank" to={`${import.meta.env.VITE_PUBLIC_HOSTED_URL}/${slugify(flow_name)}`}
          // rel="noopener noreferrer">View Published Template</Link>
          // <div>View Online</div>
          <div> Flow Already Published </div>
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
            <FormError error="Description is required" />
            // <p className="flex mb-1 h-full justify-center items-center pl-4">
            //   Description is required
            // </p>
          )}
          <div>URL Preview:</div>
          <div>{`tryanything.xyz/templates/${slugify(flow_name, {
            lower: true,
          })}`}</div>
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
