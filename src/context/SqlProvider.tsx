import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";
import Database from "tauri-plugin-sql-api";

const db = await Database.load("sqlite:test.db");

interface SqlContextInterface {
  tables: any[];
  addEvent: (name: string) => void;
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

  const addEvent = async (name: string) => {
    await db.execute("INSERT INTO events (name) VALUES (?)", [name]);
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
    // setTables(tableData as any);
  };

  const initDb = async () => {
    await db.execute(
      "CREATE TABLE IF NOT EXISTS events (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"
    );
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
