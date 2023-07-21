import ReactFlow, {
  Controls,
  ControlButton,
  ControlButtonProps,
} from "reactflow";

import { AiOutlineCode } from "react-icons/ai";
import { Outlet, Link, useLocation } from "react-router-dom";
export default function CustomControls() {
  return (
    <Controls style={{ background: "darkgray" }}>
      <Link className="text-black" to="/toml">
        <ControlButton onClick={() => console.log("action")} title="action">
          <AiOutlineCode className=" text-black" />
        </ControlButton>
      </Link>
    </Controls>
  );
}
