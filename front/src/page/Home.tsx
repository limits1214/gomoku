import { useEffect } from "react"
import { Link } from "react-router-dom"

const Home = () => {
  return (
    <div>
      <div>channel: 01</div>
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