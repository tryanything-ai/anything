import ReactFlow, {
  Controls,
  ControlButton,
  ControlButtonProps,
} from "reactflow";

import { AiOutlineCode } from "react-icons/ai";

export default function CustomControls() {
  return (
    <Controls style={{ background: "darkgray" }}>
      <ControlButton onClick={() => console.log("action")} title="action">
        <AiOutlineCode className=" text-black" />
      </ControlButton>
    </Controls>
  );
}
