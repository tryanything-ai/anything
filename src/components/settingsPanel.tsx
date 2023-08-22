import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useForm, SubmitHandler } from "react-hook-form";
import { useParams, useNavigate } from "react-router-dom";

type Inputs = {
  flow_name: string;
};

const FlowSettingsPanel = () => {
  const [loading, setLoading] = useState(false);
  const { deleteFlow } = useLocalFileContext();
  const { updateFlowFrontmatter } = useFlowContext(); 
  const { flow_name } = useParams();
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  const _deleteFlow = async () => {
      if (flow_name) {
        await deleteFlow(flow_name);
        navigate("/");
      }
    }
  
  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      if (flow_name && data.flow_name != flow_name) {
        await updateFlowFrontmatter(flow_name, { name: data.flow_name }); 
        navigate(`/flows/${data.flow_name}`);
      }
    } catch (error) {
      console.log(error);
    } finally {
      console.log(data);
      setLoading(false);
    }
  };

  // console.log(watch("example")); // watch input value
  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500 gap-5">
      <h1 className="text-2xl font-bold">Flow Settings</h1>

      <form
        onSubmit={handleSubmit(onSubmit)}
        className="flex flex-col flex-grow gap-5"
      >
        <input
          type="text"
          placeholder="Type here"
          className="input input-bordered input-md w-full"
          defaultValue={flow_name}
          {...register("flow_name", { required: true })}
        />

        {/* register your input into the hook by invoking the "register" function */}
        {/* <input defaultValue={flow_name} {...register("flow_name", { required: true})} /> */}
        {errors.flow_name && <span>This field is required</span>}
        <input type="submit" className="btn btn-primary" />
      </form>
      <button
        className="btn btn-error mt-4"
        onClick={_deleteFlow}
      >
        Delete Flow
      </button>
    </div>
  );
};

export default FlowSettingsPanel;
