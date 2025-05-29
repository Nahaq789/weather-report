import { Sensor } from "@/lib/models/sensor/sensor";
import { ApexOptions } from "apexcharts";
import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

const Chart = dynamic(() => import("react-apexcharts"), {
  ssr: false,
  loading: () => <div>loading...</div>,
});

interface Series {
  name: string;
  data: number[];
}

interface TimeChartProps {
  series: Series[];
  sensors: Sensor[];
}

const timeLine = [
  "現在",
  "5秒",
  "10秒",
  "15秒",
  "20秒",
  "25秒",
  "30秒",
  "35秒",
  "40秒",
  "45秒",
  "50秒",
  "55秒",
  "60秒",
].reverse();
const TimeChart = ({ series, sensors }: TimeChartProps) => {
  const categories = timeLine.slice(timeLine.length - sensors.length, 14);
  const options: ApexOptions = {
    chart: {
      height: "100%",
    },
    stroke: {
      curve: "smooth",
    },
    xaxis: {
      categories: categories,
    },
  };
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  if (!isClient) {
    return <div>loading...</div>;
  }

  return (
    <>
      <div className="w-full h-full">
        <Chart type="line" options={options} series={series}></Chart>
      </div>
    </>
  );
};

export default TimeChart;
