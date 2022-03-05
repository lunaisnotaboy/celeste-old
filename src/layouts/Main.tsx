import { Button } from '../components/Button'
import { Test } from '../components/Test'
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
      <Test />
    </div>
  )
}
