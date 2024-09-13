"use client";

import { useState } from "react";
import { CheckIcon } from "@heroicons/react/20/solid";

const frequencies = [
  { value: "monthly", label: "Monthly", priceSuffix: "/month" },
  { value: "annually", label: "Annually", priceSuffix: "/year" },
];
const tiers = [
  {
    name: "Pay As You Go",
    id: "payg",
    href: "#",
    price: { monthly: "$9.99", annually: "$50" },
    description: "A plan that scales to Anything.",
    features: [
      "7 Day Free Trial",
      "10K included tasks",
      "$0.99 / 1k tasks after that",
      "Unlimited users",
      "Unlimited workflows",
      "Unlimited connections",
    ],
    featured: false,
    cta: "Subscribe",
  },
  {
    name: "Done For You",
    id: "dfy",
    href: "#",
    price: "Custom",
    description:
      "Our team works hand in hand to implement automations for your business.",
    features: [
      "Implement AI Automations for your business",
      "Maintain and Improve Automations",
      "Educate your team",
      "Dedicated Support",
    ],
    featured: true,
    cta: "Contact sales",
  },
];

function classNames(...classes: any) {
  return classes.filter(Boolean).join(" ");
}

export default function PricingGroup() {
  const [frequency, setFrequency] = useState(frequencies[0]);

  return (
    <div className="bg-white py-24 sm:py-32">
      <div className="mx-auto max-w-7xl px-6 lg:px-8">
        <div className="mx-auto max-w-4xl text-center">
          <h2 className="text-base font-semibold leading-7 text-purple-600">
            Pricing
          </h2>
          <p className="mt-2 text-4xl font-bold tracking-tight text-gray-900 sm:text-5xl">
            Choose the right plan for you
          </p>
        </div>
        <p className="mx-auto mt-6 max-w-2xl text-center text-lg leading-8 text-gray-600">
          Whether you're looking for a self-service solution or a fully managed
          service, we've got you covered.
        </p>
        {/* <div className="mt-16 flex justify-center">
          <fieldset aria-label="Payment frequency">
            <RadioGroup
              value={frequency}
              onChange={setFrequency}
              className="grid grid-cols-2 gap-x-1 rounded-full p-1 text-center text-xs font-semibold leading-5 ring-1 ring-inset ring-gray-200"
            >
              {frequencies.map((option) => (
                <Radio
                  key={option.value}
                  value={option}
                  className="cursor-pointer rounded-full px-2.5 py-1 text-gray-500 data-[checked]:bg-purple-600 data-[checked]:text-white"
                >
                  {option.label}
                </Radio>
              ))}
            </RadioGroup>
          </fieldset>
        </div> */}
        <div className="isolate mx-auto mt-10 grid max-w-md grid-cols-1 gap-8 lg:mx-0 lg:max-w-none lg:grid-cols-2">
          {tiers.map((tier) => (
            <div
              key={tier.id}
              className={classNames(
                tier.featured ? "bg-gray-900 ring-gray-900" : "ring-gray-200",
                "rounded-3xl p-8 ring-1 xl:p-10",
              )}
            >
              <h3
                id={tier.id}
                className={classNames(
                  tier.featured ? "text-white" : "text-gray-900",
                  "text-lg font-semibold leading-8",
                )}
              >
                {tier.name}
              </h3>
              <p
                className={classNames(
                  tier.featured ? "text-gray-300" : "text-gray-600",
                  "mt-4 text-sm leading-6",
                )}
              >
                {tier.description}
              </p>
              <p className="mt-6 flex items-baseline gap-x-1">
                <span
                  className={classNames(
                    tier.featured ? "text-white" : "text-gray-900",
                    "text-4xl font-bold tracking-tight",
                  )}
                >
                  {typeof tier.price === "string"
                    ? tier.price
                    : frequency &&
                      tier.price[frequency.value as keyof typeof tier.price]}
                </span>
                {typeof tier.price !== "string" && frequency ? (
                  <span
                    className={classNames(
                      tier.featured ? "text-gray-300" : "text-gray-600",
                      "text-sm font-semibold leading-6",
                    )}
                  >
                    {frequency.priceSuffix}
                  </span>
                ) : null}
              </p>
              <a
                href={tier.href}
                aria-describedby={tier.id}
                className={classNames(
                  tier.featured
                    ? "bg-white/10 text-white hover:bg-white/20 focus-visible:outline-white"
                    : "bg-purple-600 text-white shadow-sm hover:bg-purple-500 focus-visible:outline-purple-600",
                  "mt-6 block rounded-md px-3 py-2 text-center text-sm font-semibold leading-6 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2",
                )}
              >
                {tier.cta}
              </a>
              <ul
                role="list"
                className={classNames(
                  tier.featured ? "text-gray-300" : "text-gray-600",
                  "mt-8 space-y-3 text-sm leading-6 xl:mt-10",
                )}
              >
                {tier.features.map((feature) => (
                  <li key={feature} className="flex gap-x-3">
                    <CheckIcon
                      aria-hidden="true"
                      className={classNames(
                        tier.featured ? "text-white" : "text-purple-600",
                        "h-6 w-5 flex-none",
                      )}
                    />
                    {feature}
                  </li>
                ))}
              </ul>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
