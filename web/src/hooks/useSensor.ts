import { useWebSocket } from "@/lib/websocket/socket";
import { useEffect, useState } from "react";

export const useSensor = (area: string) => {
	const { socket, connected } = useWebSocket("ws://localhost:5678/sensor", area);
	const [data, setData] = useState(null);

	useEffect(() => {
		if (!socket) return;

		socket.onmessage = (event) => {
			setData(event.data);
		};
	}, [socket]);

	const sendMessage = (message: string) => {
		if (connected && socket && socket.readyState === WebSocket.OPEN) {
			socket.send(message);
			return true;
		}
		return false;
	};

	return { sendMessage, data, connected };
};
