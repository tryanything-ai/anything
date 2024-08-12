-- Create Schema for Anything Workflow Engine
CREATE SCHEMA IF NOT EXISTS anything;
-- Permissions 
GRANT USAGE ON SCHEMA anything TO anon, authenticated, service_role;
GRANT ALL ON ALL TABLES IN SCHEMA anything TO anon, authenticated, service_role;
GRANT ALL ON ALL ROUTINES IN SCHEMA anything TO anon, authenticated, service_role;
GRANT ALL ON ALL SEQUENCES IN SCHEMA anything TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA anything GRANT ALL ON TABLES TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA anything GRANT ALL ON ROUTINES TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA anything GRANT ALL ON SEQUENCES TO anon, authenticated, service_role;

--- Create Schema For Anything Marketplace
CREATE SCHEMA IF NOT EXISTS marketplace;

-- GRANT USAGE ON SCHEMA marketplace to authenticated;
-- GRANT USAGE ON SCHEMA marketplace to service_role;
--grant priveledges as needed for supabase

GRANT USAGE ON SCHEMA marketplace TO anon, authenticated, service_role;
GRANT ALL ON ALL TABLES IN SCHEMA marketplace TO anon, authenticated, service_role;
GRANT ALL ON ALL ROUTINES IN SCHEMA marketplace TO anon, authenticated, service_role;
GRANT ALL ON ALL SEQUENCES IN SCHEMA marketplace TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA marketplace GRANT ALL ON TABLES TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA marketplace GRANT ALL ON ROUTINES TO anon, authenticated, service_role;
ALTER DEFAULT PRIVILEGES FOR ROLE postgres IN SCHEMA marketplace GRANT ALL ON SEQUENCES TO anon, authenticated, service_role;
