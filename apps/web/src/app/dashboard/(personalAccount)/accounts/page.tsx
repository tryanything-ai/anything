"use client"

import { useState, useEffect } from "react";
import { PartyPopper } from "lucide-react";
import api from "@/lib/anything-api";

export default function AccountsPage() {
    const [secrets, setSecrets] = useState<any[]>([]);

    const fetchSecrets = async () => {
        try {
            const response = await api.secrets.getSecrets();
            setSecrets(response);
        } catch (error) {
            console.error('Error fetching secrets:', error);
        }
    }

    useEffect(() => {
        fetchSecrets();
    }, []);

    return (
        <div className="flex flex-col gap-y-4 py-12 h-full w-full items-center justify-center content-center max-w-screen-md mx-auto text-center">
            <h1 className="text-2xl font-bold">Accounts</h1>
            <p>
                {JSON.stringify(secrets)}
            </p>
        </div>
    )
}