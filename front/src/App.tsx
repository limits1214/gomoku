import { BrowserRouter, Route, Routes } from 'react-router-dom'
import DefaultLayout from './layout/DefaultLayout.tsx'
import TestChat from './page/test/TestChat.tsx'
import Test from './page/test/Test.tsx'
import Home from './page/Home.tsx'
import Room from './page/Room.tsx'
import NotFound from './page/NotFound.tsx'
import { useEffect, useMemo, useState } from 'react'
import { useAuthStore, useWsStore } from './store/store.ts'
import useWebSocket, { ReadyState } from 'react-use-websocket'
function App() {

  const { setLastWsMessage,  setSendWsMessage } = useWsStore();
  const { isInitEnd, accessToken } = useAuthStore();
  const [socketUrl, setSocketUrl] = useState<string | null>(null);
  const {sendMessage, lastMessage, readyState} = useWebSocket(socketUrl, {
      // shouldReconnect: (event) => {
      //     console.log('shouldReconnect', event)
      //     return false
      // },
        // reconnectAttempts: 10,
        // reconnectInterval: 1000,
        onOpen: () => {
          console.log('ws open')
            const obj = {
                t: "wsInitial",
                d: {
                  jwt: accessToken
                }
            }
            sendMessage(JSON.stringify(obj));
        },
        onClose: () => {
          console.log('ws close')
        },
    });
  const connectionStatus = {
    [ReadyState.CONNECTING]: 'Connecting',
    [ReadyState.OPEN]: 'Open',
    [ReadyState.CLOSING]: 'Closing',
    [ReadyState.CLOSED]: 'Closed',
    [ReadyState.UNINSTANTIATED]: 'Uninstantiated',
  }[readyState];

  useEffect(() => {
    // setSocketUrl(`wss://echo.websocket.org`);
    if (isInitEnd) {
      setSocketUrl(`wss://0gnlyzkqd6.execute-api.ap-northeast-2.amazonaws.com/dev/`);
    }
  }, [isInitEnd])

  useEffect(() =>{
    console.log('ws state: ', connectionStatus)
    if (connectionStatus == 'Open') {
      setSendWsMessage(sendMessage)
    }
  }, [connectionStatus, sendMessage, setSendWsMessage])

  useEffect(() => {
    if (lastMessage != null) {
      console.log('lastMessage: ', lastMessage.data)
      try {
        const json = JSON.parse(lastMessage.data );
        setLastWsMessage(json)
      } catch(error) {
        console.error(error);
      }
      
    }
  }, [lastMessage, setLastWsMessage])

  // useEffect(() => {
  //   console.log("???")
    
  //   if (sendWsMessage != null) {
  //     sendMessage(JSON.stringify(sendWsMessage))
  //   }
  // }, [sendWsMessage, sendMessage ])
  
  useEffect(() => {
    console.log('start')
    return () => {
      console.log('end')
    }
  },[])
  useInitialAuthCheck();
  return (
    <BrowserRouter basename='/gomoku'>
      <Routes>
        <Route element={<DefaultLayout/>}>
          <Route index element={<Home/>} />
          <Route path='room/:roomSn' element={<Room/>} />
        </Route>
        <Route path='test'>
          <Route index element={<Test/>} />
          <Route path='chat' element={<TestChat/>} />
        </Route>
        <Route path="*" element={<NotFound />} />
      </Routes>
    </BrowserRouter>
  )
}

export default App


const useInitialAuthCheck = () =>{
  const { setIsInitEnd, setAccessToken } = useAuthStore();
  useMemo(() => {
    const refresh = async () => {
      console.log('refersh')
      try {
        const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/access/refresh`, {
          credentials: 'include',
          method: 'POST',
        })
        const json = await res.json();
        if (json.data && json.data.accessToken) {
          const accessToken = json.data.accessToken;
          setAccessToken(accessToken)
        }
      } catch (error) {
        console.error(error)
      }
      setIsInitEnd(true)
    }

    refresh();
  }, [])
}