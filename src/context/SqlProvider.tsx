import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";
import { invoke } from "@tauri-apps/api";
import { v4 as uuidv4 } from "uuid";

const DB_STRING = "sqlite:test.db";

export type EventInput = {
  flow_id: string; //flow needs a computer friendly name that can be changed without changing processing
  flow_name: string; //flow needs a user friendly name
  flow_version: string; //flows will have versions so you can have confidence messing arround in future
  node_id: string; //represents exact_id inside a flow
  node_type: string; //represents front_end representation of node
  worker_type: string; //worker type === "start" or "javascript interpreter" or "rest" etc
  stage: string;
  event_status: string;
  session_status: string;
  created_at: string;
  data: any;
};
//Load Database once
invoke("plugin:sqlite|load");

interface SqlContextInterface {
  tables: any[];
  addEvent: (event: EventInput) => void;
  getTableData: (tableName: string) => any;
  getFlowEvents: (flowName: string) => any;
}

export const SqlContext = createContext<SqlContextInterface>({
  tables: [],
  addEvent: () => {},
  getTableData: () => {},
  getFlowEvents: () => {},
});

export const useSqlContext = () => useContext(SqlContext);

export const SqlProvider = ({ children }: { children: ReactNode }) => {
  const [tables, setTables] = useState<any[]>([]);

  const db = {
    execute: async (query: string, values?: any[]) => {
      console.log("Executing Sql on JS side", query, values);
      return await invoke("plugin:sqlite|execute", {
        db: DB_STRING,
        query,
        values: values ?? [],
      });
    },
    select: async (query: string, values?: any[]) => {
      console.log("Selecting Sql on JS side", query, values);
      return await invoke("plugin:sqlite|select", {
        db: DB_STRING,
        query,
        values: values ?? [],
      });
    },
  };

  const addEvent = async (event: EventInput) => {
    try {
      await db.execute(
        "INSERT INTO events (event_id, session_id, node_id, node_type, flow_id, flow_name, flow_version, stage,worker_type, event_status, session_status, created_at, data) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)",
        [
          uuidv4(),
          uuidv4(),
          event.node_id,
          event.node_type,
          event.flow_id,
          event.flow_name,
          event.flow_version,
          event.stage,
          event.worker_type,
          event.event_status,
          event.session_status,
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
    // return [];
  };

  const getFlowEvents = async (flowName: string) => {
    const flowEvents = await db.select(
      `SELECT * FROM events WHERE flow_name = $1 AND event_status = 'PENDING'`,
      [flowName]
    );
    console.log("flowEvents in db", flowEvents);
    return flowEvents;
  };

  const initDb = async () => {
    try {
      await db.execute(`CREATE TABLE IF NOT EXISTS events (
      event_id TEXT PRIMARY KEY,
      session_id TEXT,
      node_id TEXT,
      node_type TEXT,
      flow_id TEXT,
      flow_name TEXT,
      flow_version TEXT,
      worker_type TEXT,
      stage TEXT,
      event_status TEXT,
      session_status TEXT,
      created_at DATETIME,
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
      value={{ addEvent, tables, getTableData, getFlowEvents }}
    >
      {children}
    </SqlContext.Provider>
  );
};
