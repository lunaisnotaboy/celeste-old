import React from 'react'
import ReactDOM from 'react-dom'
import Main from './layouts/Main'
import './assets/styles/main.css'
import { invoke } from '@tauri-apps/api/tauri'

invoke('cmd_test', { invokeMessage: "meow" })
.then((res) => console.log(res))
.catch((e) => console.error(e))

ReactDOM.render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>,
  document.getElementById('root')
)
