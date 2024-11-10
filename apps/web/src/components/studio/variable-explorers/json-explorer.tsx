import { Button } from "@repo/ui/components/ui/button";

interface JsonExplorerProps {
  data: any;
  className?: string;
  onSelect: (path: string) => void;
  parentPath?: string;
}

export function JsonExplorer({
  data,
  onSelect,
  className = "",
  parentPath = "",
}: JsonExplorerProps): JSX.Element {
  const renderValue = (key: string, value: any, path: string) => {
    if (value === null) {
      return (
        <div>
          <Button
            variant="ghost"
            className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
            onClick={() => onSelect(path)}
          >
            {key}
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
              onClick={() => onSelect(path)}
              className="p-1 m-1 h-auto bg-blue-500 text-blue-100 hover:bg-blue-600 hover:text-blue-50 font-medium"
            >
              {key}
            </Button>
            <span className="ml-2 text-gray-400">
              {Array.isArray(value)
                ? `[${value.length} item${value.length !== 1 ? "s" : ""}]`
                : `{${Object.keys(value).length} value${Object.keys(value).length !== 1 ? "s" : ""}}`}
            </span>
          </div>
          <div className="ml-4">
            {Object.entries(value).map(([k, v]) => {
              const newPath = Array.isArray(value)
                ? `${path}[${k}]`
                : `${path}.${k}`;
              return <div key={k}>{renderValue(k, v, newPath)}</div>;
            })}
          </div>
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
          {key}
        </Button>
        <span className="text-gray-400">
          {typeof value === "string" ? `"${value}"` : String(value)}
        </span>
      </div>
    );
  };

  return (
    <>
      {Object.entries(data).map(([key, value]) => (
        <div key={key}>{renderValue(key, value, `${parentPath}${key}`)}</div>
      ))}
    </>
  );
}
