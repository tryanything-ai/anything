import {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

import api from "../tauri_api/api";
import { v4 as uuidv4 } from "uuid";

const DB_STRING = "sqlite:test.db";

export type EventInput = {
  flow_id: string; //flow needs a computer friendly name that can be changed without changing processing
  flow_name: string; //flow needs a user friendly name
  flow_version: string; //flows will have versions so you can have confidence messing arround in future
  node_id: string; //represents exact_id inside a flow
  node_type: string; //represents front_end representation of node
  node_label: string; //what the user will see in the flow
  worker_type: string; //worker type === "start" or "javascript interpreter" or "rest" etc
  worker_name: string; //what the user will use to reference the node in props for args. needs to be snake_case
  stage: string;
  event_status: string;
  session_status: string;
  created_at: string;
  data: any;
};

//Load Database once
api.loadSqlLite();

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

  const db = {
    execute: async (query: string, values?: any[]) => {
      // console.log("Executing Sql on JS side", query, values);
      return await await api.executeSqlLite({
        db: DB_STRING,
        query,
        values: values ?? [],
      });
    },
    select: async (query: string, values?: any[]): Promise<any> => {
      // console.log("Selecting Sql on JS side", query, values);
      return await api.selectSqlLite({
        db: DB_STRING,
        query,
        values: values ?? [],
      });
    },
  };

  const addEvent = async (event: EventInput) => {
    try {
      //TODO: implement in rust. this does not conform exactly to event_context and other things usually shaped in rust
      await db.execute(
        "INSERT INTO events (event_id, session_id, node_id, node_type, node_label, flow_id, flow_name, flow_version, stage, worker_type, worker_name, event_status, session_status, event_context, created_at, data) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)",
        [
          uuidv4(),
          uuidv4(),
          event.node_id,
          event.node_type,
          event.node_label,
          event.flow_id,
          event.flow_name,
          event.flow_version,
          event.stage,
          event.worker_type,
          event.worker_name,
          event.event_status,
          event.session_status,
          event, //context
          event.created_at,
          event.data,
        ]
      );
    } catch (error) {
      console.log("error adding event to db", error);
    }
  };

  const getTables = async () => {
    const tables = await db.select(
      `SELECT name
      FROM sqlite_master
      WHERE type='table' AND name NOT LIKE 'sqlite_%'
      `
    );
    console.log("tables in db", tables);
    setTables(tables as any);
  };

  const getTableData = async (tableName: string) => {
    const tableData = await db.select(`SELECT * FROM ${tableName}`);
    console.log("tableData in db", tableData);
    return tableData;
  };

  const getSessionEvents = async (flowName: string, session_id: string) => {
    const flowEvents = await db.select(
      `SELECT * FROM events WHERE flow_name = $1 AND session_id = $2 ORDER BY created_at ASC;`,
      [flowName, session_id]
    );
    return flowEvents;
  };

  const getEvent = async (event_id: string) => {
    const event = await db.select(`SELECT * FROM events WHERE event_id = $1;`, [
      event_id,
    ]);
    return event[0];
  };

  const initDb = async () => {
    try {
      await db.execute(`CREATE TABLE IF NOT EXISTS events (
      event_id TEXT PRIMARY KEY,
      session_id TEXT,
      node_id TEXT,
      node_type TEXT,
      node_label TEXT, 
      flow_id TEXT,
      flow_name TEXT,
      flow_version TEXT,
      worker_type TEXT,
      worker_name TEXT,
      stage TEXT,
      event_status TEXT,
      session_status TEXT,
      created_at DATETIME,
      event_result TEXT,
      event_context TEXT,
      data TEXT
      )`);
    } catch (error) {
      console.log("error creating events table", error);
    }
  };

  useEffect(() => {
    const go = async () => {
      await initDb();
      await getTables();
    };
    go();
  }, []);

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
