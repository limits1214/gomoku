'use client'
import { useWsStore } from '@/store/store';
import React, { use, useEffect, useState } from 'react'


const Room = ({channelId, roomId}: {channdlId: string, roomId: string}) => {
  const {sendWsMessage, lastWsMessage} = useWsStore();

  useEffect(() => {
    // fetch room info
  }, [])

  useEffect(() => {
    if (sendWsMessage) {
      const obj = {
        t: 'echo',
        d: {
          msg: 'msg'
        }
      };
      sendWsMessage(JSON.stringify(obj));
    }
  }, [sendWsMessage])

  useEffect(() => {
    if (lastWsMessage) {
      console.log('lastMessage', lastWsMessage)
    }
  }, [lastWsMessage])

  const chatInputOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setChat(event.target.value)
  }

  const handleChatSend = () => {
    console.log(chat)
  }
  const [chat, setChat] = useState("");

  

  return (
    <div className=''>
      <div className=''>

      </div>
      <div className='border border-black'>
        <input type="text" onChange={chatInputOnChange} />
        <button onClick={handleChatSend}>send</button>
      </div>
    </div>
  )
}

export default Room