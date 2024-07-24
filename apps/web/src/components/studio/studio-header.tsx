"use client"

import { Button } from "@/components/ui/button"
import { useRouter } from 'next/navigation';
import { XIcon } from "lucide-react"

export default function StudioHeader({ flow_name, savingStatus }: { flow_name: string, savingStatus: string }) {
    const router = useRouter();

    const handleBack = () => {
        router.back();
    };

    return (
        <header className="sticky top-0 z-10 flex h-[57px] items-center gap-1 border-b bg-background px-4">
            <div className="border-b p-2">
                <Button onClick={handleBack} variant="outline" size="icon" aria-label="Home">
                    <XIcon className="size-5 fill-foreground" />
                </Button>
            </div>
            <h1 className="text-xl font-semibold inline">{flow_name} <span className="text-sm font-normal">{"  "}{savingStatus}</span></h1>
            <Button variant="outline" size="sm" className="ml-auto gap-1.5 text-sm">
                <ShareIcon className="size-3.5" />
                Share
            </Button>
        </header>
    )
}


function ShareIcon(props: any) {
    return (
        <svg
            {...props}
            xmlns="http://www.w3.org/2000/svg"
            width="24"
            height="24"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
        >
            <path d="M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" />
            <polyline points="16 6 12 2 8 6" />
            <line x1="12" x2="12" y1="2" y2="15" />
        </svg>
    )
}