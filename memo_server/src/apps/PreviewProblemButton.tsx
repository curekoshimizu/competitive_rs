import { Link } from 'react-router-dom';

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
    <>
      <Button
        color="primary"
        component={Link}
        to={`/preview/${contest}/${problem}/${numProblems}`}
        variant="contained"
      >
        {problem}
      </Button>
    </>
  );
};

export default PreviewProblemButton;
