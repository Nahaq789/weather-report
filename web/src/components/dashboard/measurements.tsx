import { useSensor } from "@/hooks/useSensor";
import { Sensor } from "@/models/sensor/sensor";
import { useEffect, useState } from "react";
import TimeChart from "../charts/timeChart";
import dayjs from "dayjs";

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
			try {
				const parsedData = JSON.parse(data);
				setSensors(prev => [...prev, parsedData]);
			} catch (error) {
				console.error('Failed to parse sensor data', error);
			}
		}
	}, [data]);

	const recentSensors = sensors.filter(sensor => {
		const sensorTime = dayjs(sensor.timeStamp);
		const oneMinuteAgo = dayjs().subtract(10, 'second');
		return sensorTime.isAfter(oneMinuteAgo);
	});

	console.log(recentSensors);

	return (
		<>
			<div className="bg-white rounded-lg shadow-md p-4">
				<h2 className="font-semibold text-lg mb-4">
					温度・湿度の推移（過去1時間）
				</h2>
				<div className="h-full bg-gray-100 rounded-md flex items-center justify-center">
					<TimeChart sensors={recentSensors} />
				</div>
			</div>
		</>
	);
};

export default Measurements;
