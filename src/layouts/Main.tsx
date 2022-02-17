import { Button } from '../components/Button'
import { invoke } from '@tauri-apps/api/tauri'
import React from 'react'

function db_test() {
  invoke('database_test', { mxc: 'mxc://tg43g43g' })
    .then((res) => console.log(res))
    .catch((e) => console.error(e))
}

export default function Main() {
  return (
    <div className='main'>
      <h1>Hello, World!</h1>

      <Button label='Button' />
      <Button label='Database Test' onClick={() => db_test()} />
    </div>
  )
}
