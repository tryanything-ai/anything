import * as flows from './flows';
import * as action_templates from './action-templates';
import * as secrets from './secrets';
import * as testing from './testing';
import * as tasks from './tasks'; 
import * as charts from './charts';
import * as auth from './auth'; 
import * as billing from './billing';
import * as marketplace from './marketplace';
import * as profiles from './profiles';
import * as variables from './variable-explorer'; 
import * as agents from './agents';
import * as campaigns from './campaigns'; 
import * as contacts from './contacts';

import * as accounts from './accounts';

const api = {
  flows,
  testing,
  tasks,
  action_templates,
  secrets, 
  charts,
  auth,
  billing,
  marketplace, 
  profiles,
  variables,
  agents,
  campaigns,
  contacts,
  accounts,
};

export default api;

// Re-export types
export * from './testing';
export * from './tasks';
export * from './charts';
export * from "./types/workflows";