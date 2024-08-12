import * as flows from './flows';
import * as action_templates from './action-templates';
import * as secrets from './secrets';
import * as testing from './testing';
import * as tasks from './tasks'; 
import * as charts from './charts';
import * as auth from './auth'; 

const api = {
  flows,
  testing,
  tasks,
  action_templates,
  secrets, 
  charts,
  auth
};

export default api;