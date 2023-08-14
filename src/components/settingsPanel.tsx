import { useEffect, useState } from "react";
import { useFlowContext } from "../context/FlowProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useForm, SubmitHandler } from "react-hook-form";
import { useParams, useNavigate} from "react-router-dom";

type Inputs = {
  example: string;
  exampleRequired: string;
};

const FlowSettingsPanel = () => {
  const { deleteFlow } = useLocalFileContext();
  const { flow_name } = useParams();
  const navigate = useNavigate();
  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = (data) => console.log(data);

  // console.log(watch("example")); // watch input value
  return (
    <div className="flex flex-col h-full p-4 border-l border-gray-500">
      <h1 className="text-2xl font-bold">Setttings Form</h1>

      <form onSubmit={handleSubmit(onSubmit)} className="flex-grow">
        {/* register your input into the hook by invoking the "register" function */}
        <input defaultValue="test" {...register("example")} />
        {/* include validation with required or other standard HTML validation rules */}
        <input {...register("exampleRequired", { required: true })} />
        {/* errors will return when field validation fails  */}
        {errors.exampleRequired && <span>This field is required</span>}
        <input type="submit" />
      </form>
      <button
        className="btn btn-error mt-4"
        onClick={() => { if (flow_name) { deleteFlow(flow_name); navigate("/")} }}
      >
        Delete Flow
      </button>
    </div>
  );
};

export default FlowSettingsPanel;
