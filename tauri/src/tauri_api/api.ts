import * as listen from "./listen";
import * as invoke from "./invoke";
import db from "./db";
import * as fs from "./fs";

const api = {
  ...invoke,
  ...listen,      
  // ...watch, //stub for listening for file changes vs using fs-watch api          
  db,
  fs, 
};

export default api;
