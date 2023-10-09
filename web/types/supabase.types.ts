export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]x

export interface Database {
  public: {
    Tables: {
      flow_template_tags: {
        Row: {
          created_at: string
          flow_template_id: string
          tag_id: string
        }
        Insert: {
          created_at?: string
          flow_template_id: string
          tag_id: string
        }
        Update: {
          created_at?: string
          flow_template_id?: string
          tag_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "flow_template_tags_flow_template_id_fkey"
            columns: ["flow_template_id"]
            referencedRelation: "flow_templates"
            referencedColumns: ["template_id"]
          },
          {
            foreignKeyName: "flow_template_tags_tag_id_fkey"
            columns: ["tag_id"]
            referencedRelation: "tags"
            referencedColumns: ["id"]
          }
        ]
      }
      flow_templates: {
        Row: {
          anonymous: boolean | null
          created_at: string
          flow_json: Json | null
          flow_name: string | null
          flow_templates_version: string
          published: boolean
          publisher_id: string
          template_id: string
        }
        Insert: {
          anonymous?: boolean | null
          created_at?: string
          flow_json?: Json | null
          flow_name?: string | null
          flow_templates_version?: string
          published?: boolean
          publisher_id: string
          template_id?: string
        }
        Update: {
          anonymous?: boolean | null
          created_at?: string
          flow_json?: Json | null
          flow_name?: string | null
          flow_templates_version?: string
          published?: boolean
          publisher_id?: string
          template_id?: string
        }
        Relationships: [
          {
            foreignKeyName: "flow_templates_publisher_id_fkey"
            columns: ["publisher_id"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
      profiles: {
        Row: {
          avatar_url: string | null
          full_name: string | null
          id: string
          updated_at: string | null
          username: string | null
          website: string | null
        }
        Insert: {
          avatar_url?: string | null
          full_name?: string | null
          id: string
          updated_at?: string | null
          username?: string | null
          website?: string | null
        }
        Update: {
          avatar_url?: string | null
          full_name?: string | null
          id?: string
          updated_at?: string | null
          username?: string | null
          website?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "profiles_id_fkey"
            columns: ["id"]
            referencedRelation: "users"
            referencedColumns: ["id"]
          }
        ]
      }
      tags: {
        Row: {
          created_at: string
          id: string
          tag_label: string
          tag_uuid: string
        }
        Insert: {
          created_at?: string
          id: string
          tag_label: string
          tag_uuid?: string
        }
        Update: {
          created_at?: string
          id?: string
          tag_label?: string
          tag_uuid?: string
        }
        Relationships: []
      }
    }
    Views: {
      [_ in never]: never
    }
    Functions: {
      [_ in never]: never
    }
    Enums: {
      [_ in never]: never
    }
    CompositeTypes: {
      [_ in never]: never
    }
  }
}
