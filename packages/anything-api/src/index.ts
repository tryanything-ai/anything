import * as flows from './flows';
import * as action_templates from './action-templates';
import * as secrets from './secrets';
import * as testing from './testing';
import * as tasks from './tasks'; 
import * as charts from './charts';
import * as auth from './auth'; 
import * as billing from './billing';

import dotenv from 'dotenv';
import path from 'path';

// Load the .env file
dotenv.config({ path: path.resolve(__dirname, '../.env') });

const api = {
  flows,
  testing,
  tasks,
  action_templates,
  secrets, 
  charts,
  auth,
  billing
};

export default api;

// Re-export types
export * from './testing';
export * from './tasks';
export * from './charts';