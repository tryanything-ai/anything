import {
    createContext,
    ReactNode,
    useContext,
    useState,
    useEffect
} from "react";
import { v4 as uuidv4 } from "uuid";
import { useParams } from "react-router-dom";
import { useFlowContext } from "../context/FlowProvider";
import api from "../tauri_api/api";

export enum SessionStatus {
    WAITING = "WAITING",
    PROCESSING = "PROCESSING",
    ERROR = "ERROR",
    COMPLETE = "COMPLETE"
}

interface DebuggingContextInterface {
    startDebuggingSession: () => void;
    stopDebuggingSession: () => void;
    session_id: string;
    events: any[];
    debugging: boolean;
    session_status: SessionStatus | null;
}

export const DebuggingContext = createContext<DebuggingContextInterface>({
    startDebuggingSession: () => { },
    stopDebuggingSession: () => { },
    session_id: "",
    events: [],
    debugging: false,
    session_status: null
});

export const useDebuggingContext = () => useContext(DebuggingContext);


export const DebuggingProvider = ({ children }: { children: ReactNode }) => {
    const { flowFrontmatter, getTrigger } = useFlowContext();
    const [debugging, setDebugging] = useState(false);
    const [session_id, setSessionId] = useState<string>("");
    const [events, setEvents] = useState<any[]>([]);
    const [session_status, setSessionStatus] = useState<SessionStatus | null>(null);

    //polling state
    const [isPolling, setIsPolling] = useState(false);
    const [pollingIntervalId, setPollingIntervalId] = useState(null);

    const fetchEvents = async () => {
        try {
            console.log("fetching events for session_id: ", session_id);
            const newEvents: any = await api.flows.fetchSessionEvents(session_id);
            setEvents(newEvents.events);

            const flow_session_status = newEvents.events[0].flow_session_status;
            console.log("flow_session_status: ", flow_session_status);
            setSessionStatus(flow_session_status);
            // Check if all events are complete, and if so, stop polling
            const allComplete = newEvents.events.every(event => event.event_status === 'COMPLETE');

            if (allComplete) {
                console.log('All events are complete, stopping polling');
                clearInterval(pollingIntervalId);
                setIsPolling(false);
            }

        } catch (error) {
            console.error('Failed to fetch events:', error);
            clearInterval(pollingIntervalId); // Consider whether to stop polling on error
            setIsPolling(false);
        }
    };

    const startPolling = () => {
        if (!isPolling) {
            setIsPolling(true);
            fetchEvents(); // Fetch immediately, then set up interval
            const intervalId = setInterval(fetchEvents, 1000); // Poll every 5000ms (5 seconds)
            setPollingIntervalId(intervalId);
        }
    };

    const startDebuggingSession = async () => {
        try {
            setDebugging(true);

            let session_id = uuidv4();

            console.log("session_id from debug Provider", session_id);

            setSessionId(session_id);

            //start polling 
            startPolling();


            let res = await api.flows.executeFlow(
                flowFrontmatter.flow_id,
                flowFrontmatter.flow_version_id,
                session_id, //session_id
                "DEBUG" //stage
            );

            console.log("res from executeFlow", res);

        } catch (error) {
            console.log("error executingFlow from DebugPanel", error);
        }
    }

    const stopDebuggingSession = () => {
        setDebugging(false);
        setSessionId("");
        setEvents([]);
    }

    // Clean up on component unmount to avoid memory leaks
    useEffect(() => {
        return () => {
            if (pollingIntervalId) {
                clearInterval(pollingIntervalId);
            }
        };
    }, [pollingIntervalId]);

    return (
        <DebuggingContext.Provider
            value={{
                startDebuggingSession,
                stopDebuggingSession,
                session_id,
                events,
                debugging,
                session_status
            }}
        >
            {children}
        </DebuggingContext.Provider>
    );
};
