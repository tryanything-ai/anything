import { useAnything } from "@/context/AnythingContext";
import { useEffect, useState } from "react";
import api from "@repo/anything-api";
import { useAccounts } from "@/context/AccountsContext";
import { Button } from "@repo/ui/components/ui/button";
import { createClient } from "@/lib/supabase/client";
import { RadioGroup, RadioGroupItem } from "@repo/ui/components/ui/radio-group";
import { Label } from "@repo/ui/components/ui/label";
import Image from "next/image";
import { FileIcon } from "lucide-react";
import Link from "next/link";

interface File {
  file_id: string;
  file_name: string;
  content_type: string;
  public_url: string;
}

function FilePreview({ file }: { file: File }) {
  if (file.content_type.startsWith("image/")) {
    return (
      <div className="relative w-20 h-20">
        <Image
          src={file.public_url}
          alt={file.file_name}
          fill
          className="object-cover rounded-md"
        />
      </div>
    );
  }

  return (
    <div className="w-20 h-20 flex items-center justify-center bg-gray-100 rounded-md">
      <span className="text-xs text-gray-500">{file.content_type}</span>
    </div>
  );
}

function FileRow({
  file,
  onInsert,
}: {
  file: File;
  onInsert: (value: string) => void;
}) {
  const [format, setFormat] = useState<"url" | "base64">("url");

  const handleInsert = () => {
    onInsert(`{{files.${file.file_name}.${format}}}`);
  };

  return (
    <div className="flex items-center gap-4 p-2 border-b last:border-b-0">
      <FilePreview file={file} />

      <div className="flex-1">
        <div className="font-medium">{file.file_name}</div>
        <div className="text-sm text-gray-500">{file.content_type}</div>
      </div>

      <RadioGroup
        defaultValue="url"
        onValueChange={(value) => setFormat(value as "url" | "base64")}
        className="flex gap-4"
      >
        <div className="flex items-center space-x-2">
          <RadioGroupItem value="url" id={`url-${file.file_id}`} />
          <Label htmlFor={`url-${file.file_id}`}>URL</Label>
        </div>
        <div className="flex items-center space-x-2">
          <RadioGroupItem value="base64" id={`base64-${file.file_id}`} />
          <Label htmlFor={`base64-${file.file_id}`}>Base64</Label>
        </div>
      </RadioGroup>

      <Button
        variant="ghost"
        className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
        onClick={handleInsert}
      >
        Insert
      </Button>
    </div>
  );
}

function EmptyState() {
  return (
    <div className="flex flex-col items-center justify-center p-8 text-center">
      <div className="rounded-full bg-gray-100 p-3 mb-4">
        <FileIcon className="h-6 w-6 text-gray-400" />
      </div>
      <h3 className="text-lg font-semibold mb-2">No files found</h3>
      <p className="text-sm text-gray-500 mb-4">
        Upload files to use them in your automations
      </p>
      <Link href="/settings/files">
        <Button
          variant="outline"
          className="gap-2 border-blue-500 text-blue-500 hover:bg-blue-50"
        >
          <FileIcon className="h-4 w-4" />
          Upload your first file
        </Button>
      </Link>
    </div>
  );
}

export function FilesExplorer(): JSX.Element {
  const {
    workflow: { selected_node_data },
    explorer: { insertVariable },
  } = useAnything();

  const [files, setFiles] = useState<File[]>([]);
  const [loading, setLoading] = useState(false);
  const { selectedAccount } = useAccounts();

  const fetchFiles = async () => {
    try {
      setLoading(true);
      if (!selectedAccount) {
        console.error("No account selected");
        return;
      }
      const response = await api.files.getFiles(
        await createClient(),
        selectedAccount.account_id,
      );
      setFiles(response || []);
    } catch (error) {
      console.error("Error fetching files:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    console.log("[FILES EXPLORER] Initial fetch triggered");
    fetchFiles();
  }, [selected_node_data?.action_id]);

  return (
    <div className="flex flex-col w-full">
      {selected_node_data && (
        <div className="w-full">
          {loading && (
            <div className="flex items-center justify-center p-8">
              <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-400" />
            </div>
          )}
          {files.length === 0 && !loading && <EmptyState />}
          {files.length > 0 && (
            <div className="h-auto w-full flex flex-col bg-white bg-opacity-5 overflow-hidden border rounded-md">
              <div className="p-3">
                <div className="flex-1 font-bold mb-2">Files</div>
                <div className="w-full rounded-lg p-2.5 bg-[whitesmoke]">
                  {files.map((file) => (
                    <FileRow
                      key={file.file_id}
                      file={file}
                      onInsert={insertVariable}
                    />
                  ))}
                </div>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
