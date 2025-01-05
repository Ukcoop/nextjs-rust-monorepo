'use client';

import { useEffect, useState } from 'react';
import axios from 'axios';

export default function Home({ baseUrl }: { baseUrl: string }) {
  const [response, setResponse] = useState({message: ''});

  useEffect(() => {
    const getData = async () => {
      const res = await axios.get(baseUrl);
      setResponse(res.data);
    };

    getData();
  });

  return (
    <div>
      message from server: {response.message}
    </div>
  );
}
