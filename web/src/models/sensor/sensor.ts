import { Aggregate } from "../aggregate/aggregate";
import { Status } from "../status/status";

export interface Sensor {
	location: string;
	timeStamp: Date;
	aggregate: Aggregate;
	status?: Status;
}
