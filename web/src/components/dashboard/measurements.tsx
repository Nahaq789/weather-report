import { useSensor } from "@/hooks/useSensor";
import { Sensor } from "@/models/sensor/sensor";
import { useEffect, useState } from "react";
import TimeChart from "../charts/timeChart";

const Measurements = () => {
	const { sendMessage, data, connected } = useSensor();
	const [sensors, setSensors] = useState<Sensor[]>([]);

	useEffect(() => {
		if (connected) {
			sendMessage("hoge");
		}
	}, [connected, sendMessage]);

	useEffect(() => {
		if (data) {
			setSensors(prev => [...prev, data])
		}
	}, [data]);

	console.log(sensors);

	return (
		<>
			<div className="bg-white rounded-lg shadow-md p-4">
				<h2 className="font-semibold text-lg mb-4">
					温度・湿度の推移（過去1時間）
				</h2>
				<div className="h-full bg-gray-100 rounded-md flex items-center justify-center">
					<TimeChart />
				</div>
			</div>
		</>
	);
};

export default Measurements;
