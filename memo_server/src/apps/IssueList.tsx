import Box from '@material-ui/core/Box';
import Typography from '@material-ui/core/Typography';

import IssueButton from './IssueButton';

const range = (n: number): number[] => [...Array(n).keys()];

interface ContestRowProp {
  contest: string;
  numProblems: number;
}

const ContestRow: React.FC<ContestRowProp> = ({ contest, numProblems }) => {
  return (
    <Box display="flex" m={1} p={1}>
      <Box width={300}>
        <Typography component="h3" variant="h5">
          {contest}
        </Typography>
      </Box>
      <Box display="flex" m={1} p={1} width="100%">
        {range(numProblems).map((idx) => {
          const code: number = 'a'.charCodeAt(0);
          const problem = String.fromCharCode(code + idx);

          return (
            <Box width={200}>
              <IssueButton contest={contest} problem={problem} />
            </Box>
          );
        })}
      </Box>
    </Box>
  );
};

const IssueList: React.FC = () => {
  return (
    <>
      <h1>At Coder</h1>
      <ContestRow contest="abc088" numProblems={4} />
      <ContestRow contest="abc089" numProblems={4} />
    </>
  );
};

export default IssueList;
