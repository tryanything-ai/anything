import { useState } from "react";
import { useForm, SubmitHandler } from "react-hook-form";
import { useParams } from "react-router-dom";
import api from "../tauri_api/api";

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
  const { flow_id } = useParams();

  //TODO: switch this over to event model not chatting directly
  const prompt = async (message: string) => {
    setMessages((messages) => [
      ...messages,
      { message: message, from: "User" },
    ]);
    let unlisten = api.subscribeToEvent("prompt_response", (event) => {
      // if (event.payload?.message?.length > 0) onToken(event.payload.message);
      console.log("prompt_response event received");
      console.debug(event);
      if (event.payload.message) {
        setMessages((messages) => [
          ...messages,
          { message: event.payload.message, from: "AI" },
        ]);
      }
    });
    console.log("prompt sent: " + message);

    api.sendPrompt({ message });
  };

  const {
    register,
    handleSubmit,
    watch,
    reset,
    formState: { errors },
  } = useForm<Inputs>();

  const onSubmit: SubmitHandler<Inputs> = async (data) => {
    try {
      setLoading(true);
      await prompt(data.message);
      reset();
    } catch (error) {
      console.log(error);
    } finally {
      console.log(data);
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-full p-4 border-gray-500 gap-5">
      <h1 className="text-2xl font-bold text-center">{flow_id}</h1>
      <div className="flex-grow p-4">
        {messages.map((message) => {
          return (
            <div key={message.message} className="flex flex-col gap-1">
              <div className="text-sm text-gray-500">{message.from}</div>
              <div className="text-lg">{message.message}</div>
            </div>
          );
        })}
      </div>
      <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-2">
        <div className="relative">
          <input
            type="text"
            placeholder="Send a message"
            className="input input-bordered input-md w-full pr-12"
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
