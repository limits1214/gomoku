'use client'

import  { useEffect, useState } from 'react'
import useWebSocket, { ReadyState } from 'react-use-websocket';
import { useWsStore } from '../store/store';

const Ws = () => {
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
          //     t: "wsInitial",
          //     d: {
          //       jwt: accessToken
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
    // if (isInitEnd) {
    console.log(process.env.NEXT_PUBLIC_WS_URL)
    const wsurl = process.env.NEXT_PUBLIC_WS_URL as string;
      setSocketUrl(wsurl);
    // }
  }, [])

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

  return <></>
}

export default Ws