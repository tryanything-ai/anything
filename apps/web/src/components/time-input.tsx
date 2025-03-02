import { useState, useEffect } from "react";
import { Input } from "@repo/ui/components/ui/input";

interface TimeInputProps {
  id?: string;
  value: string;
  onChange: (value: string) => void;
  disabled?: boolean;
  className?: string;
}

export function TimeInput({
  id,
  value,
  onChange,
  disabled = false,
  className = "",
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
    <div className={`flex items-center space-x-2 ${className}`}>
      <Input
        id={id ? `${id}-hours` : undefined}
        type="number"
        min={0}
        max={23}
        value={hours}
        onChange={handleHoursChange}
        disabled={disabled}
        className="w-20"
        placeholder="HH"
      />
      <span className="text-lg">:</span>
      <Input
        id={id ? `${id}-minutes` : undefined}
        type="number"
        min={0}
        max={59}
        value={minutes}
        onChange={handleMinutesChange}
        disabled={disabled}
        className="w-20"
        placeholder="MM"
      />
    </div>
  );
}
