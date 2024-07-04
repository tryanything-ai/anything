"use client"

import { useEffect, useState } from "react"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import Link from "next/link"
import {
    Sheet,
    SheetClose,
    SheetContent,
    SheetDescription,
    SheetFooter,
    SheetHeader,
    SheetTitle,
    SheetTrigger,
} from "@/components/ui/sheet"
import { useAnything } from "@/context/AnythingContext"
import { Package, ShoppingCart, Home, Users, LineChart } from "lucide-react"
import { Badge } from "@/components/ui/badge"
import { ActionPanelLeftPanelNavigation } from "./left-panel-navigation"
import api from "@/lib/anything-api"


export function StudioActionsSheet() {
    const { workflow } = useAnything();
    const [actions, setActions] = useState<any>([]);

    const fetchActions = async () => {
        try {
            const res = await api.action_templates.getActionTemplates();
            setActions(res);
        } catch (error) {
            console.error('Error fetching actions:', error);
        }
    }

    useEffect(() => {
        fetchActions();
    }, []);

    return (
        <Sheet open={workflow.showingActionSheet} onOpenChange={(open) => workflow.setShowingActionSheet(open)}>
            <SheetContent side={"bottom"} className="h-4/5">
                <SheetHeader>
                    <SheetTitle>Actions Library</SheetTitle>
                    <SheetDescription>
                        Add a new step to your workflow to automate your tasks.
                    </SheetDescription>
                </SheetHeader>
                <div className="py-4 flex flex-row">
                    {/* Left Hand Panel */}
                    <ActionPanelLeftPanelNavigation />
                    <div className="bg-pink-400 flex-1 w-full h-full mx-7">{JSON.stringify(actions, null, 3)}</div>
                </div>
            </SheetContent>
        </Sheet>
    )
}
