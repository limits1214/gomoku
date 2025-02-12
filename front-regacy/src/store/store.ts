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

interface AuthState {
  accessToken: string | null,
  isInitEnd: boolean,
  setAccessToken: (token: string|null) => void,
  clearAccessToken: () => void,
  setIsInitEnd: (isInitEnd: boolean) => void,
}

export const useAuthStore = create<AuthState>()(
  devtools(
      (set) => ({
          accessToken: null,
          isInitEnd: false,
          setAccessToken: (token) => set({accessToken: token}),
          clearAccessToken: () => set({accessToken: null}),
          setIsInitEnd: (isInitEnd) => set({isInitEnd}),
      })
  )
)