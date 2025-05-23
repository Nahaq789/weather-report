import { atom, useAtom } from "jotai";
import React from "react";
import Area from "../lib/models/area/area";
import { areaAtom } from "@/atoms/areaAtom";

const Sidebar = () => {
  const [area, setArea] = useAtom(areaAtom);

  const areaLabels: Record<Area, string> = {
    [Area.Tokyo]: "東京",
    [Area.Osaka]: "大阪",
    [Area.Sapporo]: "札幌",
    [Area.Fukuoka]: "福岡",
    [Area.Okinawa]: "沖縄",
  };

  const areas: Array<{ id: Area; label: string }> = [
    { id: Area.Tokyo, label: areaLabels[Area.Tokyo] },
    { id: Area.Osaka, label: areaLabels[Area.Osaka] },
    { id: Area.Sapporo, label: areaLabels[Area.Sapporo] },
    { id: Area.Fukuoka, label: areaLabels[Area.Fukuoka] },
    { id: Area.Okinawa, label: areaLabels[Area.Okinawa] },
  ];

  return (
    <>
      <div className="col-span-2 bg-white rounded-lg shadow-md p-4">
        <h2 className="font-semibold text-lg mb-4 border-b pb-2">観測地点</h2>
        <ul className="space-y-2">
          {areas.map((areaItem) => (
            <li
              key={areaItem.id}
              className={`px-3 py-2 rounded-md cursor-pointer ${
                area === areaItem.id
                  ? "bg-blue-100 font-medium"
                  : "hover:bg-gray-100"
              }`}
              onClick={() => setArea(areaItem.id)}
            >
              {areaItem.label}
            </li>
          ))}
        </ul>

        <h2 className="font-semibold text-lg mt-6 mb-4 border-b pb-2">設定</h2>
        <div className="space-y-4">
          <div>
            <label className="block text-sm mb-1">更新間隔</label>
            <select className="w-full p-2 border rounded-md bg-gray-50">
              <option>5秒</option>
              <option>10秒</option>
              <option>30秒</option>
              <option>1分</option>
            </select>
          </div>
        </div>
      </div>
    </>
  );
};

export default Sidebar;
