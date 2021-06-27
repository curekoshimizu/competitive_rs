import Box from '@material-ui/core/Box';
import Typography from '@material-ui/core/Typography';

import PreviewProblemButton from './PreviewProblemButton';

const range = (n: number): number[] => [...Array(n).keys()];

interface ContestRowProp {
  contest: string;
  numProblems: number;
}

const ContestRow: React.FC<ContestRowProp> = ({ contest, numProblems }) => {
  return (
    <Box display="flex" justifyContent="center" m={1} p={1}>
      <Box width={300}>
        <Typography component="h3" variant="h5">
          {contest.replace('abc', 'abc ').toUpperCase()}
        </Typography>
      </Box>
      <Box display="flex" width="100%">
        {range(numProblems).map((idx) => {
          const code: number = 'a'.charCodeAt(0);
          const problem = String.fromCharCode(code + idx);

          return (
            <Box key={idx} width={300}>
              <PreviewProblemButton
                contest={contest}
                numProblems={numProblems}
                problem={problem}
              />
            </Box>
          );
        })}
      </Box>
    </Box>
  );
};

export default ContestRow;
