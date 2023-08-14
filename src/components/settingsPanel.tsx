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
  const { deleteFlow, renameFlow } = useLocalFileContext();
  const { flow_name } = useParams();
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = (data) => {
    setLoading(true);
    if (flow_name && data.flow_name != flow_name) {
      renameFlow(flow_name, data.flow_name);
      //wait for 2 seconds
      setTimeout(() => {
        navigate(`/flows/${data.flow_name}`);
        setLoading(false);
      }, 2000);
    }
    console.log(data);
    setLoading(false);
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
        onClick={() => {
          if (flow_name) {
            deleteFlow(flow_name);
            navigate("/");
          }
        }}
      >
        Delete Flow
      </button>
    </div>
  );
};

export default FlowSettingsPanel;
