import { Sensor } from "@/models/sensor/sensor";
import dynamic from "next/dynamic";
import { useEffect, useState } from "react";

const Chart = dynamic(() => import("react-apexcharts"), {
  ssr: false,
  loading: () => <div>loading...</div>
})

interface TimeChartProps {
  sensors: Sensor[]
}

const TimeChart = ({ sensors }: TimeChartProps) => {
  const series = [{
    name: "temperature",
    data: [23, 24, 25, 13, 47, 23, 89, 21]
  },
  {
    name: "humidity",
    data: [11, 45, 26, 130, 7, 3, 14, 32]
  },
  {
    name: "hoge",
    data: sensors.map(sensor => sensor.aggregate.temperature.avg)
  }
  ];
  const options = {
    chart: {
      height: "100%"
    }
  };
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  if (!isClient) {
    return <div>loading...</div>
  }

  return (
    <>
      <div className="w-full h-full">
        <Chart type="line" options={options} series={series}></Chart>
      </div >
    </>
  )
}

export default TimeChart;
