interface ChannelRoomPageParam {
  params: Promise<{channelId: string; roomId: string}>
}

const ChannelRoomPage = async ({ params }: ChannelRoomPageParam) => {
  const param = await params;
  return (
    <div>
      ChannelRoomPage
      {param.channelId} - {param.roomId}
    </div>
  )
}

export default ChannelRoomPage