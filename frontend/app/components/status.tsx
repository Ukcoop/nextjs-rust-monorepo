import HourglassEmptyIcon from '@mui/icons-material/HourglassEmpty';
import DoneIcon from '@mui/icons-material/Done';
import CloseIcon from '@mui/icons-material/Close';

const base = 'flex items-center w-full min-h-10 rounded-md mb-2 p-2';

interface StatusParams {
  status: {
    code: string;
    data: string;
  };
}

export default function Status({ status }: StatusParams) {
  const codes = {
    loading: (
      <div
        className={`${base} bg-blue-500/30 border-2 border-blue-500 text-blue-600 dark:text-blue-400`}
      >
        <HourglassEmptyIcon />
        please wait
      </div>
    ),
    success: (
      <div
        className={`${base} bg-green-500/30 border-2 border-green-500 text-green-600 dark:text-green-400`}
      >
        <DoneIcon />
        {status.data}
      </div>
    ),
    error: (
      <div
        className={`${base} bg-red-500/30 border-2 border-red-500 text-red-600 dark:text-red-400`}
      >
        <CloseIcon />
        {status.data}
      </div>
    ),
  };

  return codes[status.code as keyof typeof codes];
}

