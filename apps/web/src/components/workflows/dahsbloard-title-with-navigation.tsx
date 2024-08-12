import Link from "next/link";
import { Button } from "../ui/button";

interface Props {
    title: string;
    description: string;
    href: string;
    children?: React.ReactNode;
}

export default function DashboardTitleWithNavigation({ title, description, href, children }: Props) {
    return (
        <div className="max-w-screen-xl mx-auto flex flex-row justify-between">
            <div className="flex flex-row items-end">
                <h2 className="text-2xl font-bold tracking-tight ">{title}</h2>
                <p className="text-muted-foreground ml-2 ">
                    {description}
                </p>
            </div>
            {children}
            <div className="flex items-center">
                <Link href={href}>
                    <Button>
                        Edit Wofklow
                    </Button>
                </Link>
            </div>
        </div >
    )
}