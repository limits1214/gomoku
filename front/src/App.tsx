import { BrowserRouter, Route, Routes } from 'react-router-dom'
import DefaultLayout from './layout/DefaultLayout.tsx'
import TestChat from './page/test/TestChat.tsx'
import Test from './page/test/Test.tsx'
import Home from './page/Home.tsx'
import Room from './page/Room.tsx'
import NotFound from './page/NotFound.tsx'
import { useEffect, useState } from 'react'
import { useWsStore } from './store/store.ts'
import useWebSocket, { ReadyState } from 'react-use-websocket'
function App() {

  const { setLastWsMessage,  setSendWsMessage } = useWsStore();
  
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
            // const obj = {
            //     auth: {
            //         accessToken 
            //     }
            // }
            // sendMessage(JSON.stringify(obj));
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
    
    setSocketUrl(`wss://0gnlyzkqd6.execute-api.ap-northeast-2.amazonaws.com/dev/?asdfzz=1234`);
  }, [])

  useEffect(() =>{
    // console.log('ws state: ', connectionStatus)
    if (connectionStatus == 'Open') {
      //
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
