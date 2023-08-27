import { ReactNode, useEffect, useState } from "react";
import { useFlowContext } from "../../context/FlowProvider";
import clsx from "clsx";

export default function BaseNode({
  children,
  id,
  flow_id,
}: {
  children: ReactNode;
  id: string;
  flow_id: string;
}) {
  const { currentProcessingStatus } = useFlowContext();
  const [processing, setProcessing] = useState(false);

  let animated_styles =
    "animate-border bg-white bg-gradient-to-r from-red-500 via-purple-500 to-blue-500 bg-[length:600%_600%]";
  //"animate-border inline-block rounded-md bg-white bg-gradient-to-r from-red-500 via-purple-500 to-blue-500 bg-[length:400%_400%] p-1"
  useEffect(() => {
    //FIXME: manage flow_name by ID globally here we need it to only show activity if the activity is from the right flow
    console.log("now: Processing set to true in node", currentProcessingStatus);
    console.log("now: Data id", id);
    if (currentProcessingStatus && currentProcessingStatus?.node_id === id) {
      setProcessing(true);
    } else {
      setProcessing(false);
    }
  }, [currentProcessingStatus]);

  return (
    <div
      className={clsx(
        "bg-primary border min-w-60 min-h-20 rounded-md inline-block ",
        {
          [animated_styles]: processing,
        }
      )}
    >
      <div className="m-2 rounded-md p-4 text-primary-content flex flex-col justify-center align-middle">
        {children}
      </div>
    </div>

    // <div className="bg-secondary w-60 h-20 p-4 border rounded-md text-primary-content flex flex-col justify-center align-middle">
    //   {processing ? (
    //     <div className=" bg-secondary rounded-full w-7 h-7 absolute top-0 right-0 transform translate-x-1/2 -translate-y-1/2 flex items-center justify-center p-0.5 overflow-hidden shadow z-10">
    //       <span className="loading loading-spinner text-accent"></span>
    //     </div>
    //   ) : null}

    //   {children}
    // </div>
    // <div className="animate-border inline-block rounded-md bg-white bg-gradient-to-r from-red-500 via-purple-500 to-blue-500 bg-[length:400%_400%] p-1">
    //   {children}
    // </div>
  );
}
