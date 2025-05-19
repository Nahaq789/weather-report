import { atom, useAtom } from "jotai";
import { useEffect, useState } from "react";

const url = "ws://localhost:5678/hoge";

const socketAtom = atom<WebSocket | null>(null);

export const useSensor = () => {
  const [socket, setSocket] = useAtom(socketAtom);

  useEffect(() => {
    setSocket((socket) => {
      if (socket) return socket;
      return new WebSocket(url);
    });
  }, [setSocket]);

  useEffect(() => {
    if (socket) {
      socket.onmessage = (res) => {
        console.log(res);
      };
    }
  }, [socket]);

  const sendMessage = (some: string) => {
    socket?.send("hoge");
  };

  return {
    sendMessage,
  };
};

export const useWebSocket = (url: string) => {
  const [socket, setSocket] = useAtom(socketAtom);
  const [connected, setConnected] = useState(false);

  useEffect(() => {
    const newSocket = new WebSocket(url);
    newSocket.onopen = () => setConnected(true);
    newSocket.onclose = () => setConnected(false);
    setSocket(newSocket)

    return () => {
      if (socket && socket.readyState === WebSocket.OPEN) {
        socket.close();
      }
    }
  }, [url, setSocket])

  return { socket, connected }
}
