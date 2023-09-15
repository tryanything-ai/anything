import * as listen from "./listen";
import db from "./db";
import * as invoke from "./invoke";

const api = {
  ...invoke,
  ...listen,
  db,
};

export default api;
