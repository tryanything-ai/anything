import { useParams } from "react-router-dom";
import { VscRepoForked, VscCode, VscComment, VscDebug } from "react-icons/vsc";
import { useNavigationContext } from "../context/NavigationProvider";

export default function Header() {
  const {
    nodePanel,
    setNodePanel,
    tomlPanel,
    setTomlPanel,
    chatPanel,
    setChatPanel,
  } = useNavigationContext();

  const { flow_name } = useParams();

  return (
    <div className="w-full z-10 bg-primary pl-2 text-white overflow-hidden">
      <div className="flex flex-row">
        <div className="">flows/{flow_name}</div>
        <div className="flex-grow" />
        <button onClick={() => setNodePanel(!nodePanel)}>
          <VscRepoForked className="mr-2 h-5 w-5" />
        </button>
        <button onClick={() => setChatPanel(!chatPanel)}>
          <VscDebug className="mr-2 h-4 w-5" />
        </button>
        <button onClick={() => setTomlPanel(!tomlPanel)}>
          <VscCode className="mr-2 h-5 w-5" />
        </button>
      </div>
    </div>
  );
}
