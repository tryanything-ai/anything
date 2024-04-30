//Maximum extensibility requires carefull control upgrading interfaces
//These versions represent the changing state of Anything internally
//THese are not for say a user changing a flow to do something new
//Its about system compatability and how things work together over time

//TODO: some day some system to manage changelogs etc
//TODO: define automatable way to upgrade all interfaces deterministically if  possible

//Extension Interface
//Used to mark how extensions work and when it changes
export const ANYTHING_EXTENSION_VERSION = "0.0.0";

//Trigger Interface 
//Used to mark how triggers work and when it changes
export const ANYTHING_TRIGGER_VERSION = "0.0.0";

//Action Interface
//Used to mark how actions work and when it changes
export const ANYTHING_ACTION_VERSION = "0.0.0";

//Flow Interface
//Used to mark how triggers, actions, and extensions al work together and when it changes
export const ANYTHING_FLOW_VERSION = "0.0.0";


