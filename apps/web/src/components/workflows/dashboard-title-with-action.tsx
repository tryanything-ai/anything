import { Button } from "../ui/button";

interface Props {
    title: string;
    description: string;
    action: () => void
}
export default function DashboardTitleWithAction({ title, description, action }: Props) {
    return (
        <div className="max-w-screen-xl mx-auto flex flex-row justify-between">
            <div className="flex flex-row items-end">
                <h2 className="text-2xl font-bold tracking-tight ">{title}</h2>
                <p className="text-muted-foreground ml-2 ">
                    {description}
                </p>
            </div>

            <div className="flex items-center">
                <Button onClick={action}>Create</Button>
            </div>
        </div>
    )
}