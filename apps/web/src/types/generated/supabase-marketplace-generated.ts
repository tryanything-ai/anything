export type Json =
  | string
  | number
  | boolean
  | null
  | { [key: string]: Json | undefined }
  | Json[]

export type Database = {
  marketplace: {
    Tables: {
      flow_template_tags: {
        Row: {
          account_id: string
          created_at: string | null
          created_by: string | null
          flow_template_id: string
          tag_id: string
          updated_at: string | null
          updated_by: string | null
        }
        Insert: {
          account_id: string
          created_at?: string | null
          created_by?: string | null
          flow_template_id: string
          tag_id: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Update: {
          account_id?: string
          created_at?: string | null
          created_by?: string | null
          flow_template_id?: string
          tag_id?: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "flow_template_tags_account_id_fkey"
            columns: ["account_id"]
            isOneToOne: false
            referencedRelation: "accounts"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_tags_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_tags_flow_template_id_fkey"
            columns: ["flow_template_id"]
            isOneToOne: false
            referencedRelation: "flow_templates"
            referencedColumns: ["flow_template_id"]
          },
          {
            foreignKeyName: "flow_template_tags_tag_id_fkey"
            columns: ["tag_id"]
            isOneToOne: false
            referencedRelation: "tags"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_tags_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
      }
      flow_template_versions: {
        Row: {
          account_id: string
          anything_flow_version: string
          commit_message: string | null
          created_at: string | null
          created_by: string | null
          flow_template_id: string
          flow_template_json: Json
          flow_template_version: string
          flow_template_version_id: string
          flow_template_version_name: string
          public: boolean
          publisher_id: string
          recommended_version: boolean
          updated_at: string | null
          updated_by: string | null
        }
        Insert: {
          account_id: string
          anything_flow_version: string
          commit_message?: string | null
          created_at?: string | null
          created_by?: string | null
          flow_template_id: string
          flow_template_json: Json
          flow_template_version?: string
          flow_template_version_id?: string
          flow_template_version_name: string
          public?: boolean
          publisher_id: string
          recommended_version?: boolean
          updated_at?: string | null
          updated_by?: string | null
        }
        Update: {
          account_id?: string
          anything_flow_version?: string
          commit_message?: string | null
          created_at?: string | null
          created_by?: string | null
          flow_template_id?: string
          flow_template_json?: Json
          flow_template_version?: string
          flow_template_version_id?: string
          flow_template_version_name?: string
          public?: boolean
          publisher_id?: string
          recommended_version?: boolean
          updated_at?: string | null
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "flow_template_versions_account_id_fkey"
            columns: ["account_id"]
            isOneToOne: false
            referencedRelation: "accounts"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_versions_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_versions_flow_template_id_fkey"
            columns: ["flow_template_id"]
            isOneToOne: false
            referencedRelation: "flow_templates"
            referencedColumns: ["flow_template_id"]
          },
          {
            foreignKeyName: "flow_template_versions_publisher_id_fkey"
            columns: ["publisher_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_template_versions_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
      }
      flow_templates: {
        Row: {
          account_id: string
          anonymous_publish: boolean
          created_at: string | null
          created_by: string | null
          flow_template_description: string | null
          flow_template_id: string
          flow_template_name: string
          public: boolean
          publisher_id: string
          slug: string
          updated_at: string | null
          updated_by: string | null
        }
        Insert: {
          account_id: string
          anonymous_publish: boolean
          created_at?: string | null
          created_by?: string | null
          flow_template_description?: string | null
          flow_template_id?: string
          flow_template_name: string
          public: boolean
          publisher_id: string
          slug: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Update: {
          account_id?: string
          anonymous_publish?: boolean
          created_at?: string | null
          created_by?: string | null
          flow_template_description?: string | null
          flow_template_id?: string
          flow_template_name?: string
          public?: boolean
          publisher_id?: string
          slug?: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "flow_templates_account_id_fkey"
            columns: ["account_id"]
            isOneToOne: false
            referencedRelation: "accounts"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_templates_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_templates_publisher_id_fkey"
            columns: ["publisher_id"]
            isOneToOne: false
            referencedRelation: "profiles"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "flow_templates_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
      }
      profiles: {
        Row: {
          account_id: string
          avatar_url: string | null
          bio: string | null
          created_at: string | null
          created_by: string | null
          full_name: string | null
          github: string | null
          id: string
          instagram: string | null
          linkedin: string | null
          public: boolean
          tiktok: string | null
          twitter: string | null
          updated_at: string | null
          updated_by: string | null
          username: string | null
          website: string | null
          youtube: string | null
        }
        Insert: {
          account_id: string
          avatar_url?: string | null
          bio?: string | null
          created_at?: string | null
          created_by?: string | null
          full_name?: string | null
          github?: string | null
          id?: string
          instagram?: string | null
          linkedin?: string | null
          public?: boolean
          tiktok?: string | null
          twitter?: string | null
          updated_at?: string | null
          updated_by?: string | null
          username?: string | null
          website?: string | null
          youtube?: string | null
        }
        Update: {
          account_id?: string
          avatar_url?: string | null
          bio?: string | null
          created_at?: string | null
          created_by?: string | null
          full_name?: string | null
          github?: string | null
          id?: string
          instagram?: string | null
          linkedin?: string | null
          public?: boolean
          tiktok?: string | null
          twitter?: string | null
          updated_at?: string | null
          updated_by?: string | null
          username?: string | null
          website?: string | null
          youtube?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "profiles_account_id_fkey"
            columns: ["account_id"]
            isOneToOne: false
            referencedRelation: "accounts"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "profiles_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "profiles_id_fkey"
            columns: ["id"]
            isOneToOne: true
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "profiles_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
      }
      tags: {
        Row: {
          created_at: string | null
          created_by: string | null
          id: string
          tag_icon: string | null
          tag_label: string
          tag_slug: string
          tag_uuid: string
          updated_at: string | null
          updated_by: string | null
        }
        Insert: {
          created_at?: string | null
          created_by?: string | null
          id: string
          tag_icon?: string | null
          tag_label: string
          tag_slug: string
          tag_uuid?: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Update: {
          created_at?: string | null
          created_by?: string | null
          id?: string
          tag_icon?: string | null
          tag_label?: string
          tag_slug?: string
          tag_uuid?: string
          updated_at?: string | null
          updated_by?: string | null
        }
        Relationships: [
          {
            foreignKeyName: "tags_created_by_fkey"
            columns: ["created_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
          {
            foreignKeyName: "tags_updated_by_fkey"
            columns: ["updated_by"]
            isOneToOne: false
            referencedRelation: "users"
            referencedColumns: ["id"]
          },
        ]
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

type PublicSchema = Database[Extract<keyof Database, "public">]

export type Tables<
  PublicTableNameOrOptions extends
    | keyof (PublicSchema["Tables"] & PublicSchema["Views"])
    | { schema: keyof Database },
  TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
    ? keyof (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
        Database[PublicTableNameOrOptions["schema"]]["Views"])
    : never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
  ? (Database[PublicTableNameOrOptions["schema"]]["Tables"] &
      Database[PublicTableNameOrOptions["schema"]]["Views"])[TableName] extends {
      Row: infer R
    }
    ? R
    : never
  : PublicTableNameOrOptions extends keyof (PublicSchema["Tables"] &
        PublicSchema["Views"])
    ? (PublicSchema["Tables"] &
        PublicSchema["Views"])[PublicTableNameOrOptions] extends {
        Row: infer R
      }
      ? R
      : never
    : never

export type TablesInsert<
  PublicTableNameOrOptions extends
    | keyof PublicSchema["Tables"]
    | { schema: keyof Database },
  TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
    ? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
  ? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Insert: infer I
    }
    ? I
    : never
  : PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
    ? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
        Insert: infer I
      }
      ? I
      : never
    : never

export type TablesUpdate<
  PublicTableNameOrOptions extends
    | keyof PublicSchema["Tables"]
    | { schema: keyof Database },
  TableName extends PublicTableNameOrOptions extends { schema: keyof Database }
    ? keyof Database[PublicTableNameOrOptions["schema"]]["Tables"]
    : never = never,
> = PublicTableNameOrOptions extends { schema: keyof Database }
  ? Database[PublicTableNameOrOptions["schema"]]["Tables"][TableName] extends {
      Update: infer U
    }
    ? U
    : never
  : PublicTableNameOrOptions extends keyof PublicSchema["Tables"]
    ? PublicSchema["Tables"][PublicTableNameOrOptions] extends {
        Update: infer U
      }
      ? U
      : never
    : never

export type Enums<
  PublicEnumNameOrOptions extends
    | keyof PublicSchema["Enums"]
    | { schema: keyof Database },
  EnumName extends PublicEnumNameOrOptions extends { schema: keyof Database }
    ? keyof Database[PublicEnumNameOrOptions["schema"]]["Enums"]
    : never = never,
> = PublicEnumNameOrOptions extends { schema: keyof Database }
  ? Database[PublicEnumNameOrOptions["schema"]]["Enums"][EnumName]
  : PublicEnumNameOrOptions extends keyof PublicSchema["Enums"]
    ? PublicSchema["Enums"][PublicEnumNameOrOptions]
    : never

