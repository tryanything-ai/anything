import { useEffect, useState, ChangeEvent, MouseEventHandler } from "react";
import { useFuseSearch } from "../hooks/useFuseSearch";

interface BaseSearchProps<T> {
  data: T[];
  onResultsChange: (results: T[]) => void;
  searchKey: keyof T | (keyof T)[];
  placeholder?: string;
}

const BaseSearch = <T,>({
  data,
  searchKey,
  onResultsChange,
  placeholder = "Search…",
}: BaseSearchProps<T>) => {
  const [value, setValue] = useState("");
  const results = useFuseSearch(data, value, searchKey, {
    // Any other options you'd like to pass
  });

  useEffect(() => {
    // Extract just the items from the Fuse results
    const items = results.map((result) => result.item);
    onResultsChange(items);
  }, [results]);

  return (
    <div className="form-control w-full">
      {/* <div className="input-group w-full max-w-96"> */}
      <input
        type="text"
        placeholder={placeholder}
        value={value}
        className="input input-bordered w-full"
        onChange={(e) => setValue(e.target.value)}
      />
      {/* <button className="btn btn-square" onClick={onClick}>
          <svg
            xmlns="http:www.w3.org/2000/svg"
            className="h-6 w-6"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth="2"
              d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
            />
          </svg>
        </button> */}
    </div>
    // </div>
  );
};

export default BaseSearch;
