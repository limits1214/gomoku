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
      const obj = {
        t: 'topicSubscribe',
        d: {
          topic: `ROOM#${roomId}`
        }
      };
      sendWsMessage(JSON.stringify(obj));
    }
    return () => {
      if (sendWsMessage) {
        const obj = {
          t: 'topicUnSubscribe',
          d: {
            topic: `ROOM#${roomId}`
          }
        };
        sendWsMessage(JSON.stringify(obj));
      }
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
    if (sendWsMessage) {
      const obj = {
        t: 'roomChat',
        d: {
          msg: chat,
          roomId
        }
      };
      sendWsMessage(JSON.stringify(obj));

      const obj2 = {
        t: 'echo',
        d: {
          msg: 'msg'
        }
      };
      sendWsMessage(JSON.stringify(obj2));
    }
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