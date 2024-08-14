"use client";

import { useState } from "react";
import { type ComponentProps } from "react";
import { Button } from "@repo/ui/components/ui/button";
import { Alert, AlertDescription } from "@repo/ui/components/ui/alert";
import { AlertTriangle } from "lucide-react";

type Props = Omit<ComponentProps<typeof Button>, "formAction"> & {
  pendingText?: string;
  formAction: (prevState: any, formData: FormData) => Promise<any>;
  errorMessage?: string;
};

const initialState = {
  message: "",
};

export function SubmitButton({
  children,
  formAction,
  errorMessage,
  pendingText = "Submitting...",
  ...props
}: Props): JSX.Element {
  const [state, setState] = useState(initialState);
  const [isPending, setIsPending] = useState(false);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    setIsPending(true);
    const formData = new FormData(event.currentTarget);
    try {
      const result = await formAction(state, formData);
      setState(result);
    } catch (error: any) {
      setState({ message: error.message });
    } finally {
      setIsPending(false);
    }
  };

  return (
    <form onSubmit={handleSubmit} className="flex flex-col gap-y-4 w-full">
      {Boolean(errorMessage || state?.message) && (
        <Alert variant="destructive" className="w-full">
          <AlertTriangle className="h-4 w-4" />
          <AlertDescription>{errorMessage || state?.message}</AlertDescription>
        </Alert>
      )}
      <div>
        <Button
          {...props}
          type="submit"
          aria-disabled={isPending}
        >
          {isPending ? pendingText : children}
        </Button>
      </div>
    </form>
  );
}
