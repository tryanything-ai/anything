import React, {
  useEffect,
  useState,
  memo,
} from "react";
import { DeepKeys, useFuseSearch } from "../hooks/useFuseSearch";

interface BaseSearchProps<T> {
  data: T[];
  onResultsChange: (results: T[]) => void;
  searchKey: DeepKeys<T> | DeepKeys<T>[];
  placeholder?: string;
}

const BaseSearch: React.FC<BaseSearchProps<any>> = ({
  data,
  searchKey,
  onResultsChange,
  placeholder = "Search…",
}) => {
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
      <input
        type="text"
        placeholder={placeholder}
        value={value}
        className="input input-bordered w-full"
        onChange={(e) => setValue(e.target.value)}
      />
    </div>
  );
};

export default memo(BaseSearch);