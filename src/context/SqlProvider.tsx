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
  addEvent: (name: string) => void;
}

export const SqlContext = createContext<SqlContextInterface>({
  addEvent: () => {},
});

export const useSqlContext = () => useContext(SqlContext);

export const SqlProvider = ({ children }: { children: ReactNode }) => {
  const addEvent = async (name: string) => {
    await db.execute("INSERT INTO events (name) VALUES (?)", [name]);
  };

  const initDb = async () => {
    await db.execute(
      "CREATE TABLE IF NOT EXISTS events (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"
    );
  };

  useEffect(() => {
    initDb();
  }, []);

  return (
    <SqlContext.Provider value={{ addEvent }}>{children}</SqlContext.Provider>
  );
};
