import { Link } from 'react-router-dom'

const Test = () => {
  return (
    <>
      <Link to={"/test/chat"}>
        <button>test chat</button>
      </Link>
    </>
  )
}

export default Test