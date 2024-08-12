import { ArrowBigLeft, Play } from "lucide-react";

const NoNodeSelected = () => {
    return (
      <div className="flex flex-col justify-center items-center h-96 w-full">
        <div className="flex flex-row mt-auto mb-auto text-center border-2 border-dashed rounded-md p-4">
          <div className="flex flex-col justify-center items-center mr-2">
            <ArrowBigLeft size={36} />
          </div>
          <div className="text-xl font-normal">
            <div>Select a node</div>
            <div>to configure</div>
          </div>
        </div>
      </div>
    );
  };

export default NoNodeSelected;