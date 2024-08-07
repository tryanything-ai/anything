import * as flows from './flows';
import * as action_templates from './action-templates';
import * as secrets from './secrets';
import * as testing from './testing';
import * as tasks from './tasks'

const api = {
  flows,
  testing,
  tasks,
  action_templates,
  secrets
};

export default api;