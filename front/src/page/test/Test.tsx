import { Link } from 'react-router-dom'

const Test = () => {
  return (
    <>
      <h3>{import.meta.env.VITE_ENV}</h3>
      <Link to={"/test/chat"}>
        <button>test chat</button>
      </Link>
    </>
  )
}

export default Test