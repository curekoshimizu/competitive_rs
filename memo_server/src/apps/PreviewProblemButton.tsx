import { useState } from 'react';
import { Link } from 'react-router-dom';

import Box from '@material-ui/core/Box';
import Button from '@material-ui/core/Button';
import Checkbox, { CheckboxProps } from '@material-ui/core/Checkbox';
import { green, grey } from '@material-ui/core/colors';
import IconButton from '@material-ui/core/IconButton';
import { withStyles } from '@material-ui/core/styles';
import ChevronRightIcon from '@material-ui/icons/ChevronRight';

const GreenCheckbox = withStyles({
  root: {
    color: grey[600],
    '&$checked': {
      color: green[600],
    },
  },
  checked: {},
})((props: CheckboxProps) => <Checkbox color="default" {...props} />);

interface PreviewProblemButtonProp {
  contest: string;
  problem: string;
  solved: boolean;
  keywords: string[];
}

const PreviewProblemButton: React.FC<PreviewProblemButtonProp> = ({
  contest,
  problem,
  solved,
  keywords,
}) => {
  const [showKeywords, setShowKeywords] = useState<boolean>(false);

  return (
    <div>
      <Box display="flex">
        <GreenCheckbox checked={solved} />
        <Box px={2}>
          <Button
            color="primary"
            component={Link}
            to={`/preview/${contest}/${problem}`}
            variant="contained"
          >
            {problem} 問題
          </Button>
        </Box>
        <Box px={2}>
          <Button
            color="secondary"
            component={Link}
            to={`/answer/${contest}/${problem}`}
            variant="contained"
          >
            {problem} 解答例
          </Button>
        </Box>
      </Box>

      {keywords.length !== 0 && (
        <IconButton onClick={() => setShowKeywords(!showKeywords)} size="small">
          <ChevronRightIcon />
        </IconButton>
      )}

      {showKeywords && (
        <ul>
          {keywords.map((keyword) => {
            return <li key={keyword}>{keyword}</li>;
          })}
        </ul>
      )}
    </div>
  );
};

export default PreviewProblemButton;
