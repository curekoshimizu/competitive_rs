import { useState } from 'react';

import useAsyncEffect from 'use-async-effect';

import ContestRow from './ContestRow';

interface ContestList {
  contests: string[];
}

const ProblemList: React.FC = () => {
  const [contestList, setContestList] = useState<ContestList>({ contests: [] });
  useAsyncEffect(async () => {
    const data = await fetch(`/api/v1/atcoder`);
    const ret = (await data.json()) as ContestList;
    setContestList(ret);
  }, []);

  return (
    <>
      {contestList.contests.map((contest) => {
        return <ContestRow contest={contest} key={contest} />;
      })}
    </>
  );
};

export default ProblemList;
