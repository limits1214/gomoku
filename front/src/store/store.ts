import { SendMessage } from "react-use-websocket";
import { create } from "zustand";
import { devtools } from "zustand/middleware";

interface WsState {
  lastWsMessage: object,
  setLastWsMessage: (lastMessage: object) => void,
  sendWsMessage: SendMessage,
  setSendWsMessage: (sendMessage: SendMessage) => void,
}


export const useWsStore = create<WsState>()(
  devtools(
    (set)=>({
      lastWsMessage: "",
      setLastWsMessage(lastWsMessage) {
          set({lastWsMessage})
      },
      sendWsMessage: "",
      setSendWsMessage(sendWsMessage) {
        set({sendWsMessage})
      }
    })
  )
)