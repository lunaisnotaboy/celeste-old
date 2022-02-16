import { Button } from '../components/Buttons/Button'
import { invoke } from '@tauri-apps/api/tauri'

function db_test() {
  invoke('database_test', { mxc: "mxc://tg43g43g" });
}

export default function Main() {
  return (
    <div className='main'>
      <h1>Hello, World!</h1>

      <Button label='Button' onClick={() => db_test()} primary />
    </div>
  )
}
