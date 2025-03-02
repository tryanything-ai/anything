import { useState, useRef } from "react";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@repo/ui/components/ui/dialog";
import { Button } from "@repo/ui/components/ui/button";
import { FileUp, X } from "lucide-react";
import { Alert, AlertDescription } from "@repo/ui/components/ui/alert";
import Papa from "papaparse";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";

interface UploadContactListDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  accountId: string;
  campaignId: string;
}

export function UploadContactsListDialog({
  open,
  onOpenChange,
  accountId,
  campaignId,
}: UploadContactListDialogProps) {
  const [file, setFile] = useState<File | null>(null);
  const [isUploading, setIsUploading] = useState(false);
  const [dragActive, setDragActive] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [parseError, setParseError] = useState<string | null>(null);

  const handleDrag = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === "dragenter" || e.type === "dragover") {
      setDragActive(true);
    } else if (e.type === "dragleave") {
      setDragActive(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);

    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      const droppedFile = e.dataTransfer.files[0];
      validateAndSetFile(droppedFile);
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files[0]) {
      validateAndSetFile(e.target.files[0]);
    }
  };

  const validateAndSetFile = (file: File) => {
    // Check if file is CSV
    if (file.type !== "text/csv" && !file.name.endsWith(".csv")) {
      alert("Please upload a CSV file");
      return;
    }

    // Check file size (max 5MB)
    if (file.size > 5 * 1024 * 1024) {
      alert("Please upload a file smaller than 5MB");
      return;
    }

    setFile(file);
  };

  const handleUpload = async () => {
    if (!file) return;

    try {
      setIsUploading(true);
      setParseError(null);

      // Parse the CSV file
      const parse_result = await new Promise<Papa.ParseResult<any>>(
        (resolve, reject) => {
          Papa.parse(file, {
            header: true,
            skipEmptyLines: true,
            complete: resolve,
            error: reject,
          });
        },
      );

      // Transform the parsed data into the format expected by our API
      const contacts = parse_result.data.map((row) => ({
        name: `${row.first_name || ""} ${row.last_name || ""}`.trim(),
        phone_number: row.phone_number,
        email: row.email || undefined,
        additional_data: Object.keys(row)
          .filter(
            (key) =>
              !["first_name", "last_name", "phone_number", "email"].includes(
                key,
              ),
          )
          .reduce(
            (obj, key) => {
              obj[key] = row[key];
              return obj;
            },
            {} as Record<string, any>,
          ),
      }));

      // Filter out entries without phone numbers
      const validContacts = contacts.filter((contact) => contact.phone_number);

      if (validContacts.length === 0) {
        setParseError(
          "No valid contacts found. Ensure your CSV has a phone_number column.",
        );
        setIsUploading(false);
        return;
      }

      // Use our new function to add contacts with deduplication
      const result = await api.campaigns.addContactsToCampaignWithDeduplication(
        await createClient(),
        accountId,
        campaignId,
        validContacts,
      );

      console.log(
        "[CAMPAIGN] Successfully added contacts to campaign:",
        result,
      );

      // Show success message with counts from the response
      alert(`Successfully processed ${validContacts.length} contacts:
      - ${result.created_contacts} new contacts created
      - ${result.existing_contacts} existing contacts found
      - ${result.added_to_campaign} contacts added to campaign`);

      onOpenChange(false);
    } catch (error) {
      console.error("Error processing file:", error);
      setParseError("There was an error processing your customer list");
    } finally {
      setIsUploading(false);
    }
  };

  const clearFile = () => {
    setFile(null);
    if (fileInputRef.current) {
      fileInputRef.current.value = "";
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[500px]">
        <DialogHeader>
          <DialogTitle>Upload Customer List</DialogTitle>
          <DialogDescription>
            Upload a CSV file with your customer contact information.
          </DialogDescription>
        </DialogHeader>

        <div className="py-4">
          <Alert>
            <AlertDescription>
              Your CSV file should include columns for: first_name, last_name,
              phone_number, and email (optional).
            </AlertDescription>
          </Alert>

          <div
            className={`mt-4 border-2 border-dashed rounded-lg p-6 text-center ${
              dragActive
                ? "border-primary bg-primary/5"
                : "border-muted-foreground/25"
            }`}
            onDragEnter={handleDrag}
            onDragLeave={handleDrag}
            onDragOver={handleDrag}
            onDrop={handleDrop}
          >
            {file ? (
              <div className="flex items-center justify-between p-2 border rounded">
                <div className="flex items-center">
                  <FileUp className="h-5 w-5 mr-2 text-muted-foreground" />
                  <span className="text-sm font-medium">{file.name}</span>
                  <span className="ml-2 text-xs text-muted-foreground">
                    ({(file.size / 1024).toFixed(1)} KB)
                  </span>
                </div>
                <Button
                  variant="ghost"
                  size="icon"
                  onClick={clearFile}
                  className="h-8 w-8"
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            ) : (
              <>
                <FileUp className="h-10 w-10 text-muted-foreground mx-auto mb-2" />
                <p className="text-sm text-muted-foreground mb-2">
                  Drag and drop your CSV file here, or click to browse
                </p>
                <Button
                  variant="secondary"
                  onClick={() => fileInputRef.current?.click()}
                >
                  Select File
                </Button>
                <input
                  type="file"
                  ref={fileInputRef}
                  onChange={handleFileChange}
                  accept=".csv"
                  className="hidden"
                />
              </>
            )}
          </div>

          {parseError && (
            <Alert variant="destructive" className="mt-4">
              <AlertDescription>{parseError}</AlertDescription>
            </Alert>
          )}
        </div>

        <DialogFooter>
          <Button onClick={handleUpload} disabled={!file || isUploading}>
            {isUploading ? "Uploading..." : "Upload Customer List"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
