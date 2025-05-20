import dynamic from "next/dynamic";
import { useEffect, useState } from "react";


const series = [{
  data: [23, 24, 25, 13, 47, 23, 89, 21]
}];

const Chart = dynamic(() => import("react-apexcharts"), {
  ssr: false,
  loading: () => <div>loading...</div>
})

const TimeChart = () => {
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
