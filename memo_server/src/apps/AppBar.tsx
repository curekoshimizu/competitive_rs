import { useHistory } from 'react-router-dom';

import AppBar from '@material-ui/core/AppBar';
import Button from '@material-ui/core/Button';
import { createStyles, makeStyles, Theme } from '@material-ui/core/styles';
import Toolbar from '@material-ui/core/Toolbar';
import Typography from '@material-ui/core/Typography';

export interface Links {
  path: string;
  title: string;
}

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      flexGrow: 1,
    },
    menuButton: {
      marginRight: theme.spacing(2),
    },
    title: {
      flexGrow: 1,
      color: 'white',
    },
  })
);

interface AppBarProps {
  links: Array<Links>;
}

export default (({ links }) => {
  const classes = useStyles();
  const history = useHistory();

  return (
    <div className={classes.root}>
      <AppBar position="static">
        <Toolbar>
          {links.map((link) => (
            <Button key={link.path} onClick={() => history.push(link.path)}>
              <Typography className={classes.title} component="h2" variant="h5">
                {link.title}
              </Typography>
            </Button>
          ))}
        </Toolbar>
      </AppBar>
    </div>
  );
}) as React.FC<AppBarProps>;
