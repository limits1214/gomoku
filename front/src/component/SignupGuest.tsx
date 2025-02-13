'use client'

import { SignupGuestAction } from '@/actions';
import React, { useState } from 'react'

const SignupGuest = () => {
  const [nickName, setNickName] = useState("");
  const [isVisible, setIsVisible] = useState(true);
  const onChangeNickName = (event: React.ChangeEvent<HTMLInputElement>) => {
    const nickName = event.target.value;
    setNickName(nickName)
  }

  const handleSingup = async () => {
    await SignupGuestAction(nickName);
    setIsVisible(false)
  }
  
  return (
    <>
      {isVisible && <div>
          <input type="text" onChange={onChangeNickName} />
          <button onClick={ handleSingup }>Signup</button>
        </div>
      }
    </>
  )
}

export default SignupGuest