const styles = {
    primary: 'flex items-center justify-center w-20 h-10 ml-2 bg-black dark:bg-gray-300 hover:bg-gray-900 hover:dark:bg-white text-white dark:text-black rounded-md p-2 px-4',
    secondary: 'flex items-center justify-center w-20 h-10 ml-2 bg-transparent border-2 border-black dark:border-gray-500 hover:bg-gray-200 hover:dark:bg-gray-900 text-black dark:text-white rounded-md p-2 px-4'
};

interface ButtonParams {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  text: any;
  style: keyof typeof styles;
  onClick: () => unknown;
}

export default function Button({ text, style, onClick }: ButtonParams) {
  return (
    <div className={styles[style]} onClick={onClick}>
      <a>{text}</a>
    </div>
  );
}

