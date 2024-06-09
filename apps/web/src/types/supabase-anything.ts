import { MergeDeep } from 'type-fest'
import { QueryResult, QueryData, QueryError } from '@supabase/supabase-js'
import { Database as DatabaseGenerated, Tables } from './generated/supabase-anything-generated'
export type { Json } from './generated/supabase-anything-generated'
import { Workflow } from './flows'
import { createClient } from "@/lib/supabase/server";

// Create Helper Types for nested queries from Anything-Server 
// that essentially proxies SUPABASE Postgrest queries

const supabase = createClient<Database>();

const db_workflows_query = supabase.from('flows')
    .select(`
  *,
  flow_versions(
    *
  )
`)

export type DB_WORKFLOWS_QUERY = QueryData<typeof db_workflows_query>
export type DB_FLOW_VERSION = Tables<'flow_versions'>
// export type DB_WORKFLOW = Tables<"flows">;

// Map JSON columns to TypeScript types:
export type Database = MergeDeep<
    DatabaseGenerated,
    {
        public: {
            Tables: {
                flow_versions: {
                    Row: {
                        flow_definition: Workflow;
                    };
                    Insert: {
                        flow_definition: Workflow;
                    };
                    Update: {
                        flow_definition?: Workflow;
                    };
                }
            }
        }
    }
>