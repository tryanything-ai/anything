import { BaseNodeIcon, SvgRenderer } from "ui";
import { VscPlayCircle } from "../utils/rawIcons";

const StartButton = () => {
  return (
    <div className="bg-white z-10 hover:bg-success hover:text-success-content absolute top-0 right-0 w-11 h-11 m-5 rounded-md flex flex-row p-1 bg-opacity-5 start-button">
      <div className="bg-blue-200 h-8 w-12">
        <SvgRenderer
          className="w-full h-full bg-pink-400 opaciity-100"
          icon={VscPlayCircle}
        />
      </div>
      {/* <BaseNodeIcon className="h-9 w-9" icon={VscPlayCircle} /> */}
      <div className="pl-2 h-full w-full pt-1 text-xl start-text">Start</div>
    </div>
  );
};

export default StartButton;
