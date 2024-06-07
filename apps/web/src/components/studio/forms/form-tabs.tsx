import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import { useParams } from 'next/navigation'
// "use client"
import { PartyPopper } from "lucide-react";
import DashboardTitleWithAction from "@/components/workflows/dashboard-title-with-action";
import { Separator } from "@/components/ui/separator";
import { useAnything } from "@/context/AnythingContext";
// import { useParams } from 'next/navigation'
import { useEffect, useState } from "react";
// import { DBWorkflow } from "@/context/FlowsProvider";
import DashboardTitleWithNavigation from "@/components/workflows/dahsbloard-title-with-navigation";

export default function FormTabs() {

//     const { workflow_version }
// } = useAnything();

const [workflow, setWorkflow] = useState<any | undefined>(undefined);
const params = useParams<{ workflowId: string; }>()

// useEffect(() => {
//     const fetchData = async () => {
//         console.log("params in useEffect", params);
//         if (params.workflowId) {
//             let flow = await getFlowById(params.workflowId);
//             setWorkflow(flow);
//         }
//     };

//     fetchData();
// }, [params.workflowId, flows]);

return (
    <Tabs defaultValue="account" className="w-[400px]">
        <TabsList>
            <TabsTrigger value="account">Account</TabsTrigger>
            <TabsTrigger value="password">Password</TabsTrigger>
        </TabsList>
        <TabsContent value="account">Make changes to your account here.</TabsContent>
        <TabsContent value="password">Change your password here.</TabsContent>
    </Tabs>
)
}

