const execSync = require("child_process").execSync;
const dotenv = require("dotenv");
const path = require("path");

dotenv.config();

const projectId = process.env.SUPABASE_PROJECT_ID;

if (!projectId) {
  console.error("Error: PROJECT_REF is not set in .env file.");
  process.exit(1);
}

const outputPath = path.join(__dirname, "./types/supabase.generated-types.ts");

const command = `npx -y supabase gen types typescript --project-id "${projectId}" --schema public > ${outputPath}`;
execSync(command, { stdio: "inherit" });
