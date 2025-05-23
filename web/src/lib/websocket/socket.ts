import { atom, useAtom } from "jotai";
import { useEffect, useState } from "react";

const socketAtom = atom<WebSocket | null>(null);

export const useWebSocket = (url: string, reconnectKey?: string) => {
	const [socket, setSocket] = useAtom(socketAtom);
	const [connected, setConnected] = useState(false);

	useEffect(() => {
		if (socket) {
			socket.close();
			setConnected(false);
		}
		const newSocket = new WebSocket(url);
		newSocket.onopen = () => setConnected(true);
		newSocket.onclose = () => setConnected(false);
		newSocket.onerror = (error) => {
			console.error("WebSocket error: ", error);
			setConnected(false);
		}
		setSocket(newSocket);

		return () => {
			if (socket && socket.readyState === WebSocket.OPEN || socket?.readyState === WebSocket.CONNECTING) {
				socket.close();
			}
		};
	}, [url, reconnectKey, setSocket]);

	return { socket, connected };
};
