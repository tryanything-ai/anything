import React, {
  createContext,
  useState,
  useEffect,
  useContext,
  ReactNode,
} from "react";

interface EventLoopContextInterface {
  theme: string;
  setTheme: (theme: string) => void;
}
// TODO: This should be in Rust i beleive but I don't have time for that right now
export const EventLoopContext = createContext<EventLoopContextInterface>({
  theme: localStorage.getItem("theme") || "dark",
  setTheme: () => {},
});

export const useEventLoopContext = () => useContext(EventLoopContext);

export const EventLoopProvider = ({ children }: { children: ReactNode }) => {
  const [theme, setTheme] = useState(localStorage.getItem("theme") || "dark");

  useEffect(() => {
    document.body.setAttribute("data-theme", theme);
    localStorage.setItem("theme", theme);
  }, [theme]);

  return (
    <EventLoopContext.Provider value={{ theme, setTheme }}>
      {children}
    </EventLoopContext.Provider>
  );
};
