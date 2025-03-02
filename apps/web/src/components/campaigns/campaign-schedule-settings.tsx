import { useState } from "react";
import { useAnything } from "@/context/AnythingContext";
import { createClient } from "@/lib/supabase/client";
import api from "@repo/anything-api";
import { Button } from "@repo/ui/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { Checkbox } from "@repo/ui/components/ui/checkbox";
import { Label } from "@repo/ui/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@repo/ui/components/ui/select";
import { TimeInput } from "@/components/time-input";

// Include all days of the week
const DAYS_OF_WEEK = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday", // Include weekend
  "Sunday", // Include weekend
];

const TIMEZONES = [
  "America/New_York",
  "America/Chicago",
  "America/Denver",
  "America/Los_Angeles",
  "America/Anchorage",
  "America/Honolulu",
  "America/Phoenix",
  "Europe/London",
  "Europe/Paris",
  "Asia/Tokyo",
  "Australia/Sydney",
];

interface CampaignScheduleSettingsProps {
  campaignId: string;
  initialSettings?: {
    schedule_days_of_week?: string[];
    schedule_start_time?: string;
    schedule_end_time?: string;
    timezone?: string;
  };
  onSaved?: () => void;
}

export function CampaignScheduleSettings({
  campaignId,
  initialSettings,
  onSaved,
}: CampaignScheduleSettingsProps) {
  const {
    accounts: { selectedAccount },
  } = useAnything();

  // Default to all days if no initial settings are provided
  const [selectedDays, setSelectedDays] = useState<string[]>(
    initialSettings?.schedule_days_of_week || DAYS_OF_WEEK,
  );
  const [startTime, setStartTime] = useState(
    initialSettings?.schedule_start_time || "09:00:00",
  );
  const [endTime, setEndTime] = useState(
    initialSettings?.schedule_end_time || "17:00:00",
  );
  const [timezone, setTimezone] = useState(
    initialSettings?.timezone || "America/New_York",
  );
  const [isSaving, setIsSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);

  const handleDayToggle = (day: string) => {
    if (selectedDays.includes(day)) {
      setSelectedDays(selectedDays.filter((d) => d !== day));
    } else {
      setSelectedDays([...selectedDays, day]);
    }
  };

  // Add convenience functions for selecting weekdays/weekends
  const selectWeekdaysOnly = () => {
    setSelectedDays(["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"]);
  };

  const selectWeekendsOnly = () => {
    setSelectedDays(["Saturday", "Sunday"]);
  };

  const selectAllDays = () => {
    setSelectedDays([...DAYS_OF_WEEK]);
  };

  const handleSaveSettings = async () => {
    if (!selectedAccount || !campaignId) return;

    try {
      setIsSaving(true);
      setSaveError(null);

      await api.campaigns.updateCampaign(
        await createClient(),
        selectedAccount.account_id,
        campaignId,
        {
          schedule_days_of_week: selectedDays,
          schedule_start_time: startTime,
          schedule_end_time: endTime,
          timezone: timezone,
        },
      );

      // Call the onSaved callback if provided
      if (onSaved) {
        onSaved();
      }

      // Show success message
      alert("Campaign schedule settings have been saved.");
    } catch (error) {
      console.error("Error saving schedule settings:", error);
      setSaveError("Failed to save schedule settings. Please try again.");
    } finally {
      setIsSaving(false);
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Campaign Schedule</CardTitle>
        <CardDescription>
          Set when this campaign is allowed to make calls
        </CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <div className="flex justify-between items-center">
            <Label>Days of Week</Label>
            <div className="flex space-x-2">
              <Button
                variant="outline"
                size="sm"
                onClick={selectWeekdaysOnly}
                type="button"
              >
                Weekdays
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={selectWeekendsOnly}
                type="button"
              >
                Weekends
              </Button>
              <Button
                variant="outline"
                size="sm"
                onClick={selectAllDays}
                type="button"
              >
                All Days
              </Button>
            </div>
          </div>
          <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
            {DAYS_OF_WEEK.map((day) => (
              <div key={day} className="flex items-center space-x-2">
                <Checkbox
                  id={`day-${day}`}
                  checked={selectedDays.includes(day)}
                  onCheckedChange={() => handleDayToggle(day)}
                />
                <Label htmlFor={`day-${day}`}>{day}</Label>
              </div>
            ))}
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div className="space-y-2">
            <Label htmlFor="start-time">Start Time</Label>
            <TimeInput
              id="start-time"
              value={startTime}
              onChange={(value) => setStartTime(value)}
            />
          </div>
          <div className="space-y-2">
            <Label htmlFor="end-time">End Time</Label>
            <TimeInput
              id="end-time"
              value={endTime}
              onChange={(value) => setEndTime(value)}
            />
          </div>
        </div>

        <div className="space-y-2">
          <Label htmlFor="timezone">Timezone</Label>
          <Select value={timezone} onValueChange={setTimezone}>
            <SelectTrigger id="timezone">
              <SelectValue placeholder="Select timezone" />
            </SelectTrigger>
            <SelectContent>
              {TIMEZONES.map((tz) => (
                <SelectItem key={tz} value={tz}>
                  {tz}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>

        {saveError && <div className="text-red-500 text-sm">{saveError}</div>}
      </CardContent>
      <CardFooter>
        <Button
          onClick={handleSaveSettings}
          disabled={isSaving || selectedDays.length === 0}
        >
          {isSaving ? "Saving..." : "Save Schedule"}
        </Button>
      </CardFooter>
    </Card>
  );
}
