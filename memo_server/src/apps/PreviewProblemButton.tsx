import { Link } from 'react-router-dom';

import Box from '@material-ui/core/Box';
import Button from '@material-ui/core/Button';

interface PreviewProblemButtonProp {
  contest: string;
  problem: string;
  numProblems: number;
}

const PreviewProblemButton: React.FC<PreviewProblemButtonProp> = ({
  contest,
  problem,
  numProblems,
}) => {
  return (
    <Box display="flex">
      <Box px={2}>
        <Button
          color="primary"
          component={Link}
          to={`/preview/${contest}/${problem}/${numProblems}`}
          variant="contained"
        >
          {problem} 問題
        </Button>
      </Box>
      <Box px={2}>
        <Button
          color="secondary"
          component={Link}
          to={`/answer/${contest}/${problem}/${numProblems}`}
          variant="contained"
        >
          {problem} 解答例
        </Button>
      </Box>
    </Box>
  );
};

export default PreviewProblemButton;
