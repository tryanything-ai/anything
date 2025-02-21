"use client";

import { useEffect, useState, useRef } from "react";
import { format, parseISO } from "date-fns";
import { Card } from "@repo/ui/components/ui/card";
import { Button } from "@repo/ui/components/ui/button";
import { Phone, Play, Pause } from "lucide-react";
import api from "@repo/anything-api";
import { useAnything } from "@/context/AnythingContext";
import { useParams } from "next/navigation";
import { createClient } from "@/lib/supabase/client";

interface CallCost {
  transport: number;
  stt: number;
  llm: number;
  tts: number;
  vapi: number;
  total: number;
}

interface Message {
  role: "bot" | "user" | "system" | "tool_calls" | "tool_call_result";
  message: string;
  toolCalls?: {
    function: {
      name: string;
      arguments: string;
    };
  }[];
}

interface Call {
  id: string;
  analysis: {
    successEvaluation: boolean;
    summary: string;
  };
  createdAt: string;
  status: string;
  endedReason?: string;
  startedAt?: string;
  endedAt?: string;
  cost: number;
  costBreakdown: CallCost;
  artifact?: {
    recordingUrl?: string;
    transcript?: string;
    messages?: Message[];
  };
  customer?: {
    number: string;
    name?: string;
  };
}

export default function InboxPage() {
  const params = useParams();
  const {
    accounts: { selectedAccount },
  } = useAnything();

  const [selectedCall, setSelectedCall] = useState<Call | null>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [calls, setCalls] = useState<Call[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const audioRef = useRef<HTMLAudioElement | null>(null);

  const getCalls = async () => {
    if (!selectedAccount) {
      setIsLoading(false);
      return;
    }

    try {
      setIsLoading(true);
      const calls = await api.agents.getVapiCalls(
        await createClient(),
        selectedAccount.account_id,
      );
      console.log("calls: ", calls);
      setCalls(calls);
    } catch (error) {
      console.error("Error fetching calls:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    getCalls();
  }, []);

  useEffect(() => {
    if (selectedCall?.artifact?.recordingUrl) {
      audioRef.current = new Audio(selectedCall.artifact.recordingUrl);
      audioRef.current.addEventListener("ended", () => setIsPlaying(false));
    }
    return () => {
      if (audioRef.current) {
        audioRef.current.pause();
        audioRef.current.removeEventListener("ended", () =>
          setIsPlaying(false),
        );
      }
    };
  }, [selectedCall]);

  useEffect(() => {
    if (audioRef.current) {
      if (isPlaying) {
        audioRef.current.play();
      } else {
        audioRef.current.pause();
      }
    }
  }, [isPlaying]);

  const getDuration = (call: Call) => {
    if (call.startedAt && call.endedAt) {
      const start = parseISO(call.startedAt);
      const end = parseISO(call.endedAt);
      const durationMs = end.getTime() - start.getTime();
      const minutes = Math.floor(durationMs / 60000);
      const seconds = Math.floor((durationMs % 60000) / 1000);
      return `${minutes}:${seconds.toString().padStart(2, "0")}`;
    }
    return "N/A";
  };

  return (
    <div className="flex h-[calc(100vh-4rem)]">
      {/* Left sidebar - Call list */}
      <div className="w-1/4 overflow-y-auto border-r">
        {calls.map((call) => (
          <div
            key={call.id}
            className={`p-4 border-b cursor-pointer hover:bg-gray-50 ${
              selectedCall?.id === call.id ? "bg-gray-100" : ""
            }`}
            onClick={() => setSelectedCall(call)}
          >
            <div className="flex items-center gap-2">
              <Phone className="h-4 w-4" />
              <span className="font-medium">
                {call.customer?.number || "Unknown"}
              </span>
            </div>
            <div className="text-sm text-gray-500 mt-1">
              {format(parseISO(call.createdAt), "PPp")}
            </div>
            <div className="text-sm text-gray-500">
              Duration: {getDuration(call)}
            </div>

            <div
              className={`inline-block px-2 py-1 rounded-full text-xs font-medium ${
                call.status === "completed"
                  ? "bg-green-100 text-green-800"
                  : "bg-gray-100 text-gray-800"
              }`}
            >
              {call.status}
            </div>
          </div>
        ))}
      </div>

      {/* Right panel - Call details */}
      <div className="w-3/4 overflow-y-auto">
        {selectedCall ? (
          <div className="p-4">
            <div className="p-6">
              <div className="flex items-center justify-between mb-6">
                <div>
                  <h2 className="text-2xl font-bold">
                    {selectedCall.customer?.name ||
                      selectedCall.customer?.number ||
                      "Unknown"}
                  </h2>
                  <p className="text-gray-500">
                    {format(parseISO(selectedCall.createdAt), "PPpp")}
                  </p>
                  {selectedCall.analysis.successEvaluation && (
                    <div className="mt-2">
                      <p className="text-sm font-bold">Summary</p>
                      <p className="text-sm text-gray-500 max-w-xl">
                        {selectedCall.analysis.summary}
                      </p>
                    </div>
                  )}
                </div>
                {selectedCall.artifact?.recordingUrl && (
                  <Button
                    onClick={() => setIsPlaying(!isPlaying)}
                    className="flex items-center gap-2"
                  >
                    {isPlaying ? (
                      <>
                        <Pause className="h-4 w-4" /> Pause
                      </>
                    ) : (
                      <>
                        <Play className="h-4 w-4" /> Play Recording
                      </>
                    )}
                  </Button>
                )}
              </div>

              <div className="bg-gray-50 p-4 rounded-lg">
                <h3 className="font-semibold mb-2">Conversation</h3>
                <div className="">
                  {selectedCall.artifact?.messages
                    ?.filter((message) => message.role !== "system")
                    .map((message, index) => (
                      <div key={index} className={`p-3`}>
                        <p
                          className={`text-sm font-bold mb-1 ${
                            message.role === "bot" 
                              ? "text-blue-600" 
                              : message.role === "user"
                              ? "text-green-600"
                              : message.role === "tool_calls"
                              ? "text-purple-600" 
                              : message.role === "tool_call_result"
                              ? "text-orange-600"
                              : "text-gray-600"
                          }`}
                        >
                          {message.role === "bot" 
                            ? "Assistant"
                            : message.role === "user" 
                            ? "User"
                            : message.role === "tool_calls"
                            ? "Tool Call"
                            : message.role === "tool_call_result" 
                            ? "Tool Result"
                            : message.role}
                        </p>
                        {message.role === "tool_calls" ? (
                          <div className="text-sm">
                            {message.toolCalls?.map((call, i) => (
                              <div key={i} className="mb-2">
                                <p className="font-medium">Function: {call.function.name}</p>
                                <p className="text-gray-600">Args: {call.function.arguments}</p>
                              </div>
                            ))}
                          </div>
                        ) : (
                          <p className="whitespace-pre-wrap">{message.message}</p>
                        )}
                      </div>
                    ))}
                </div>
              </div>
            </div>
          </div>
        ) : (
          <div className="h-full flex items-center justify-center text-gray-500">
            Select a call to view details
          </div>
        )}
      </div>
    </div>
  );
}
