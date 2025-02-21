'use client'
import { useWsStore } from '@/store/store';
import Link from 'next/link';
import React, {  useEffect, useState } from 'react'

interface RoomParam {
  roomId: string,
  roomName: string
}


const Room = ({roomId, roomName}: RoomParam) => {
  const {sendWsMessage, lastWsMessage} = useWsStore();

  useEffect(() => {
    // fetch room info
    // const a = async () => {
    //   console.log('channel', channel)
    //   const params = new URLSearchParams({
    //     channel,
    //     roomNum
    //   });
    //   const queryString = params.toString();
    //   const res = await fetch(`/api/channelroom?${queryString}`)
    //   const j = await res.json();
    //   console.log('j', j)
    // }
    // a();
    if (sendWsMessage) {
      console.log('topic subscribe', roomId, roomName)
    }

    return () => {
      console.log('todo topic unsubscribe')
    }
  }, [roomId, roomName, sendWsMessage])

  useEffect(() => {
    if (sendWsMessage) {
      // const obj = {
      //   t: 'echo',
      //   d: {
      //     msg: 'msg'
      //   }
      // };
      // sendWsMessage(JSON.stringify(obj));
    }
  }, [sendWsMessage])

  useEffect(() => {
    if (lastWsMessage) {
      console.log('lastMessage z', lastWsMessage)
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
      <Link href={"/"}>home</Link>
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