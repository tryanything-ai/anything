import { Button } from "@repo/ui/components/ui/button";
import { useState } from "react";
import { ChevronDown, ChevronRight } from "lucide-react";

interface JsonExplorerProps {
  data: any;
  className?: string;
  onSelect: (path: string) => void;
  parentPath?: string;
}

interface JsonValueProps {
  value: any;
  path: string;
  keyName: string;
  onSelect: (path: string) => void;
  className?: string;
}

function JsonValue({
  value,
  path,
  keyName,
  onSelect,
  className,
}: JsonValueProps) {
  const [isExpanded, setIsExpanded] = useState(true);

  if (value === null) {
    return (
      <div>
        <Button
          variant="ghost"
          className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
          onClick={() => onSelect(path)}
        >
          {keyName}
        </Button>
        <span className="text-gray-400">null</span>
      </div>
    );
  }

  // Try to parse string values as JSON
  if (typeof value === "string") {
    try {
      const parsedValue = JSON.parse(value);
      if (typeof parsedValue === "object") {
        value = parsedValue;
      }
    } catch (e) {
      // Not valid JSON, keep as string
    }
  }

  if (typeof value === "object") {
    return (
      <div className={className}>
        <div className="flex items-center">
          <Button
            variant="ghost"
            size="sm"
            className="p-0 h-6 w-6 mr-1"
            onClick={() => setIsExpanded(!isExpanded)}
          >
            {isExpanded ? <ChevronDown className="h-4 w-4" /> : <ChevronRight className="h-4 w-4" />}
          </Button>
          <Button
            variant="ghost"
            onClick={() => onSelect(path)}
            className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
          >
            {keyName}
          </Button>
          <span className="ml-2 text-gray-400">
            {Array.isArray(value) 
              ? `[${value.length} item${value.length === 1 ? "" : "s"}]`
              : value === null || value === undefined
                ? "{empty}"
                : `{${Object.keys(value).length} value${Object.keys(value).length === 1 ? "" : "s"}}`}
          </span>
        </div>
        {isExpanded && (
          <div className="ml-4">
            {Object.entries(value).map(([k, v]) => {
              const newPath = Array.isArray(value)
                ? `${path}[${k}]`
                : `${path}.${k}`;
              return (
                <div key={k}>
                  <JsonValue
                    value={v}
                    path={newPath}
                    keyName={k}
                    onSelect={onSelect}
                  />
                </div>
              );
            })}
          </div>
        )}
      </div>
    );
  }

  return (
    <div>
      <Button
        variant="ghost"
        className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
        onClick={() => onSelect(path)}
      >
        {keyName}
      </Button>
      <span className="text-gray-400">
        {typeof value === "string" ? `"${value}"` : String(value)}
      </span>
    </div>
  );
}

export function JsonExplorer({
  data,
  onSelect,
  className = "",
  parentPath = "",
}: JsonExplorerProps): JSX.Element {
  console.log("[JSON EXPLORER] Rendering data:", { data, parentPath });

  return (
    <>
      {Object.entries(data).length === 0 ? (
        <div className="text-gray-400">{"No results"}</div>
      ) : (
        Object.entries(data).map(([key, value]) => (
          <div key={key}>
            <JsonValue
              value={value}
              path={`${parentPath}${key}`}
              keyName={key}
              onSelect={onSelect}
              className={className}
            />
          </div>
        ))
      )}
    </>
  );
}
