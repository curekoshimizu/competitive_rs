import ContestRow from './ContestRow';

const IssueList: React.FC = () => {
  return (
    <>
      <h1>At Coder</h1>
      <ContestRow contest="abc087" />
      <ContestRow contest="abc088" />
    </>
  );
};

export default IssueList;
