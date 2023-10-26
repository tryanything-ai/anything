import { createContext, ReactNode,useContext, useState } from "react";

import api from "../tauri_api/api";
import { EventInput } from "../tauri_api/types";

interface SqlContextInterface {
  tables: any[];
  addEvent: (event: EventInput) => void;
  getTableData: (tableName: string) => any;
  getSessionEvents: (flowName: string, session_id: string) => any;
  getEvent: (event_id: string) => any;
}

export const SqlContext = createContext<SqlContextInterface>({
  tables: [],
  addEvent: () => {},
  getTableData: () => {},
  getSessionEvents: () => {},
  getEvent: () => {},
});

export const useSqlContext = () => useContext(SqlContext);

export const SqlProvider = ({ children }: { children: ReactNode }) => {
  const [tables, setTables] = useState<any[]>([]);

  const addEvent = async (event: EventInput) => {
    try {
      await api.createEvent({ ...event });
      //MIGRATE_TO_RUST
      // await api.db.execute(
      //   "INSERT INTO events (event_id, session_id, node_id, node_type, node_label, flow_id, flow_name, flow_version, stage, worker_type, worker_name, event_status, session_status, event_context, created_at, data) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
      //   [
      //     uuidv4(),
      //     uuidv4(),
      //     event.node_id,
      //     event.node_type,
      //     event.node_label,
      //     event.flow_id,
      //     event.flow_name,
      //     event.flow_version,
      //     event.stage,
      //     event.worker_type,
      //     event.worker_name,
      //     event.event_status,
      //     event.session_status,
      //     event, //context
      //     event.created_at,
      //     event.data,
      //   ]
      // );
    } catch (error) {
      console.log("error adding event to db", error);
    }
  };

  const getTables = async () => {
    const tables = await api.db.select(
      `SELECT name
      FROM sqlite_master
      WHERE type='table' AND name NOT LIKE 'sqlite_%'
      `
    );
    console.log("tables in db", tables);
    setTables(tables as any);
  };

  const getTableData = async (tableName: string) => {
    const tableData = await api.db.select(`SELECT * FROM ${tableName}`);
    console.log("tableData in db", tableData);
    return tableData;
  };

  const getSessionEvents = async (flowName: string, session_id: string) => {
    const flowEvents = await api.db.select(
      `SELECT * FROM events WHERE flow_name = $1 AND session_id = $2 ORDER BY created_at ASC;`,
      [flowName, session_id]
    );
    return flowEvents;
  };

  const getEvent = async (event_id: string) => {
    const event = await api.db.select(
      `SELECT * FROM events WHERE event_id = $1;`,
      [event_id]
    );
    return event[0];
  };

  // useEffect(() => {
  //   const go = async () => {
  //     await getTables();
  //   };
  //   go();
  // }, []);

  return (
    <SqlContext.Provider
      value={{
        addEvent,
        tables,
        getTableData,
        getSessionEvents,
        getEvent,
      }}
    >
      {children}
    </SqlContext.Provider>
  );
};
