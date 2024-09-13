import { useState, useEffect } from 'react';
import { useAnything } from "@/context/AnythingContext";

export default function FreeTrialBadge() {
  const { subscription } = useAnything();
  const [daysLeft, setDaysLeft] = useState(0);
  const [isOnFreeTrial, setIsOnFreeTrial] = useState(false);

  useEffect(() => {
    const calculateDaysLeft = () => {
      if (!subscription.free_trial_ends_at) return 0;
      const endDate = new Date(subscription.free_trial_ends_at);
      const today = new Date();
      const diffTime = endDate.getTime() - today.getTime();
      return Math.ceil(diffTime / (1000 * 3600 * 24));
    };

    const calculatedDaysLeft = calculateDaysLeft();
    setDaysLeft(calculatedDaysLeft);
    setIsOnFreeTrial(calculatedDaysLeft > 0 && !subscription.paying_customer);
  }, [subscription.free_trial_ends_at, subscription.paying_customer]);

  if (!isOnFreeTrial) {
    return null;
  }

  return (
    <div className="text-xs bg-blue-100 text-blue-800 p-2 rounded-full flex items-center">
      Free Trial Tasks Used: {subscription.free_trial_task_usage}/{subscription.free_trial_task_limit} tasks • {daysLeft} days left
    </div>
  );
}

