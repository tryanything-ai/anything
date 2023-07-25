import React, {
    useCallback,
    useEffect,
    useMemo,
    useState,
    useRef,
  } from "react";
  import ReactFlow, {
    Background,
    useNodesState,
    useEdgesState,
    addEdge,
    Handle,
    Position,
    BackgroundVariant,
    Controls,
} from "reactflow";
  

//Node that acts as the beginning of a flow or one of many beginnings of a flow
function InputNode({ data }: { data: any }) {

    //Needs to fetch or receivein some way the data to be worked on. 
    //Base case. Receive a message.
    //It should also give some context of whats going on
    //IDEA:: slow mode?
    const onChange = useCallback((evt: any) => {
      console.log(evt.target.value);
    }, []);
  
    return (
      <div
        className={
          "bg-secondary w-64 h-12 border rounded-md text-white flex flex-col justify-center align-middle" +
          data.classNames
        }
      >
        <div className="text-center text-xl">{data.value}</div>
  
        <Handle type="source" position={Position.Bottom} id="a" />
      </div>
    );
  }