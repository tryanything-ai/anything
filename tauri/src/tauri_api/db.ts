import api from "./api";
const DB_STRING = "sqlite:test.db";

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
  

export default db;