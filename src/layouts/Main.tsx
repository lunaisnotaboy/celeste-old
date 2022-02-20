import { Button } from '../components/Button'
import Avatar from '../components/Avatar'
import React from 'react'

/*
function login() {
  invoke('login', { login: {username: "erin", homeserver:"https://chat.is-cute.ml", password: "egtw", device_id: "celeste" }})
  .then((res) => console.log(res))
  .catch((e) => console.error(e));
}*/

export default function Main() {
  return (
    <div className='main'>
      <h1>Hello, World!</h1>
      <Button label='Button' />
      <Avatar
        size={40}
        name='Luna (she/they)'
        variant='beam'
        colors={['#92A1C6', '#146A7C', '#F0AB3D', '#C271B4', '#C20D90']}
      />
      ;
    </div>
  )
}
