import { useState, useEffect } from "react";
import { useSqlContext } from "../context/SqlProvider";
import { useParams } from "react-router-dom";
import { useNavigationContext } from "../context/NavigationProvider";
import { VscClose } from "react-icons/vsc";
import { useLocalFileContext } from "../context/LocalFileProvider";
import { useForm, Controller } from "react-hook-form";

const NodeConfigPanel = () => {
  const { nodeId, setNodeConfigPanel } = useNavigationContext();
  const { readNodeConfig, writeNodeConfig } = useLocalFileContext();

  const { flow_name } = useParams();
  const [data, setData] = useState<any>({});

  const hydrate = async () => {
    try {
      if (!flow_name) return;
      if (!nodeId) return;
      const data: any = await readNodeConfig(nodeId, flow_name);

      Object.keys(data.data).forEach((key) => {
        setValue(key, data.data[key]);
      });

      setData(data.data);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();
  }, []);

  const {
    register,
    handleSubmit,
    setValue,
    control,
    formState: { errors },
  } = useForm();

  const onSubmit = (data: any) => {
    console.log(data);
  };

  return (
    <div className="flex flex-col h-full border-l border-gray-500">
      <button
        className="m-1 btn btn-ghost btn-square btn-xs w-6 h-6 absolute right-0"
        onClick={() => setNodeConfigPanel(false, "")}
      >
        <VscClose className="h-6 w-6" />
      </button>
      <h1 className="p-4 text-2xl font-bold">Node Settings</h1>
      <form
        className="flex flex-col gap-4 p-4"
        onSubmit={handleSubmit(onSubmit)}
      >
        {Object.keys(data).map((key, index) => {
          const value = data[key];

          if (typeof value === "string" || typeof value === "number") {
            return (
              <div>
                <div className="mb-1">{key}:</div>
                <input
                  type="text"
                  placeholder="Type here"
                  className="input input-bordered input-md w-full"
                  defaultValue={flow_name}
                  {...register(key, { required: true })}
                />
              </div>
            );
          } else if (typeof value === "boolean") {
            return (
              <Controller
                key={index}
                name={key}
                control={control}
                defaultValue={value}
                render={({ field }) => (
                  <label>
                    {key}:
                    <input
                      className="toggle toggle-success"
                      type="checkbox"
                      {...field}
                      checked={field.value}
                    />
                  </label>
                )}
              />
          )
          }
          // Extend this to handle other types as needed
        })}
        <button className="mt-2 btn btn-primary" type="submit">
          Submit
        </button>
      </form>
    </div>
  );
};

export default NodeConfigPanel;
