import { useParams } from "react-router-dom";

const FlowName = () => {
  const { flow_name } = useParams();

  return (
    <div className="bg-white z-10 absolute top-0 left-0 h-11 m-5 rounded-md flex flex-row p-1 items-center bg-opacity-5">
      <div className="text-xl mx-2">{flow_name}</div>
    </div>
  );
};

export default FlowName;
