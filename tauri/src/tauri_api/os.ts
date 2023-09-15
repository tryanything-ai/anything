import * as os from "@tauri-apps/api/os";

export const type = async () => {
    return await os.type();
}
