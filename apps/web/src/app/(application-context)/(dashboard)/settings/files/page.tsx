"use client";

import { useState, useEffect, useCallback } from "react";
import { Trash2, Upload, Download, FileIcon } from "lucide-react";
import api from "@repo/anything-api";
import { Button } from "@repo/ui/components/ui/button";
import { createClient } from "@/lib/supabase/client";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@repo/ui/components/ui/alert-dialog";

import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import {
  Table,
  TableHeader,
  TableRow,
  TableHead,
  TableBody,
  TableCell,
} from "@repo/ui/components/ui/table";
import { useAnything } from "@/context/AnythingContext";
import { Progress } from "@repo/ui/components/ui/progress";

interface FileItem {
  file_id: string;
  file_name: string;
  file_size: number;
  created_at: string;
  content_type: string;
}

export default function FilesPage(): JSX.Element {
  const [files, setFiles] = useState<FileItem[]>([]);
  const [showDeleteDialog, setShowDeleteDialog] = useState(false);
  const [fileToDelete, setFileToDelete] = useState<string>("");
  const [uploadProgress, setUploadProgress] = useState<number>(0);
  const [isUploading, setIsUploading] = useState(false);

  const {
    accounts: { selectedAccount },
  } = useAnything();

  const fetchFiles = async () => {
    try {
      if (!selectedAccount) return;
      const response = await api.files.getFiles(
        await createClient(),
        selectedAccount.account_id,
      );
      console.log("Files response:", response);
      setFiles(response || []);
    } catch (error) {
      console.error("Error fetching files:", error);
    }
  };

  const handleFileUpload = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0];
    if (!file || !selectedAccount) return;

    setIsUploading(true);
    setUploadProgress(0);

    try {
      await api.files.uploadFile(
        await createClient(),
        selectedAccount.account_id,
        file,
        "public",
        (progress: number) => setUploadProgress(progress),
      );
      await fetchFiles();
    } catch (error) {
      console.error("Error uploading file:", error);
    } finally {
      setIsUploading(false);
      setUploadProgress(0);
    }
  };

  const handleDownload = async (fileId: string, fileName: string) => {
    try {
      if (!selectedAccount) return;
      const url = await api.files.getFileDownloadUrl(
        await createClient(),
        selectedAccount.account_id,
        fileId,
      );

      // Create a temporary anchor to trigger download
      const a = document.createElement("a");
      a.href = url;
      a.download = fileName;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
    } catch (error) {
      console.error("Error downloading file:", error);
    }
  };

  const deleteFile = async () => {
    try {
      if (!selectedAccount || !fileToDelete) return;

      await api.files.deleteFile(
        await createClient(),
        selectedAccount.account_id,
        fileToDelete,
      );
      await fetchFiles();
    } catch (error) {
      console.error("Error deleting file:", error);
    } finally {
      setFileToDelete("");
      setShowDeleteDialog(false);
    }
  };

  const formatFileSize = (bytes: number): string => {
    const sizes = ["Bytes", "KB", "MB", "GB"];
    if (bytes === 0) return "0 Bytes";
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(2)} ${sizes[i]}`;
  };

  useEffect(() => {
    fetchFiles();
  }, [selectedAccount]);

  return (
    <>
      <Card>
        <CardHeader className="flex flex-row">
          <div className="flex flex-col space-y-1.5 p-6">
            <CardTitle>Files</CardTitle>
            <CardDescription>Upload and manage your files</CardDescription>
          </div>
          <div className="ml-auto py-6">
            <Button asChild>
              <label className="cursor-pointer">
                <Upload className="mr-2 h-4 w-4" />
                Upload File
                <input
                  type="file"
                  className="hidden"
                  onChange={handleFileUpload}
                />
              </label>
            </Button>
          </div>
        </CardHeader>
        <CardContent>
          {isUploading && (
            <div className="mb-4">
              <Progress value={uploadProgress} className="w-full" />
              <p className="text-sm text-gray-500 mt-2">
                Uploading... {uploadProgress}%
              </p>
            </div>
          )}

          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Name</TableHead>
                <TableHead>Size</TableHead>
                <TableHead>Type</TableHead>
                <TableHead>Uploaded</TableHead>
                <TableHead className="text-right">Actions</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {files.map((file) => (
                <TableRow key={file.file_id}>
                  <TableCell className="font-medium">
                    <div className="flex items-center">
                      <FileIcon className="mr-2 h-4 w-4" />
                      {file.file_name}
                    </div>
                  </TableCell>
                  <TableCell>{formatFileSize(file.file_size)}</TableCell>
                  <TableCell>{file.content_type}</TableCell>
                  <TableCell>
                    {new Date(file.created_at).toLocaleDateString()}
                  </TableCell>
                  <TableCell className="text-right">
                    <Button
                      variant="outline"
                      size="sm"
                      className="ml-2"
                      onClick={() =>
                        handleDownload(file.file_id, file.file_name)
                      }
                    >
                      <Download className="h-4 w-4" />
                    </Button>
                    <Button
                      variant="outline"
                      size="sm"
                      className="ml-2"
                      onClick={() => {
                        setFileToDelete(file.file_id);
                        setShowDeleteDialog(true);
                      }}
                    >
                      <Trash2 className="h-4 w-4" />
                    </Button>
                  </TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>

      <AlertDialog open={showDeleteDialog} onOpenChange={setShowDeleteDialog}>
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete this
              file.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction className="bg-red-500" onClick={deleteFile}>
              Delete File
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </>
  );
}
