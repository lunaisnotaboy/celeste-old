import React from 'react'
import ReactDOM from 'react-dom'
import Main from './layouts/Main'
import './assets/styles/main.css'
import { invoke } from '@tauri-apps/api/tauri'

invoke('setup', { invokeMessage: "meow" })
.then((res) => console.log(res))
.catch((e) => console.error(e))

invoke('cmd_b')
.then((message) => alert(message))

ReactDOM.render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>,
  document.getElementById('root')
)
