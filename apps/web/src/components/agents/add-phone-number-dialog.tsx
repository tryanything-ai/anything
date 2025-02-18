"use client";

import { useEffect, useState } from "react";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
} from "@repo/ui/components/ui/sheet";
import { ScrollArea } from "@repo/ui/components/ui/scroll-area";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import { useParams } from "next/navigation";
import { Button } from "@repo/ui/components/ui/button";
import { Loader2, Phone } from "lucide-react";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@repo/ui/components/ui/select";
import { Input } from "@repo/ui/components/ui/input";
import { Label } from "@repo/ui/components/ui/label";

interface AddPhoneNumberDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onPhoneNumberAdd: (phoneNumber: string) => Promise<void>;
}

export function AddPhoneNumberDialog({
  open,
  onOpenChange,
  onPhoneNumberAdd,
}: AddPhoneNumberDialogProps): JSX.Element {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [availableNumbers, setAvailableNumbers] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isSearching, setIsSearching] = useState(false);
  const [isPurchasing, setIsPurchasing] = useState(false);
  const [selectedCountry, setSelectedCountry] = useState("US");
  const [areaCode, setAreaCode] = useState("");
  const [selectedNumber, setSelectedNumber] = useState<string | null>(null);
  const params = useParams<{ agentId: string }>();

  const searchNumbers = async () => {
    if (!selectedAccount) return;
    
    setIsSearching(true);
    try {
      // Stub API call - replace with actual implementation
      // const numbers = await api.twilio.searchNumbers(selectedCountry, areaCode);
      // Mock response
      const mockNumbers = [
        { phoneNumber: "+14155550123", formattedNumber: "(415) 555-0123" },
        { phoneNumber: "+14155550124", formattedNumber: "(415) 555-0124" },
        { phoneNumber: "+14155550125", formattedNumber: "(415) 555-0125" },
      ];
      setAvailableNumbers(mockNumbers);
    } catch (error) {
      console.error("Error searching phone numbers:", error);
    } finally {
      setIsSearching(false);
    }
  };

  const purchaseNumber = async (phoneNumber: string) => {
    if (!selectedAccount || !phoneNumber) return;

    setIsPurchasing(true);
    try {
      // Stub API call - replace with actual implementation
      // await api.twilio.purchaseNumber(selectedAccount.account_id, params.agentId, phoneNumber);
      await onPhoneNumberAdd(phoneNumber);
      onOpenChange(false);
    } catch (error) {
      console.error("Error purchasing number:", error);
    } finally {
      setIsPurchasing(false);
    }
  };

  return (
    <Sheet open={open} onOpenChange={onOpenChange}>
      <SheetContent side={"right"} className="w-[400px]">
        <SheetHeader>
          <SheetTitle>Add Phone Number</SheetTitle>
          <SheetDescription>
            Search and purchase a phone number for your agent
          </SheetDescription>
        </SheetHeader>

        <div className="py-6 space-y-6">
          <div className="space-y-4">
            <div className="space-y-2">
              <Label>Country</Label>
              <Select
                value={selectedCountry}
                onValueChange={setSelectedCountry}
                disabled={isSearching}
              >
                <SelectTrigger>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="US">United States</SelectItem>
                  <SelectItem value="CA">Canada</SelectItem>
                  <SelectItem value="GB">United Kingdom</SelectItem>
                </SelectContent>
              </Select>
            </div>

            <div className="space-y-2">
              <Label>Area Code</Label>
              <Input
                placeholder="e.g. 415"
                value={areaCode}
                onChange={(e) => setAreaCode(e.target.value)}
                disabled={isSearching}
              />
            </div>

            <Button 
              onClick={searchNumbers}
              disabled={isSearching || !areaCode}
              className="w-full"
            >
              {isSearching ? (
                <>
                  <Loader2 className="w-4 h-4 mr-2 animate-spin" />
                  Searching...
                </>
              ) : (
                <>
                  <Phone className="w-4 h-4 mr-2" />
                  Search Numbers
                </>
              )}
            </Button>
          </div>

          <ScrollArea className="h-[300px] pr-4">
            {availableNumbers.map((number) => (
              <div
                key={number.phoneNumber}
                className="flex items-center justify-between p-4 border rounded-md mb-2 hover:border-primary cursor-pointer"
                onClick={() => setSelectedNumber(number.phoneNumber)}
              >
                <div>
                  <div className="font-medium">{number.formattedNumber}</div>
                  <div className="text-sm text-muted-foreground">
                    {number.phoneNumber}
                  </div>
                </div>
                <Button
                  size="sm"
                  disabled={isPurchasing}
                  onClick={() => purchaseNumber(number.phoneNumber)}
                >
                  {isPurchasing && selectedNumber === number.phoneNumber ? (
                    <Loader2 className="w-4 h-4 animate-spin" />
                  ) : (
                    "Select"
                  )}
                </Button>
              </div>
            ))}
          </ScrollArea>
        </div>
      </SheetContent>
    </Sheet>
  );
}
