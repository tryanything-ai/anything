import { useState } from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import { useParams } from "react-router-dom";
import { invoke } from "@tauri-apps/api";
import { useEventLoopContext } from "../context/EventLoopProvider";

type Inputs = {
  message: string;
};

type Message = {
  message: string;
  from: string;
};

const ChatInterface = () => {
  const [loading, setLoading] = useState(false);
  const [messages, setMessages] = useState<Message[]>([]);
  const { subscribeToEvent } = useEventLoopContext();
  const { flow_id } = useParams();

  const prompt = async (message: string) => {
    subscribeToEvent("prompt_response", (event) => {
      // if (event.payload?.message?.length > 0) onToken(event.payload.message);
      console.log("prompt_response event received");
      console.debug(event);
      if (event.payload.message) {
        // setMessages((messages) => [...messages, event.payload]);
      }
    });
    console.log("prompt sent: " + message);
    invoke("prompt", { message });
  };

  const {
    register,
    handleSubmit,
    watch,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      await prompt(data.message);

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
    <div className="flex flex-col h-full p-4 border-gray-500 gap-5">
      <h1 className="text-2xl font-bold text-center">{flow_id}</h1>
      <div className="flex-grow ">Mesages go here</div>
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
        <div className="relative">
          <input
            type="text"
            placeholder="Send a message"
            className="input input-bordered input-md w-full  pr-12"
            {...register("message", { required: true })}
          />
          <kbd className="absolute inset-y-0 right-3 top-1/2 transform -translate-y-1/2">
            Enter
          </kbd>
        </div>
        <div className="text-sm text-center">.. Here be dragons ..</div>
        {errors.message && (
          <span className="text-sm text-center">This field is required</span>
        )}
      </form>
    </div>
  );
};

export default ChatInterface;
