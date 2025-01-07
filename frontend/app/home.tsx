'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Home({ baseUrl }: { baseUrl: string }) {
  const [response, setResponse] = useState({messages: [ { message: 'loading' } ]});

  useEffect(() => {
    const getData = async () => {
      const res = await axios.get(baseUrl + 'getMessages');
      setResponse(res.data);
    };

    getData();
  }, [baseUrl]);

  return (
    <div>
      messages from server: {response.messages.length}
    </div>
  );
}
