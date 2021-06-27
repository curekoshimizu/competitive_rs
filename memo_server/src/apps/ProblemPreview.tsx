import Iframe from 'react-iframe';

import ContestRow from './ContestRow';

interface ProblemPreviewProp {
  contest: string;
  problem: string;
  numProblems: number;
}

const ProblemPreview: React.FC<ProblemPreviewProp> = ({
  contest,
  problem,
  numProblems,
}) => {
  return (
    <>
      <ContestRow contest={contest} numProblems={numProblems} />
      <Iframe
        height="1000"
        url={`https://atcoder.jp/contests/${contest}/tasks/${contest}_${problem}`}
        width="100%"
      />
    </>
  );
};

export default ProblemPreview;
