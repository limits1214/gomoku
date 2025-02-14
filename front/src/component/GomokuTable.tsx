'use client'

import { Canvas } from '@react-three/fiber'
import React from 'react'

const GomokuTable = () => {
  return (
    <Canvas>
      <mesh>
        <boxGeometry/>
      </mesh>
    </Canvas>
  )
}

export default GomokuTable