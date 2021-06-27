import Button from '@material-ui/core/Button';

interface IssueButtonProp {
  contest: string;
  problem: string;
}

const IssueButton: React.FC<IssueButtonProp> = ({ contest, problem }) => {
  return (
    <Button
      color="primary"
      href={`https://atcoder.jp/contests/${contest}/tasks/${contest}_${problem}`}
      variant="contained"
    >
      {problem}
    </Button>
  );
};

export default IssueButton;
