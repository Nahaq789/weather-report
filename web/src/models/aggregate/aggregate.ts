import { Humidity } from "./humidity";
import { Temperature } from "./temperature";

export interface Aggregate {
	temperature: Temperature;
	humidity: Humidity;
}
