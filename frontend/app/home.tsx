'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';

import RefreshIcon from '@mui/icons-material/Refresh';

import InputField from './components/inputField';
import Button from './components/button';
import Status from './components/status';

export default function Home({ baseUrl }: { baseUrl: string }) {
  const [status, setStatus] = useState({ code: 'ok', data: '' });
  const [reload, forceReload] = useState(true);

  const [response, setResponse] = useState({ messages: [ { username: 'server', message: 'loading' } ] });
  const [username, setUsername] = useState('username');
  const [message, setMessage] = useState('');

  const getMessages = async() => {
    setStatus({ code: 'ok', data: '' });

    try {
      const result = await axios.get(baseUrl + 'getMessages');

      if(result.status != 200) {
        setStatus({ code: 'error', data: 'could not get messages: ' + result.statusText });
      } else {
        setResponse(result.data);
      }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (e: any) {
      setStatus({ code: 'error', data: 'could not get messages: ' + e.message });
    }
  }

  useEffect(() => { if(status.code != 'error') { getMessages(); }}, [reload]);

  const sendMessage = async() => {
    setStatus({ code: 'ok', data: '' });

    const sendMessage = { username, message }
    if (message == '') return;

    try {
      const result = await axios.post(baseUrl + 'postMessage', sendMessage);

      if(result.status != 200) {
        setStatus({ code: 'error', data: 'could not send message: ' + result.statusText });
      } else {
        setMessage('');
        forceReload(!reload);
      }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (e: any) {
      setStatus({ code: 'error', data: 'could not send message: ' + e.message });
    }
  }

  return (
    <div className="h-screen max-h-screen bg-white dark:bg-gray-950">
      <div className="flex flex-col">
        <div className="flex pt-2 px-2">
          <div className="w-64 pr-2"><InputField type="text" value={username} setValue={setUsername}/></div>
          <InputField type="text" value={message} setValue={setMessage}/>
          <Button text="send" style="primary" onClick={sendMessage}/>
          <Button text={<RefreshIcon/>} style="secondary" onClick={getMessages}/>
        </div>
        {(status.code !== 'ok') && <div className="px-2"><Status status={status}/></div>}
      </div>
        <div className="h-0 border-2 border-transparent border-t-gray-500"></div>
        <div className="h-[93vh] overflow-auto p-2">
          {
          response.messages.map((message, index) => {
            return (
              <div className="flex align-center h-10 bg-sky-500/5 dark:bg-gray-500/5 border border-transparent border-b-sky-200 dark:border-b-gray-500" key={'message-' + index}>
                {<a className="p-2 text-lg font-bold">{message.username}:</a>}
                {<a className="p-2 text-lg">{message.message}</a>}
                </div>
            );
          })
        }
        </div>
      </div>
  );
}
