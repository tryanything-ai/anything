import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useForm, SubmitHandler } from "react-hook-form";
import { useParams, useNavigate } from "react-router-dom";
import kbd from "daisyui";

type Inputs = {
  flow_name: string;
};

const ChatInterface = () => {
  const [loading, setLoading] = useState(false);
  const { flow_id } = useParams();

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      // if (flow_name && data.flow_name != flow_name) {
      //   await updateFlowFrontmatter(flow_name, { name: data.flow_name });
      //   navigate(`/flows/${data.flow_name}`);
      // }
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
      <h1 className="text-2xl font-bold text-center">{flow_id}</h1>
      <div className="flex-grow ">Mesages go here</div>
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
        <div className="relative">
          <input
            type="text"
            placeholder="Send a message"
            className="input input-bordered input-md w-full  pr-12"
            // defaultValue={flow_name}
            {...register("flow_name", { required: true })}
          />
          <kbd className="absolute inset-y-0 right-3 top-1/2 transform -translate-y-1/2">
            Enter
          </kbd>
        </div>
        <div className="text-sm text-center">.. Here be dragons ..</div>
        {/* register your input into the hook by invoking the "register" function */}
        {/* <input defaultValue={flow_name} {...register("flow_name", { required: true})} /> */}
        {errors.flow_name && <span>This field is required</span>}
        {/* <input type="submit" className="btn btn-primary" /> */}
      </form>
      {/* <button
        className="btn btn-error mt-4"
        // onClick={_deleteFlow}
      >
        Delete Flow
      </button> */}
    </div>
  );
};

export default ChatInterface;
