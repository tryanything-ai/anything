import { execSync } from "child_process";
import dotenv from "dotenv";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

dotenv.config();

// Print out all environment variables
console.log(process.env);

const projectId = process.env.SUPABASE_PROJECT_ID;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

if (!projectId) {
  console.error("Error: PROJECT_REF is not set in .env file.");
  process.exit(1);
}

const outputPath = path.join(__dirname, "../types/supabase.generated-types.ts");

const command = `npx -y supabase gen types typescript --project-id "${projectId}" --schema public > ${outputPath}`;
execSync(command, { stdio: "inherit" });
