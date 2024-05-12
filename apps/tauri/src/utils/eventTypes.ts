export type ProcessingStatus = {
    message: string;
    event_id: string;
    node_id: string;
    flow_id: string;
    session_id: string;
  };
  
  export type SessionComplete = {
    event_id: string;
    node_id: string;
    flow_id: string;
    session_id: string;
  };
