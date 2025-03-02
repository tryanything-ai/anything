import { useState, useEffect } from "react";
import { Input } from "@repo/ui/components/ui/input";
import { Clock } from "lucide-react";

interface TimeInputProps {
  id?: string;
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  className?: string;
  showIcon?: boolean;
}

export function TimeInput({
  id,
  value,
  onChange,
  disabled = false,
  className = "",
  showIcon = true,
}: TimeInputProps) {
  const [hours, setHours] = useState("09");
  const [minutes, setMinutes] = useState("00");

  // Parse the initial value
  useEffect(() => {
    if (value) {
      const [h, m] = value.split(":");
      setHours(h || "09");
      setMinutes(m || "00");
    }
  }, [value]);

  // Update the parent component when hours or minutes change
  const handleHoursChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let newHours = e.target.value;

    // Ensure hours are between 0-23
    const numericHours = parseInt(newHours, 10);
    if (isNaN(numericHours)) {
      newHours = "00";
    } else if (numericHours < 0) {
      newHours = "00";
    } else if (numericHours > 23) {
      newHours = "23";
    } else {
      newHours = numericHours.toString().padStart(2, "0");
    }

    setHours(newHours);
    onChange(`${newHours}:${minutes}:00`);
  };

  const handleMinutesChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    let newMinutes = e.target.value;

    // Ensure minutes are between 0-59
    const numericMinutes = parseInt(newMinutes, 10);
    if (isNaN(numericMinutes)) {
      newMinutes = "00";
    } else if (numericMinutes < 0) {
      newMinutes = "00";
    } else if (numericMinutes > 59) {
      newMinutes = "59";
    } else {
      newMinutes = numericMinutes.toString().padStart(2, "0");
    }

    setMinutes(newMinutes);
    onChange(`${hours}:${newMinutes}:00`);
  };

  return (
    <div className={`flex items-center ${className}`}>
      {showIcon && <Clock className="w-4 h-4 text-orange-500 mr-2" />}
      <div className="flex items-center border rounded-md px-3 py-2 bg-background">
        <Input
          id={id ? `${id}-hours` : undefined}
          type="number"
          min={0}
          max={23}
          value={hours}
          onChange={handleHoursChange}
          disabled={disabled}
          className="w-12 border-0 p-0 text-center focus-visible:ring-0 focus-visible:ring-offset-0"
          placeholder="HH"
        />
        <span className="text-lg mx-1">:</span>
        <Input
          id={id ? `${id}-minutes` : undefined}
          type="number"
          min={0}
          max={59}
          value={minutes}
          onChange={handleMinutesChange}
          disabled={disabled}
          className="w-12 border-0 p-0 text-center focus-visible:ring-0 focus-visible:ring-offset-0"
          placeholder="MM"
        />
      </div>
    </div>
  );
}
