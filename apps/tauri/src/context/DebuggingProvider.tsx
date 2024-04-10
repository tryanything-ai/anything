import {
    createContext,
    ReactNode,
    useContext,
    useState,
    useEffect,
    useRef
} from "react";
import { v4 as uuidv4 } from "uuid";
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
    // session_id: string;
    events: any[];
    debugging: boolean;
    session_status: SessionStatus | null;
}

export const DebuggingContext = createContext<DebuggingContextInterface>({
    startDebuggingSession: () => { },
    stopDebuggingSession: () => { },
    // session_id: "",
    events: [],
    debugging: false,
    session_status: null
});

export const useDebuggingContext = () => useContext(DebuggingContext);


export const DebuggingProvider = ({ children }: { children: ReactNode }) => {
    const { flowFrontmatter } = useFlowContext();
    const [debugging, setDebugging] = useState(false);
    const [events, setEvents] = useState<any[]>([]);
    const [session_status, setSessionStatus] = useState<SessionStatus | null>(null);

    //polling state
    const [isPolling, setIsPolling] = useState(false);
    const timerIdRef = useRef(null);

    const fetchEvents = async (session_id: string) => {
        try {
            console.log("fetching events for session_id: ", session_id);

            const fetchedEvents: any = await api.flows.fetchSessionEvents(session_id);

            setEvents(fetchedEvents.events);

            console.log("fetchedEvents: ", fetchedEvents);

            if (fetchedEvents.events.length !== 0) {

                const flow_session_status = fetchedEvents.events[0].flow_session_status;
                console.log("flow_session_status: ", flow_session_status);

                setSessionStatus(flow_session_status);
                // Check if all events are complete, and if so, stop polling
                const allComplete = fetchedEvents.events.every(event => event.event_status === 'COMPLETE');

                if (allComplete) {
                    console.log('All events are complete, stopping polling');
                    // clearInterval(pollingIntervalId);
                    clearInterval(timerIdRef.current);
                    setIsPolling(false);
                }
            }

        } catch (error) {
            console.error('Failed to fetch events:', error);
            // clearInterval(pollingIntervalId); // Consider whether to stop polling on error
            clearInterval(timerIdRef.current);
            setIsPolling(false);
        }
    };

    const startPolling = (session_id: string) => {
        if (!isPolling) {
            setIsPolling(true);
            fetchEvents(session_id); // Fetch immediately, then set up interval
            timerIdRef.current = setInterval(() => fetchEvents(session_id), 1000);
        }
    };

    const startDebuggingSession = async () => {
        try {
            console.log("Starting Debugging Session");
            setDebugging(true);

            let new_session_id = uuidv4();

            console.log("session_id from debug Provider", new_session_id);

            //start polling 
            startPolling(new_session_id);

            let res = await api.flows.executeFlow(
                flowFrontmatter.flow_id,
                flowFrontmatter.flow_version_id,
                new_session_id, //session_id
                "DEBUG" //stage
            );

            console.log("session_id from executeFlow: ", res);

            // setSessionId(res);

        } catch (error) {
            console.log("error executingFlow from DebugPanel", error);
        }
    }

    const stopDebuggingSession = () => {
        setDebugging(false);
        // setSessionId("");
        setEvents([]);
    }

    // Clean up on component unmount to avoid memory leaks
    useEffect(() => {
        return () => {

            clearInterval(timerIdRef.current);

        };
    }, []);

    return (
        <DebuggingContext.Provider
            value={{
                startDebuggingSession,
                stopDebuggingSession,
                // session_id,
                events,
                debugging,
                session_status
            }}
        >
            {children}
        </DebuggingContext.Provider>
    );
};
