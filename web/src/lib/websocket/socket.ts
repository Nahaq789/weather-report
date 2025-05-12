import { atom, useAtom } from "jotai";
import { useEffect } from "react";

const url = "ws://localhost:5678";

const socketAtom = atom<WebSocket | null>(null);

export const useSensor = () => {
  const [socket, setSocket] = useAtom(socketAtom);

  useEffect(() => {
    setSocket((socket) => {
      if (socket) return socket;
      return new WebSocket(url)
    })
  }, [setSocket]);
}
