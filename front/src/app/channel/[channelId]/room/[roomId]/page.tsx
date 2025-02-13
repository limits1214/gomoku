const ChannelRoomPage = async ({params}: {params: {channelId: string; roomId: string}}) => {
  const param = await params;
  return (
    <div>
      ChannelRoomPage
      {param.channelId} - {param.roomId}
    </div>
  )
}

export default ChannelRoomPage