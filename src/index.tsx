import React from 'react'
import ReactDOM from 'react-dom'
import Main from './layouts/Main'
import './assets/styles/main.css'
import { invoke } from '@tauri-apps/api/tauri'

invoke('setup', { invokeMessage: "meow" })
.then((message) => console.log(message))

ReactDOM.render(
  <React.StrictMode>
    <Main />
  </React.StrictMode>,
  document.getElementById('root')
)
