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
import api from "@/lib/anything-api";

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
    window.location.href =
      "https://billing.stripe.com/p/login/test_6oE6qIbcTe8o9fqfYY";
  };

  const setupNewSubscription = async () => {
    console.log("Starting setupNewSubscription");
    // Navigate to the Stripe checkout page
    if (!selectedAccount) {
      console.log("No selected account, returning");
      return;
    }
    try { 
      console.log("Fetching checkout link for account:", selectedAccount.account_id);
      const res = await api.billing.getCheckoutLink(selectedAccount.account_id, returnUrl);
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
      </CardContent>
      <CardFooter>
        {/* {!subscription.paying_customer ? ( */}
        <Button onClick={setupNewSubscription}>Subscribe Now</Button>
        {/* // ) : ( */}
        <Button onClick={manageSubscription}>Manage Subscription</Button>
        {/* // )} */}
      </CardFooter>
    </Card>
  );
}
