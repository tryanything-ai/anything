import { ComponentType } from "react";

export const HeaderButton = ({
  children,
  callback,
  className,
}: {
  children: any;
  callback: () => void;
  className?: string;
}) => {
  return (
    <button className={`btn btn-primary m-1 ml-4 ${className}`} onClick={callback}>
      {children}
    </button>
  );
};
export const PageHeader = ({
  title,
  buttonLabel,
  callback,
  ActionComponent,
}: {
  title: string;
  buttonLabel?: string;
  callback?: () => void;
  ActionComponent?: ComponentType<any>;
}) => {
  return (
    <div className="flex flex-row w-full justify-between">
      <div className="h2">{title}</div>
      {ActionComponent ? (
        <ActionComponent />
      ) : (
        <HeaderButton callback={callback}>{buttonLabel}</HeaderButton>
      )}
    </div>
  );
};
