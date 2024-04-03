import * as listen from "./listen";
import * as os from "./os";
import * as path from "./path";
import * as watch from "./watch";
import * as flows from './flows';
import db from "./db";

const api = {
  ...listen,      
  flows,
  // ...watch, //stub for listening for file changes vs using fs-watch api          
  db,
  os, 
  path, 
  watch, 
};

export default api;
