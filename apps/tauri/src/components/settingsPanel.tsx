import { useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { useNavigate, useParams } from "react-router-dom";

import { useFlowContext } from "../context/FlowProvider";
import { useFlowsContext } from "../context/FlowsProvider";
import { FormError } from "./formError";

type Inputs = {
  flow_name: string;
};

type DeleteInputs = {
  flow_name: string;
};

const DeleteModal = ({
  flow_name,
  flow_id,
}: {
  flow_name: string;
  flow_id: string;
}) => {
  const { deleteFlow } = useFlowsContext();
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<DeleteInputs>();

  const onSubmitDelete = async () => {
    if (flow_id) {
      await deleteFlow(flow_id);
      navigate("/");
    }
  };
  return (
    <>
      <button
        className="btn btn-error"
        onClick={() =>
          (
            document.getElementById("my_modal_1") as HTMLDialogElement
          ).showModal()
        }
      >
        Delete Flow
      </button>
      {/* Modal */}
      <dialog id="my_modal_1" className="modal">
        <div className="modal-box">
          <form
            onSubmit={handleSubmit(onSubmitDelete)}
            className="flex flex-col flex-grow gap-5"
          >
            <h3 className="font-bold text-lg">Are you sure?</h3>
            <p className="py-4">
              Flow will be permantly deleted including all past versions and
              events.
            </p>
            <p>Input flow name to confirm deletion</p>
            <label className="label">
              <span className="label-text">Flow Name</span>
            </label>
            <input
              type="text"
              placeholder="Type here"
              className="input input-bordered input-md w-full"
              {...register("flow_name", {
                required: true,
                validate: (value) => value === flow_name,
              })}
            />
            {errors.flow_name && <FormError error={"Does Not Match"} />}
            <input type="submit" className="btn btn-primary" />
          </form>
          <form method="dialog" className="mt-2">
            <button className="btn w-full">Cancel</button>
          </form>
        </div>
      </dialog>
    </>
  );
};

const FlowSettingsPanel = () => {
  const [loading, setLoading] = useState(false);
  const { updateFlow } = useFlowsContext();
  const { flowFrontmatter } = useFlowContext();

  const { flow_name } = useParams();

  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      if (flow_name && flowFrontmatter) {
        let UpdateFlowArgs = {
          flow_name: data.flow_name,
          active: flowFrontmatter.active,
          version: flowFrontmatter.version,
        };

        console.log(
          "Updating Flow In Settings Panel with Args",
          UpdateFlowArgs
        );
        let res = await updateFlow(flowFrontmatter.flow_id, UpdateFlowArgs);
        console.log("res from rename flow in settings panel", res);
        navigate(`/flows/${data.flow_name}`);
      } else {
        console.log("Data problem in settings panel");
      }
    } catch (error) {
      console.log("error in settings panel", error);
    } finally {
      console.log(data);
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-full gap-5 p-4">
      <h1 className="text-2xl font-bold">Flow Settings</h1>
      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex flex-col flex-grow gap-5"
      >
        <label className="label">
          <span className="label-text">Flow Name</span>
        </label>
        <input
          type="text"
          placeholder="Type here"
          className="input input-bordered input-md w-full"
          defaultValue={flow_name}
          {...register("flow_name", { required: true })}
        />
        {errors.flow_name && <FormError error={"Required"} />}
        <input type="submit" className="btn btn-primary" />
      </form>
      <DeleteModal flow_name={flow_name} flow_id={flowFrontmatter.flow_id} />
    </div>
  );
};

export default FlowSettingsPanel;
