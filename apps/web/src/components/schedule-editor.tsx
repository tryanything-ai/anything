import { useState, useEffect } from "react";
import { Clock, Calendar } from "lucide-react";
import { Label } from "@repo/ui/components/ui/label";
import { Button } from "@repo/ui/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@repo/ui/components/ui/select";

interface ScheduleEditorProps {
  days: string[];
  startTime: string;
  endTime: string;
  timezone: string;
  onDaysChange: (days: string[]) => void;
  onStartTimeChange: (time: string) => void;
  onEndTimeChange: (time: string) => void;
  onTimezoneChange: (timezone: string) => void;
  isEditing: boolean;
}

export function ScheduleEditor({
  days,
  startTime,
  endTime,
  timezone,
  onDaysChange,
  onStartTimeChange,
  onEndTimeChange,
  onTimezoneChange,
  isEditing,
}: ScheduleEditorProps) {
  // For time editing
  const [editingTime, setEditingTime] = useState<"start" | "end" | null>(null);

  // Format time for display
  const formatTimeDisplay = (time: string) => {
    return time.substring(0, 5);
  };

  // Toggle day selection
  const toggleDay = (day: string) => {
    if (days.includes(day)) {
      onDaysChange(days.filter((d) => d !== day));
    } else {
      onDaysChange([...days, day]);
    }
  };

  // All available days
  const allDays = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
  ];

  // Time options
  const hours = Array.from({ length: 24 }, (_, i) =>
    i.toString().padStart(2, "0"),
  );
  const minutes = ["00", "15", "30", "45"];

  // Handle time selection
  const handleTimeSelect = (
    type: "start" | "end",
    hour: string,
    minute: string,
  ) => {
    const newTime = `${hour}:${minute}:00`;
    if (type === "start") {
      onStartTimeChange(newTime);
    } else {
      onEndTimeChange(newTime);
    }
    setEditingTime(null);
  };

  // Parse current times
  const parseTime = (timeString: string) => {
    const [hour = "00", minute = "00"] = timeString
      .split(":")
      .map((part) => part.padStart(2, "0"));
    return { hour, minute };
  };

  const startTimeParts = parseTime(startTime);
  const endTimeParts = parseTime(endTime);

  return (
    <div className="px-3 py-2 rounded-md border">
      <div className="flex flex-col gap-2">
        {/* Time Display/Editor */}
        <div className="flex items-center gap-2 relative">
          <Clock className="w-4 h-4 text-orange-500" />

          {isEditing ? (
            <>
              {/* Start Time Button */}
              <button
                type="button"
                onClick={() =>
                  setEditingTime(editingTime === "start" ? null : "start")
                }
                className="text-sm font-medium hover:bg-gray-100 px-1 py-0.5 rounded"
              >
                {formatTimeDisplay(startTime)}
              </button>

              <span>-</span>

              {/* End Time Button */}
              <button
                type="button"
                onClick={() =>
                  setEditingTime(editingTime === "end" ? null : "end")
                }
                className="text-sm font-medium hover:bg-gray-100 px-1 py-0.5 rounded"
              >
                {formatTimeDisplay(endTime)}
              </button>

              {/* Timezone Selector */}
              <Select value={timezone} onValueChange={onTimezoneChange}>
                <SelectTrigger className="h-7 text-xs border-0 bg-transparent w-auto">
                  <SelectValue placeholder="Timezone" />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="America/New_York">Eastern (ET)</SelectItem>
                  <SelectItem value="America/Chicago">Central (CT)</SelectItem>
                  <SelectItem value="America/Denver">Mountain (MT)</SelectItem>
                  <SelectItem value="America/Los_Angeles">
                    Pacific (PT)
                  </SelectItem>
                  <SelectItem value="America/Anchorage">
                    Alaska (AKT)
                  </SelectItem>
                  <SelectItem value="Pacific/Honolulu">Hawaii (HT)</SelectItem>
                </SelectContent>
              </Select>

              {/* Time Picker Dropdown */}
              {editingTime && (
                <div className="absolute top-full left-0 mt-1 p-3 bg-white border rounded-md shadow-lg z-10 w-64">
                  <div className="mb-2">
                    <Label className="text-xs text-muted-foreground">
                      Select time
                    </Label>
                  </div>
                  <div className="grid grid-cols-4 gap-1">
                    {hours.map((hour) => (
                      <div key={hour} className="space-y-1">
                        {minutes.map((minute) => (
                          <button
                            key={`${hour}:${minute}`}
                            type="button"
                            onClick={() =>
                              handleTimeSelect(editingTime, hour, minute)
                            }
                            className={`w-full text-xs py-1 px-2 rounded ${
                              (editingTime === "start" &&
                                hour === startTimeParts.hour &&
                                minute === startTimeParts.minute) ||
                              (editingTime === "end" &&
                                hour === endTimeParts.hour &&
                                minute === endTimeParts.minute)
                                ? "bg-blue-100 text-blue-800"
                                : "hover:bg-gray-100"
                            }`}
                          >
                            {hour}:{minute}
                          </button>
                        ))}
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </>
          ) : (
            <>
              <span>
                {formatTimeDisplay(startTime)} - {formatTimeDisplay(endTime)}
              </span>
              <span className="text-sm text-muted-foreground">
                ({timezone})
              </span>
            </>
          )}
        </div>

        {/* Days Display/Editor */}
        <div className="flex items-start gap-2">
          <Calendar className="w-4 h-4 text-purple-500 mt-0.5" />
          <div className="flex flex-wrap gap-1">
            {isEditing
              ? allDays.map((day) => (
                  <button
                    key={day}
                    type="button"
                    onClick={() => toggleDay(day)}
                    className={`px-2 py-1 text-xs font-medium rounded-full transition-colors ${
                      days.includes(day)
                        ? "bg-blue-100 text-blue-800 hover:bg-blue-200"
                        : "bg-gray-100 text-gray-600 hover:bg-gray-200"
                    }`}
                  >
                    {day.substring(0, 3)}
                  </button>
                ))
              : days.map((day) => (
                  <span
                    key={day}
                    className="px-2 py-1 text-xs font-medium rounded-full bg-blue-100 text-blue-800"
                  >
                    {day.substring(0, 3)}
                  </span>
                ))}
          </div>
        </div>
      </div>
    </div>
  );
}
