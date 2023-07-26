import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";
import { useSqlContext } from "../context/SqlProvider";

export default function Tables() {
  const { getTableData } = useSqlContext();
  const [data, setData] = useState<any[]>([]);

  const { table } = useParams();

  const hydrate = async () => {
    try {
      if (!table) return;
      const data = await getTableData(table);
      console.log("data in tableData", data);
      setData(data);
    } catch (error) {
      console.log("error", error);
    }
  };

  useEffect(() => {
    hydrate();
  }, []);
  return (
    <div className="flex flex-col h-full w-full m-10">
      <div className="text-5xl text-white mx-5">{table}</div>
      <div className="text-2xl my-2 mx-5">The Data</div>
      <ul>
        {data.map((dat) => {
          return (
            <li key={dat.id} className="text-2xl my-2 mx-5">
              {JSON.stringify(dat)}
            </li>
          );
        })}
      </ul>
    </div>
  );
}
