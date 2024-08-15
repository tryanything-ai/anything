import { useAnything } from "@/context/AnythingContext";
import {
  BaseEdge,
  EdgeLabelRenderer,
  getSimpleBezierPath,
  getSmoothStepPath,
  getStraightPath,
  useReactFlow,
  Edge,
  EdgeProps,
} from "reactflow";

export default function CustomEdge({
  id,
  sourceX,
  sourceY,
  targetX,
  targetY,
}: EdgeProps): JSX.Element {
  const { setEdges } = useReactFlow();
  const { workflow } = useAnything();

  const [edgePath, labelX, labelY] = getSimpleBezierPath({
    sourceX,
    sourceY,
    targetX,
    targetY,
  });

  return (
    <>
      <BaseEdge id={id} path={edgePath} />
      <EdgeLabelRenderer>
        <button
          style={{
            position: "absolute",
            transform: `translate(-50%, -50%) translate(${labelX}px,${labelY}px)`,
            pointerEvents: "all",
          }}
          className="nodrag nopan h-6 w-6 hover:bg-green-500 bg-gray-300 rounded-xl text-white font-bold text-md"
          onClick={() => {
            // setEdges((es) => es.filter((e) => e.id !== id));
            workflow.showActionSheetForEdge(id);
          }}
        >
          +
        </button>
      </EdgeLabelRenderer>
    </>
  );
}
