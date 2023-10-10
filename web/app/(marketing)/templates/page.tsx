// "use client";
// import { Metadata } from "next";
import { clsx } from "clsx";
import { MockNewFlows } from "../../../../tauri/src/utils/newNodes";
import { TemplateCard } from "@/components/templateCard";
import { Database } from "@/types/supabase.types";

type Flow = Database["public"]["Tables"]["flow_templates"]["Row"];

const mockRows: Flow[] = [
  {
    anonymous: true,
    slug: "flow-1",
    created_at: "2023-04-15T12:30:00.000Z",
    flow_json: JSON.stringify(MockNewFlows[0]),
    flow_name: "Send Message on File Change",
    flow_templates_version: "v1.0.1",
    published: true,
    publisher_id: "pub12345",
    template_id: "temp1",
  },
  {
    anonymous: false,
    slug: "flow-2",
    created_at: "2023-03-20T10:20:00.000Z",
    flow_json: JSON.stringify(MockNewFlows[0]),
    flow_name: "Flow 2",
    flow_templates_version: "v1.0.2",
    published: false,
    publisher_id: "pub67890",
    template_id: "temp2",
  },
  {
    anonymous: null,
    slug: "flow-3",
    created_at: "2023-04-01T15:15:00.000Z",
    flow_json: JSON.stringify(MockNewFlows[0]),
    flow_name: "Flow 3",
    flow_templates_version: "v1.0.3",
    published: true,
    publisher_id: "pub11121",
    template_id: "temp3",
  },
  {
    anonymous: true,
    slug: "flow-4",
    created_at: "2023-05-10T09:10:00.000Z",
    flow_json: JSON.stringify(MockNewFlows[0]),
    flow_name: "Flow 4",
    flow_templates_version: "v1.0.4",
    published: false,
    publisher_id: "pub23111",
    template_id: "temp4",
  },
  {
    anonymous: false,
    slug: "flow-5",
    created_at: "2023-04-25T14:50:00.000Z",
    flow_json: JSON.stringify(MockNewFlows[1]),
    flow_name: "Flow 5",
    flow_templates_version: "v1.0.5",
    published: true,
    publisher_id: "pub99887",
    template_id: "temp5",
  },
];
export default function TemplatePage() {
  return (
    <>
      {/* Hero Copy */}
      <div className="mt-16 flex flex-col items-center gap-4">
        <h1 className="md:h1 h2 w-full px-4 text-center md:w-[805px] md:px-0">
          Anything Templates
        </h1>
        <p className="body-xl w-full px-4 text-center text-slate-11 md:w-[572px] md:px-0">
          Automate anything with ready to use templates
        </p>
      </div>

      {/* Pricing */}
      <div className="my-16 flex flex-col items-center">
        <div className="grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-6 mx-auto max-w-7xl">
          {mockRows.map((template, index) => (
            <TemplateCard key={index} template={template} />
          ))}
          {/* {pricing.map((plan, planIndex) => (
            <div
              key={planIndex}
              className={clsx(
                "flex h-[353px] flex-col gap-8 rounded-lg bg-slate-2 px-6 py-12",
                plan.promoted === true ? "border-[3px] border-crimson-6" : ""
              )}
            >
              <div className="flex flex-col gap-2">
                <h6 className="body-semibold text-slate-12">{plan.name}</h6>
                <div className="flex items-center gap-3">
                  <h5 className="text-[32px] font-bold leading-9">${plan.base / 100}</h5>
                  <div className="flex flex-col items-start">
                    <span className="caption">{plan.currency.toUpperCase()}</span>
                    <span className="caption-s text-slate-11">Billed {plan.interval}</span>
                  </div>
                </div>
              </div>
              {plan.promoted ? (
                <SignUpButton type="primary">Buy this plan</SignUpButton>
              ) : (
                <SignUpButton type="outline">Buy this plan</SignUpButton>
              )}

              <div className="flex flex-col gap-4">
                {plan.features.map((feature, featureIndex) => (
                  <div key={featureIndex} className="flex items-center gap-3">
                    <CheckBoxIcon
                      className={clsx(
                        "h-6 w-6 ",
                        plan.promoted ? "stroke-crimson-9" : "stroke-slate-11"
                      )}
                    />
                    <p className="body text-slate-11">{feature}</p>
                  </div>
                ))}
              </div>
            </div>
          ))} */}
        </div>
      </div>
    </>
  );
}
