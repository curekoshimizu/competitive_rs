import { useState } from 'react';

import Box from '@material-ui/core/Box';
import Typography from '@material-ui/core/Typography';
import useAsyncEffect from 'use-async-effect';

import PreviewProblemButton from './PreviewProblemButton';

interface ContestInfo {
  name: string;
  solved: boolean;
  keywords: string[];
}

interface ContestRowProp {
  contest: string;
}

const ContestRow: React.FC<ContestRowProp> = ({ contest }) => {
  const [contestInfo, setContestInfo] = useState<ContestInfo[]>([]);
  useAsyncEffect(async () => {
    const data = await fetch(`/api/v1/atcoder/${contest}`);
    const ret = (await data.json()) as ContestInfo[];

    ret.sort((a, b) => {
      return a.name.charCodeAt(0) - b.name.charCodeAt(0);
    });
    setContestInfo(ret);
  }, []);

  return (
    <Box display="flex" justifyContent="center" m={1} p={1}>
      <Box width={300}>
        <Typography component="h3" variant="h5">
          {contest.replace('abc', 'abc ').toUpperCase()}
        </Typography>
      </Box>
      <Box display="flex" width="100%">
        {contestInfo.map(({ name, solved, keywords }) => {
          return (
            <Box key={name} width={300}>
              <PreviewProblemButton
                contest={contest}
                keywords={keywords}
                problem={name}
                solved={solved}
              />
            </Box>
          );
        })}
      </Box>
    </Box>
  );
};

export default ContestRow;
