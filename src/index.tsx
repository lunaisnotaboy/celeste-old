import { invoke } from '@tauri-apps/api/tauri'
import Main from './layouts/Main'
import ReactDOM from 'react-dom'
import React from 'react'
import './assets/styles/main.css'

invoke('cmd_test', { invokeMessage: 'meow' })
  .then((res) => console.log(res))
  .catch((e) => console.error(e))

ReactDOM.render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>,
  document.getElementById('root')
)
