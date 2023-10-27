import { ReactNode } from "react";

export default function PageLayout({ children }: { children: ReactNode }) {
  return <div className="p-14 flex h-full w-full flex-row">{children}</div>;
}
