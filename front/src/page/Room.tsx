
import { useParams } from 'react-router-dom'

const Room = () => {
  const { roomSn } = useParams()
  return (
    <div>Room {roomSn}</div>
  )
}

export default Room