import SignupGuest from "@/component/SignupGuest";
import Link from "next/link";


export default async function Home() {
  return (
    <div>
      <SignupGuest/>
      Home
      <button>CreateRoom</button>
      <Link href="/channel/1/room/1">
        <button>Move Room 1</button>
      </Link>
    </div>
  );
}

