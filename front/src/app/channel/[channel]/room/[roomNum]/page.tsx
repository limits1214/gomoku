import Room from "@/component/Room";

interface ChannelRoomPageParam {
  params: Promise<{ channel: string; roomNum: string }>
}

const ChannelRoomPage = async ({ params }: ChannelRoomPageParam) => {
  const param = await params;
  return (
    <div >
      ChannelRoomPage
      {param.channel} - {param.roomNum}
      <Room channel={param.channel} roomNum={param.roomNum}/>
    </div>
  )
}

export default ChannelRoomPage