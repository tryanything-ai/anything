import { ReactNode } from "react";

const TemplateGrid = ({ children }: { children: ReactNode }) => {
  return (
    <div className ="3xl:grid-cols-4 mx-auto grid max-w-7xl grid-cols-1 gap-6 lg:grid-cols-2 xl:grid-cols-3">
      {children}
    </div>
  );
};

export default TemplateGrid;
