import SignupGuest from "@/component/SignupGuest";
import Link from "next/link";

interface RoomInfo {
  channel: string,
  roomId: string,
  roomName: string,
  roomNum: string,
}

export default async function Home() {
  const channel = "1";
  const params = new URLSearchParams({
    channel
  });
  const queryString = params.toString();
  const res = await fetch(`${process.env.API_URL}/room/list?${queryString}`);
  const j = await res.json();
  const roomList = j.data.list as RoomInfo[];
  // todo pagination with more btn
  const paginationKey = j.data.paginationKey as string | null
  return (
    <div>
      <SignupGuest/>
      Home
      <button>CreateRoom</button>
      <div className="flex flex-col">
        {roomList.map((roomInfo) => 
          <div key={roomInfo.roomId} className="border-2 m-1 border-black">
            <Link  href={`/channel/${roomInfo.channel}/room/${roomInfo.roomNum}`}>
              <button>{roomInfo.roomNum}/{roomInfo.roomName}</button>
            </Link>
          </div>
        )}
      </div>
      <button>More</button>
    </div>
  );
}

