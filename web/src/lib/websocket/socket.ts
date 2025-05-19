import { atom, useAtom } from "jotai";
import { useEffect, useState } from "react";

const socketAtom = atom<WebSocket | null>(null);

export const useWebSocket = (url: string) => {
	const [socket, setSocket] = useAtom(socketAtom);
	const [connected, setConnected] = useState(false);

	useEffect(() => {
		const newSocket = new WebSocket(url);
		newSocket.onopen = () => setConnected(true);
		newSocket.onclose = () => setConnected(false);
		setSocket(newSocket);

		return () => {
			if (socket && socket.readyState === WebSocket.OPEN) {
				socket.close();
			}
		};
	}, [url, setSocket]);

	return { socket, connected };
};
