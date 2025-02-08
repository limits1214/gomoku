
import { useEffect, useState } from 'react';
import { useParams } from 'react-router-dom'
import useWebSocket from 'react-use-websocket';

const Room = () => {
  const { roomSn } = useParams()
  const [socketUrl, setSocketUrl] = useState<string | null>(null);
  const {sendMessage, lastMessage, readyState} = useWebSocket(socketUrl, {
      shouldReconnect: (event) => {
          console.log('shouldReconnect', event)
          return false
      },
      reconnectAttempts: 10,
      reconnectInterval: 1000,
      onOpen: () => {
          console.log('on open')
          // const obj = {
          //     auth: {
          //         accessToken 
          //     }
          // }
          // sendMessage(JSON.stringify(obj));
      },
      
  });
  useEffect(() => {
    const fetchGomokuDetail = async () => {
        // setSocketUrl(`${import.meta.env.VITE_WS_URL}`);
       
    }
    fetchGomokuDetail()
  }, [])
  return (
    <div className='bg-red-100 '>Room {roomSn}</div>
  )
}

export default Room