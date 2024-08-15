import { type ClassValue, clsx } from "clsx"
import { twMerge } from "tailwind-merge"
import { parseISO, differenceInMilliseconds, formatDuration, intervalToDuration } from 'date-fns';

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

export function formatTimeDifference(startTime: string, endTime: string): string {
    // Parse the timestamps
    const start = parseISO(startTime);
    const end = parseISO(endTime);

    // Calculate the difference in milliseconds
    const diffInMs = differenceInMilliseconds(end, start);

    // Format the difference
    let formattedDiff: string;

    if (diffInMs < 60000) {
        const diffInSeconds = diffInMs / 1000;
        formattedDiff = `${diffInSeconds.toFixed(2)} seconds`;
    } else {
        const duration = intervalToDuration({ start, end });
        formattedDiff = formatDuration(duration, { format: ['minutes', 'seconds'] });
    }

    return formattedDiff;
}
