'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';

import InputField from './components/inputField';
import Button from './components/button';

export default function Home({ baseUrl }: { baseUrl: string }) {
  const [response, setResponse] = useState({messages: [ { message: 'loading' } ]});
  const [message, setMessage] = useState('');

  const getMessages = async() => {
    const res = await axios.get(baseUrl + 'getMessages');
    setResponse(res.data);
  }

  useEffect(() => { getMessages(); });

  const sendMessage = async() => {
    const sendMessage = { message }

    const result = await axios.post(baseUrl + 'postMessage', sendMessage);
    if(result.status != 200) {
      // do error thing
    }

    setMessage('');
    getMessages();
  }

  return (
    <div className="h-screen max-h-screen bg-white dark:bg-gray-950">
      <div className="flex flex-col">
        <div className="flex pt-2 px-2">
          <InputField type="text" value={message} setValue={setMessage}/>
          <Button text="send" style="primary" onClick={sendMessage}/>
          <Button text="reload" style="secondary" onClick={getMessages}/>
        </div>
      </div>
        <div className="h-0 border-2 border-transparent border-t-gray-500"></div>
        <div className="h-[93vh] overflow-auto p-2">
          {
          response.messages.map((message, index) => {
            return (
              <div className="flex align-center h-10 bg-sky-500/5 dark:bg-gray-500/5 border border-transparent border-b-sky-200 dark:border-b-gray-500" key={'message-' + index}>
                {<a className="p-2 text-lg">{message.message}</a>}
                </div>
            );
          })
        }
        </div>
      </div>
  );
}
