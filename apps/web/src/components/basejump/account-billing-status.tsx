"use client";

import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@repo/ui/components/ui/card";
import { Alert, AlertDescription } from "@repo/ui/components/ui/alert";
import { Button } from "@repo/ui/components/ui/button";
import { useAnything } from "@/context/AnythingContext";
import api from "@repo/anything-api";

const returnUrl: string =
  process.env.NODE_ENV === "production"
    ? `https://${process.env.NEXT_PUBLIC_VERCEL_URL}`
    : `http://${process.env.NEXT_PUBLIC_VERCEL_URL}`;

export default function AccountBillingStatus(): JSX.Element {
  const {
    subscription,
    accounts: { selectedAccount },
  } = useAnything();

  const manageSubscription = async () => {
    // Implement manage subscription logic here
    //TODO: Create session for managing invoices.
    if (!selectedAccount) {
      console.error("No selected account");
      return;
    }
    console.log(
      "Navigating to billing portal for account:",
      selectedAccount.account_id,
    );
    try {
      const res = await api.billing.getBillingPortalLink(
        selectedAccount.account_id,
        returnUrl,
      );
      console.log("Received response:", res);
      if (res.portal_url) {
        console.log(
          "Redirecting to billing portal URL:",
          res.billing_portal_url,
        );
        window.location.href = res.portal_url;
      } else {
        console.error("No billing portal URL received");
      }
    } catch (error) {
      console.error("Error managing subscription:", error);
    }

    // window.location.href =
    // "https://billing.stripe.com/p/login/test_6oE6qIbcTe8o9fqfYY";
  };

  const setupNewSubscription = async () => {
    console.log("Starting setupNewSubscription");
    // Navigate to the Stripe checkout page
    if (!selectedAccount) {
      console.log("No selected account, returning");
      return;
    }
    try {
      console.log(
        "Fetching checkout link for account:",
        selectedAccount.account_id,
      );
      const res = await api.billing.getCheckoutLink(
        selectedAccount.account_id,
        returnUrl,
      );
      console.log("Received response:", res);
      if (res.checkout_url) {
        console.log("Redirecting to checkout URL:", res.checkout_url);
        window.location.href = res.checkout_url;
      } else {
        console.error("No checkout URL received");
      }
    } catch (error) {
      console.error("Error setting up new subscription:", error);
      // Handle the error appropriately, e.g., show an error message to the user
    }
  };

  return (
    <Card>
      <CardHeader>
        <CardTitle>Billing Status</CardTitle>
        <CardDescription>
          A quick overview of your billing status
        </CardDescription>
      </CardHeader>
      <CardContent>
        <p>
          Status:{" "}
          {subscription.paying_customer
            ? "Active Subscription"
            : "No Active Subscription"}
        </p>
        <p>Free Trial: {subscription.trial_ended ? "Expired" : "Active"}</p>
        {!subscription.trial_ended && (
          <div>
            <p>
              Free Trial Ends:{" "}
              {subscription.free_trial_ends_at
                ? new Date(subscription.free_trial_ends_at).toLocaleDateString()
                : "Not available"}
            </p>
            or
            <p>When you've used all free tasks.</p>
            <p>
              You have currently used {subscription.free_trial_task_usage} of{" "}
              {subscription.free_trial_task_limit} free tasks.
            </p>
          </div>
        )}
      </CardContent>
      <CardFooter>
        {!subscription.paying_customer ? (
          <Button onClick={setupNewSubscription}>Subscribe Now</Button>
        ) : (
          <Button onClick={manageSubscription}>Manage Subscription</Button>
        )}
      </CardFooter>
    </Card>
  );
}
