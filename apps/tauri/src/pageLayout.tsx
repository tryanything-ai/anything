import { ReactNode } from "react";

export default function PageLayout({ children }: { children: ReactNode }) {
  return (
    <div className="flex flex-row p-14 h-full w-full hide-scrollbar overflow-scroll">
      {children}
    </div>
  );
}
