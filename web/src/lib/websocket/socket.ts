import { atom, useAtom } from "jotai";
import { useEffect } from "react";

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
