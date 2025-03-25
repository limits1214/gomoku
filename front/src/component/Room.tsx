'use client'
import { Button } from '@/components/ui/button';
import { useWsStore } from '@/store/store';
import Link from 'next/link';
import React, {  useEffect, useRef, useState } from 'react'
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"



interface RoomParam {
  roomId: string,
  roomName: string
}


const Room = ({roomId, roomName}: RoomParam) => {
  
  const {sendWsMessage} = useWsStore();

  useEffect(() => {
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



  return (
    // <Card className=''>
      <CardContent className='flex p-2'>
        <GameArea/>
        <InfoArea roomId={roomId} roomName={roomName}/>
      </CardContent>
    // </Card>
  )
}

export default Room

const GameArea = () => {
  return (
    <Card className='flex-[5]'>
      <CardContent>
        sdf
      </CardContent>
    </Card>
  )
}

interface InfoAreaParam {
  roomId: string,
  roomName: string
}
const InfoArea = ({roomId, roomName}: InfoAreaParam) => {
  const {sendWsMessage, lastWsMessage} = useWsStore();
  const isFirstRender = useRef(true);
  useEffect(() => {
    if (isFirstRender.current) {
        isFirstRender.current = false; // 첫 실행을 무시
        return;
    }
    if (lastWsMessage) {
      console.log('lastMessage z', lastWsMessage)
    }
  }, [lastWsMessage])

  const chatInputOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setChat(event.target.value)
  }

  const handleChatSend = () => {
    if (sendWsMessage) {
      if (chat == '') {
        return;
      }
      
      const obj = {
        t: 'roomChat',
        d: {
          msg: chat,
          roomId
        }
      };
      sendWsMessage(JSON.stringify(obj));
      setChat('')
    }
  }
  const [chat, setChat] = useState("");

  return (
    <Card className='flex-[2]'>
      <CardContent>
        <div>
          <span>RoomName {roomName}</span>
          <Link href={"/"}>
            <Button>home</Button>
          </Link>
        </div>
        <Separator/>
        <div>
          <div className='flex'>
            <div>
              <span>player white</span>
              <span>time 111</span>
            </div>
            <div>
              <span>player black</span>
              <span>time 13</span>
            </div>
          </div>
        </div>
        <Separator/>
        <div>
          <Tabs defaultValue="chat" className="">
            <TabsList>
              <TabsTrigger value="chat">Chat</TabsTrigger>
              <TabsTrigger value="users">Users</TabsTrigger>
              <TabsTrigger value="settings">Settings</TabsTrigger>
            </TabsList>
            <TabsContent value="chat">
              <div>
                <input type="text" onChange={chatInputOnChange} value={chat} />
                <Button onClick={handleChatSend}>send</Button>
              </div>
            </TabsContent>
            <TabsContent value="users">User List</TabsContent>
            <TabsContent value="settings">Settings</TabsContent>
          </Tabs>
        </div>
      </CardContent>
    </Card>
  )
}