import Room from "@/component/Room";
import { notFound } from "next/navigation";

interface ChannelRoomPageParam {
  params: Promise<{ channel: string; roomNum: string }>
}

const ChannelRoomPage = async ({ params }: ChannelRoomPageParam) => {
  const {channel, roomNum} = await params;

  const urlSearchParams = new URLSearchParams({
    channel, roomNum
  });
  const qs = urlSearchParams.toString();
  const res = await fetch(`${process.env.API_URL}/room/info/channelroom?${qs}`, {
    method: 'GET',
    credentials: 'include',
  });
  const j = await res.json();
  const data = j.data;
  if (!data) {
    return notFound()
  }
  return (
    <div >
      ChannelRoomPage
      {channel} - {roomNum}
      <Room roomId={data.roomId} roomName={data.roomName} />
    </div>
  )
}

export default ChannelRoomPage