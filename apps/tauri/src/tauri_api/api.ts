import * as listen from "./listen";
import * as invoke from "./invoke";
import * as fs from "./fs";
import * as os from "./os";
import * as path from "./path";
import * as watch from "./watch";
import * as flows from './flows';
import db from "./db";

const api = {
  ...invoke,
  ...listen,      
  flows,
  // ...watch, //stub for listening for file changes vs using fs-watch api          
  db,
  fs, 
  os, 
  path, 
  watch, 
};

export default api;
