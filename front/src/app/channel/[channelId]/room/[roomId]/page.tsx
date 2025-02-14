import Room from "@/component/Room";

interface ChannelRoomPageParam {
  params: Promise<{channelId: string; roomId: string}>
}

const ChannelRoomPage = async ({ params }: ChannelRoomPageParam) => {
  const param = await params;
  return (
    <div >
      ChannelRoomPage
      {param.channelId} - {param.roomId}
      <Room channdlId={param.channelId} roomId={param.roomId}/>
    </div>
  )
}

export default ChannelRoomPage