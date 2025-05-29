import { useSensor } from "@/hooks/useSensor";
import { Sensor } from "@/lib/models/sensor/sensor";
import { useEffect, useState } from "react";
import TimeChart from "../charts/timeChart";
import dayjs from "dayjs";
import { useAtom } from "jotai";
import { areaAtom } from "@/atoms/areaAtom";

const Measurements = () => {
  const [area] = useAtom(areaAtom);
  const { sendMessage, data, connected } = useSensor(area);
  const [sensors, setSensors] = useState<Sensor[]>([]);

  useEffect(() => {
    let isActive = true;
    if (connected && isActive) {
      console.log("area: ", area);
      sendMessage(area);
    }

    return () => {
      isActive = false;
    };
  }, [connected, sendMessage, area]);

  useEffect(() => {
    setSensors([]);
  }, [area]);

  useEffect(() => {
    if (data) {
      try {
        const parsedData = JSON.parse(data);

        setSensors((prev) => {
          const newSensors = [...prev, parsedData];

          const ten = dayjs().subtract(1, "minute");
          return newSensors.filter((sensor) => {
            const sensorTime = dayjs(sensor.time_stamp);
            return sensorTime.isAfter(ten) || sensorTime.isSame(ten);
          });
        });
      } catch (error) {
        console.error("Failed to parse sensor data", error);
      }
    }
  }, [data]);

  // 温度データのシリーズ
  const temperatureSeries = [
    {
      name: "平均",
      data: sensors.map((sensor) => sensor.aggregate.temperature.avg),
    },
    {
      name: "中央値",
      data: sensors.map((sensor) => sensor.aggregate.temperature.mid),
    },
    {
      name: "最高",
      data: sensors.map((sensor) => sensor.aggregate.temperature.max),
    },
    {
      name: "最低",
      data: sensors.map((sensor) => sensor.aggregate.temperature.min),
    },
  ];

  // 湿度データのシリーズ（湿度データがある場合）
  const humiditySeries = [
    {
      name: "平均",
      data: sensors.map((sensor) => sensor.aggregate.humidity?.avg || 0),
    },
    {
      name: "中央値",
      data: sensors.map((sensor) => sensor.aggregate.humidity?.mid || 0),
    },
    {
      name: "最高",
      data: sensors.map((sensor) => sensor.aggregate.humidity?.max || 0),
    },
    {
      name: "最低",
      data: sensors.map((sensor) => sensor.aggregate.humidity?.min || 0),
    },
  ];

  return (
    <div className="h-full flex flex-col space-y-4">
      {/* チャートコンテナ - レスポンシブレイアウト */}
      <div className="flex-1 flex flex-col xl:flex-row gap-4 min-h-0">
        {/* 温度チャート */}
        <div className="bg-white rounded-lg shadow-md p-4 xl:p-6 flex-1 min-h-0">
          <div className="flex items-center justify-between mb-3">
            <h2 className="font-semibold text-base xl:text-lg text-gray-800">
              温度の推移（過去1分間）
            </h2>
            <div className="flex items-center space-x-2">
              <div className="w-2 h-2 xl:w-3 xl:h-3 rounded-full bg-green-500"></div>
              <span className="text-xs xl:text-sm text-gray-600">
                {connected ? "リアルタイム更新中" : "接続待機中"}
              </span>
            </div>
          </div>
          <div className="h-full bg-gray-50 rounded-md p-2 xl:p-4 min-h-[180px] xl:min-h-[250px] overflow-hidden">
            <div className="w-full h-full">
              <TimeChart series={temperatureSeries} sensors={sensors} />
            </div>
          </div>
        </div>

        {/* 湿度チャート */}
        <div className="bg-white rounded-lg shadow-md p-4 xl:p-6 flex-1 min-h-0">
          <div className="flex items-center justify-between mb-3">
            <h2 className="font-semibold text-base xl:text-lg text-gray-800">
              湿度の推移（過去1分間）
            </h2>
            <div className="text-xs xl:text-sm text-gray-500">
              データポイント数: {sensors.length}
            </div>
          </div>
          <div className="h-full bg-gray-50 rounded-md p-2 xl:p-4 min-h-[180px] xl:min-h-[250px] overflow-hidden">
            <div className="w-full h-full">
              <TimeChart series={humiditySeries} sensors={sensors} />
            </div>
          </div>
        </div>
      </div>

      {/* サマリー情報 */}
      {sensors.length > 0 && (
        <div className="bg-white rounded-lg shadow-md p-4 xl:p-6 flex-shrink-0">
          <h3 className="font-semibold text-base xl:text-lg text-gray-800 mb-3 xl:mb-4">
            現在の状況
          </h3>
          <div className="grid grid-cols-2 xl:grid-cols-4 gap-3 xl:gap-4">
            <div className="bg-red-50 border border-red-200 rounded-lg p-3 xl:p-4">
              <div className="text-xs xl:text-sm text-red-600 font-medium">
                現在の温度
              </div>
              <div className="text-xl xl:text-2xl font-bold text-red-700">
                {sensors[
                  sensors.length - 1
                ]?.aggregate.temperature.avg?.toFixed(1)}
                °C
              </div>
            </div>
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 xl:p-4">
              <div className="text-xs xl:text-sm text-blue-600 font-medium">
                現在の湿度
              </div>
              <div className="text-xl xl:text-2xl font-bold text-blue-700">
                {sensors[sensors.length - 1]?.aggregate.humidity?.avg?.toFixed(
                  1
                )}
                %
              </div>
            </div>
            <div className="bg-orange-50 border border-orange-200 rounded-lg p-3 xl:p-4">
              <div className="text-xs xl:text-sm text-orange-600 font-medium">
                最高温度
              </div>
              <div className="text-xl xl:text-2xl font-bold text-orange-700">
                {Math.max(
                  ...sensors.map((s) => s.aggregate.temperature.max)
                ).toFixed(1)}
                °C
              </div>
            </div>
            <div className="bg-green-50 border border-green-200 rounded-lg p-3 xl:p-4">
              <div className="text-xs xl:text-sm text-green-600 font-medium">
                最低温度
              </div>
              <div className="text-xl xl:text-2xl font-bold text-green-700">
                {Math.min(
                  ...sensors.map((s) => s.aggregate.temperature.min)
                ).toFixed(1)}
                °C
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default Measurements;
