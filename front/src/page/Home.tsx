import { useEffect, useState } from "react"
import { Link } from "react-router-dom"
import { useAuthStore } from "../store/store"

const Home = () => {
  return (
    <div>
      <div>channel: 01</div>
      <GuestSignup/>
      <div>
        <button>New Room</button>
        <table >
          <thead>
            <tr>
              <th>No</th>
              <th>Name</th>
              <th>Black</th>
              <th>White</th>
              <th>Status</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>1</td>
              <td>HelloHello Name Name Wow Wow NameNamwef sdf</td>
              <td>BlackName</td>
              <td>WhiteName</td>
              <td> - </td>
            </tr>
          </tbody>
        </table>
      </div>
      <hr className="mt-2 mb-2" />
      <div>
        <div>players</div>
        <table >
            <thead>
              <tr>
                <th>Player</th>
                <th>Room</th>
              </tr>
            </thead>
            <tbody>
              <tr>
                <td>Abfc</td>
                <td> - </td>
              </tr>
              <tr>
                <td>Abfc</td>
                <td> 2 </td>
              </tr>
            </tbody>
          </table>
      </div>
    </div>
  )
}

export default Home

const GuestSignup = () => {
  const {isInitEnd, accessToken, setAccessToken} = useAuthStore();
  const [nickName, setNickName] = useState("");
  const handleSignup= async () => {
    const res = await fetch(`${import.meta.env.VITE_API_URL}/auth/signup/guest`, {
      method: 'POST',
      credentials: 'include',
      headers: {
        "Content-Type": "application/json"
      },
      body: JSON.stringify({
        nickName
      })
    })
    const json = await res.json();
    const accessToken = json.data.accessToken;
    setAccessToken(accessToken)
  }
  const onChangeNickName = (event: React.ChangeEvent<HTMLInputElement>) => {
    const nickName = event.target.value;
    setNickName(nickName)
  }

  const [isVisible, setIsVisible] = useState(false)
  
  useEffect(() => {
    setIsVisible(isInitEnd && accessToken == null)
  }, [isInitEnd, accessToken])
  return (
    <>
      {isVisible && <div>
          <input type="text" onChange={onChangeNickName} />
          <button onClick={ handleSignup }>Signup</button>
        </div>
      }
    </>
  )
}