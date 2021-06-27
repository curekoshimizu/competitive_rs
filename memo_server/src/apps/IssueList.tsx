import ContestRow from './ContestRow';

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
