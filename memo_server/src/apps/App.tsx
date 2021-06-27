import { BrowserRouter, Route, Switch, useParams } from 'react-router-dom';

import { createMuiTheme } from '@material-ui/core';
import CssBaseline from '@material-ui/core/CssBaseline';
import {
  ThemeProvider as MaterialThemeProvider,
  StylesProvider,
} from '@material-ui/styles';
import { ThemeProvider as StyledThemeProvider } from 'styled-components';

import AppBar, { Links } from './AppBar';
import IssueList from './IssueList';
import { ProblemAnsPreview, ProblemPreview } from './ProblemPreview';

interface ProblemPreviewPageProp {
  contest: string;
  problem: string;
  numProblems: string;
}

const ProblemPreviewPage = () => {
  const { contest, problem, numProblems } = useParams<ProblemPreviewPageProp>();

  if (!(contest && problem && numProblems)) {
    return <></>;
  }

  return (
    <ProblemPreview
      contest={contest}
      numProblems={Number(numProblems)}
      problem={problem}
    />
  );
};

const ProblemAnsPage = () => {
  const { contest, problem, numProblems } = useParams<ProblemPreviewPageProp>();

  if (!(contest && problem && numProblems)) {
    return <></>;
  }

  return (
    <ProblemAnsPreview
      contest={contest}
      numProblems={Number(numProblems)}
      problem={problem}
    />
  );
};

const theme = createMuiTheme({
  palette: {
    primary: {
      main: '#1255a3',
    },
    secondary: {
      main: '#95b8e3',
    },
  },
});

const links: Links[] = [{ path: '/', title: 'Competitive Programming' }];

const App: React.FC = () => (
  <BrowserRouter>
    <StylesProvider injectFirst>
      <MaterialThemeProvider theme={theme}>
        <StyledThemeProvider theme={theme}>
          <CssBaseline />
          <main>
            <AppBar links={links} />
            <Switch>
              <Route component={IssueList} exact path="/" />
              <Route
                component={ProblemPreviewPage}
                exact
                path="/preview/:contest/:problem/:numProblems"
              />
              <Route
                component={ProblemAnsPage}
                exact
                path="/answer/:contest/:problem/:numProblems"
              />
            </Switch>
          </main>
        </StyledThemeProvider>
      </MaterialThemeProvider>
    </StylesProvider>
  </BrowserRouter>
);

export default App;
