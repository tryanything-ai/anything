export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

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
            referencedColumns: ["flow_template_id"]
          },
          {
            foreignKeyName: "flow_template_tags_tag_id_fkey"
            columns: ["tag_id"]
            referencedRelation: "tags"
            referencedColumns: ["id"]
          }
        ]
      }
      flow_template_versions: {
        Row: {
          anything_flow_template_version: string
          commit_message: string | null
          created_at: string
          flow_template_id: string
          flow_template_json: Json
          flow_template_version: string
          flow_template_version_id: string
          flow_template_version_name: string
          published: boolean
          publisher_id: string
          recommended_version: boolean
          slug: string
        }
        Insert: {
          anything_flow_template_version: string
          commit_message?: string | null
          created_at?: string
          flow_template_id: string
          flow_template_json: Json
          flow_template_version?: string
          flow_template_version_id?: string
          flow_template_version_name: string
          published?: boolean
          publisher_id: string
          recommended_version?: boolean
          slug: string
        }
        Update: {
          anything_flow_template_version?: string
          commit_message?: string | null
          created_at?: string
          flow_template_id?: string
          flow_template_json?: Json
          flow_template_version?: string
          flow_template_version_id?: string
          flow_template_version_name?: string
          published?: boolean
          publisher_id?: string
          recommended_version?: boolean
          slug?: string
        }
        Relationships: [
          {
            foreignKeyName: "flow_template_versions_flow_template_id_fkey"
            columns: ["flow_template_id"]
            referencedRelation: "flow_templates"
            referencedColumns: ["flow_template_id"]
          },
          {
            foreignKeyName: "flow_template_versions_publisher_id_fkey"
            columns: ["publisher_id"]
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      flow_templates: {
        Row: {
          anonymous_publish: boolean
          created_at: string
          flow_template_description: string | null
          flow_template_id: string
          flow_template_name: string
          published: boolean
          publisher_id: string
          slug: string
        }
        Insert: {
          anonymous_publish: boolean
          created_at?: string
          flow_template_description?: string | null
          flow_template_id?: string
          flow_template_name: string
          published: boolean
          publisher_id: string
          slug: string
        }
        Update: {
          anonymous_publish?: boolean
          created_at?: string
          flow_template_description?: string | null
          flow_template_id?: string
          flow_template_name?: string
          published?: boolean
          publisher_id?: string
          slug?: string
        }
        Relationships: [
          {
            foreignKeyName: "flow_templates_publisher_id_fkey"
            columns: ["publisher_id"]
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          }
        ]
      }
      profiles: {
        Row: {
          avatar_url: string | null
          bio: string | null
          full_name: string | null
          github: string | null
          id: string
          instagram: string | null
          linkedin: string | null
          public: boolean
          tiktok: string | null
          twitter: string | null
          updated_at: string | null
          username: string | null
          website: string | null
          youtube: string | null
        }
        Insert: {
          avatar_url?: string | null
          bio?: string | null
          full_name?: string | null
          github?: string | null
          id: string
          instagram?: string | null
          linkedin?: string | null
          public?: boolean
          tiktok?: string | null
          twitter?: string | null
          updated_at?: string | null
          username?: string | null
          website?: string | null
          youtube?: string | null
        }
        Update: {
          avatar_url?: string | null
          bio?: string | null
          full_name?: string | null
          github?: string | null
          id?: string
          instagram?: string | null
          linkedin?: string | null
          public?: boolean
          tiktok?: string | null
          twitter?: string | null
          updated_at?: string | null
          username?: string | null
          website?: string | null
          youtube?: string | null
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
          tag_Icon: string | null
          tag_label: string
          tag_slug: string
          tag_uuid: string
        }
        Insert: {
          created_at?: string
          id: string
          tag_Icon?: string | null
          tag_label: string
          tag_slug: string
          tag_uuid?: string
        }
        Update: {
          created_at?: string
          id?: string
          tag_Icon?: string | null
          tag_label?: string
          tag_slug?: string
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
