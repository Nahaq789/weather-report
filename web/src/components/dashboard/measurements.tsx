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
    if (connected) {
      console.log("area: ", area);
      sendMessage(area);
    }
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

  return (
    <>
      <div className="bg-white rounded-lg shadow-md p-4">
        <h2 className="font-semibold text-lg mb-4">
          温度・湿度の推移（過去1時間）
        </h2>
        <div className="h-full bg-gray-100 rounded-md flex items-center justify-center">
          <TimeChart sensors={sensors} />
        </div>
      </div>
    </>
  );
};

export default Measurements;
