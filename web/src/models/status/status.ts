import { Sensor } from "../sensor/sensor";

export interface Status {
	errorCount: number;
	lastErrorTime: Date;
	errors: Sensor[];
}
