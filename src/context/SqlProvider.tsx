import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";
import { invoke } from "@tauri-apps/api";

const DB_STRING = "sqlite:test.db";
//Load Database once
invoke("plugin:sqlite|load");

interface SqlContextInterface {
  tables: any[];
  addEvent: (
    event_id: string,
    flow_id: string,
    flow_name: string,
    flow_version: string,
    stage: string,
    status: string,
    created_at: string,
    data: any
  ) => void;
  getTableData: (tableName: string) => any;
}

export const SqlContext = createContext<SqlContextInterface>({
  tables: [],
  addEvent: () => {},
  getTableData: () => {},
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

  const addEvent = async (
    event_id: string,
    flow_id: string,
    flow_name: string,
    flow_version: string,
    stage: string,
    status: string,
    created_at: string,
    data: any
  ) => {
    // return true;
    try {
      await db.execute(
        "INSERT INTO events (event_id, flow_id, flow_name, flow_version, stage, status, created_at, data) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        [
          event_id,
          flow_id,
          flow_name,
          flow_version,
          stage,
          status,
          created_at,
          data,
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

  const initDb = async () => {
    try {
      // return true;
      await db.execute(`CREATE TABLE IF NOT EXISTS events (
      event_id TEXT PRIMARY KEY,
      flow_id TEXT,
      flow_name TEXT,
      flow_version TEXT,
      stage TEXT,
      status TEXT,
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
    <SqlContext.Provider value={{ addEvent, tables, getTableData }}>
      {children}
    </SqlContext.Provider>
  );
};
