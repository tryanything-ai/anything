export const PageHeader = ({
  title,
  buttonLabel,
  callback,
}: {
  title: string;
  buttonLabel: string;
  callback: () => void;
}) => {
  return (
    <div className="flex flex-row w-full justify-between">
      <div className="h2">{title}</div>
      <button className="btn btn-primary m-1 ml-4" onClick={callback}>
        {buttonLabel}
      </button>
    </div>
  );
};
