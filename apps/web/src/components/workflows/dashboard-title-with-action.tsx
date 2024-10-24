import { Button } from "@repo/ui/components/ui/button";

interface Action {
  label: string;
  onClick: () => void;
}

interface Props {
  title: string;
  description: string;
  actions: Action[];
}

export default function DashboardTitleWithAction({
  title,
  description,
  actions,
}: Props): JSX.Element {
  return (
    <div className="max-w-screen-xl mx-auto flex flex-row justify-between">
      <div className="flex flex-row items-end">
        <h2 className="text-2xl font-bold tracking-tight ">{title}</h2>
        <p className="text-muted-foreground ml-2 ">{description}</p>
      </div>

      <div className="flex items-center space-x-2">
        {actions.map((action, index) => (
          <Button key={index} onClick={action.onClick}>
            {action.label}
          </Button>
        ))}
      </div>
    </div>
  );
}
