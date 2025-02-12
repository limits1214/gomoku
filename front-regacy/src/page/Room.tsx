
import { useParams } from 'react-router-dom'
import { useWsStore } from '../store/store';
import { useEffect } from 'react';

const Room = () => {
  const { roomSn } = useParams()
  const {sendWsMessage, lastWsMessage} = useWsStore();
 
  const handleMsg = () => {
    sendWsMessage(JSON.stringify({msg: 'hi'}));
  }

  useEffect(() => {
    // if ('msg' in lastWsMessage) {
      console.log('1', lastWsMessage)
    // }
  }, [lastWsMessage])
  return (
    <div >
      <span>Room {roomSn}</span>
      <button onClick={handleMsg}>msg</button>
      <Subbb/>
    </div>
  )
}

export default Room

const Subbb = () => {
  const {sendWsMessage, lastWsMessage} = useWsStore();
  useEffect(() => {
    // const k = setInterval(() => {
    //  sendWsMessage(JSON.stringify({'zzz':' zzzz'}))
    // }, 2000)
    // return () => {
    //   clearInterval(k)
    // }
  }, [sendWsMessage])

  useEffect(() => {
    
    // if ('zzz' in lastWsMessage) {
      console.log('2', lastWsMessage)
    // }
  }, [lastWsMessage])
  return (
    <>
    subbb
    </>
  )
}