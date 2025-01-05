import Home from './home';

export default function Main() {
  const baseUrl = process.env.NODE_ENV == 'production' ? '/api/' : 'http://127.0.0.1:3080/api/';

  return (<Home baseUrl={baseUrl}/>);
}
