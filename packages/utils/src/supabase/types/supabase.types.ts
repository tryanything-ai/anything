import { PostgrestError } from '@supabase/supabase-js'

import { Database as DatabaseGenerated, Json } from './supabase.generated-types'

export type Database = DatabaseGenerated; 
export type { Json }; 

// Helpres for Tables
export type Tables<T extends keyof Database['public']['Tables']> = Database['public']['Tables'][T]['Row']
export type Enums<T extends keyof Database['public']['Enums']> = Database['public']['Enums'][T]

//Tables
export type Profile = Tables<'profiles'>
// export type FlowTemplate = Tables<'flow_templates'>
export type FlowTemplateTag = Tables<'flow_template_tags'>
export type Tag = Tables<'tags'>
export type FlowTemplateVersion = Tables<'flow_template_versions'>

// Results
export type DbResult<T> = T extends PromiseLike<infer U> ? U : never
export type DbResultOk<T> = T extends PromiseLike<{ data: infer U }> ? Exclude<U, null> : never
export type DbResultErr = PostgrestError

// https://supabase.com/docs/reference/javascript/typescript-support#helper-types
// export type Database = MergeDeep<
//   DatabaseGenerated,
//   {
//     public: {
//       Tables: {
//         profiles: {
//           Row: {
//             // id is a primary key in public.movies, so it must be `not null`
//             id: number
//           }
//         }
//       }
//     }
//   }
// >
