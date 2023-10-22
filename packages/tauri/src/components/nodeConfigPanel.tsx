import { useEffect,useState } from "react";
import { Controller,useForm } from "react-hook-form";
import { useParams } from "react-router-dom";

import { useFlowNavigationContext } from "../context/FlowNavigationProvider";
import { useLocalFileContext } from "../context/LocalFileProvider";

const NodeConfigPanel = () => {
  const { nodeId, setNodeConfigPanel } = useFlowNavigationContext();
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
    if (!flow_name) return;
    if (!nodeId) return;
    console.log("Hit Node Config Submit");
    console.log(data);

    writeNodeConfig(nodeId, flow_name, data);
    setNodeConfigPanel(false, "");
  };

  return (
    <div className="flex flex-col h-full overflow-y-auto p-4">
      <h1 className="text-2xl font-bold">Node Settings</h1>
      {nodeId ? (
        <form
          className="flex flex-col gap-4 p-4"
          onSubmit={handleSubmit(onSubmit)}
        >
          {Object.keys(data).map((key, index) => {
            const value = data[key];

            if (typeof value === "string" || typeof value === "number") {
              return (
                <div key={key}>
                  <div className="mb-1">{key}:</div>
                  <input
                    type="text"
                    className="input input-bordered input-md w-full"
                    defaultValue={value}
                    {...register(key)}
                  />
                  {errors[key] && (
                    <span>{JSON.stringify(errors[key]?.message)}</span>
                  )}
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
              );
            }
            // Extend this to handle other types as needed
          })}

          <button className="mt-2 btn btn-primary" type="submit">
            Save
          </button>
        </form>
      ) : (
        <div>Select a node to configure</div>
      )}
    </div>
  );
};

export default NodeConfigPanel;
