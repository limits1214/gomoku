import { Outlet } from "react-router-dom"


const DefaultLayout = () => {
  return (
    <main>
      <Outlet/>
    </main>
  )
}

export default DefaultLayout