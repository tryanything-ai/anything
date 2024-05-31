import { cva, VariantProps } from "class-variance-authority";
import Link from "next/link";
import React, { ComponentProps } from "react";

import { cn } from "@/lib/utils";

// https://cva.style/docs
export const buttonVariants = cva("", {
  variants: {
    variant: {
      daisy_primary:
        "btn btn-block md:w-56 rounded-[4px] bg-crimson-9 py-1 px-3 text-white hover:bg-crimson-10",
      daisy_outline:
        "btn btn-block md:w-56 rounded-[4px] py-1 px-3 bg-slate-2 ring-1 ring-inset ring-slate-6 text-slate-11 hover:bg-slate-3 hover:text-slate-12",

      primary:
        "rounded-[4px] bg-crimson-9 py-1 px-3 text-white hover:bg-crimson-10",
      secondary:
        "rounded-[4px] bg-crimson-3 py-1 px-3 text-crimson-11 hover:bg-crimson-4 ring-1 ring-inset ring-crimson-6",
      outline:
        "rounded-[4px] py-1 px-3 bg-slate-2 ring-1 ring-inset ring-slate-6 text-slate-11 hover:bg-slate-3 hover:text-slate-12",
      text: "text-crimson-9 hover:text-crimson-10",
    },
  },
  defaultVariants: {
    variant: "primary",
  },
});

type ButtonOrLinkProps = ComponentProps<"button"> & ComponentProps<"a">;

export interface ButtonProps
  extends ButtonOrLinkProps,
    VariantProps<typeof buttonVariants> {}

export const Button = ({
  variant,
  className,
  children,
  ...props
}: ButtonProps) => {
  let Component: any = props.href ? Link : "button";

  return (
    <Component
      className={cn(buttonVariants({ variant, className }))}
      {...props}
    >
      {children}
    </Component>
  );
};
