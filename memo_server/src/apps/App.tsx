import { createMuiTheme } from '@material-ui/core';
import CssBaseline from '@material-ui/core/CssBaseline';
import {
  ThemeProvider as MaterialThemeProvider,
  StylesProvider,
} from '@material-ui/styles';
import { ThemeProvider as StyledThemeProvider } from 'styled-components';

import IssueList from './IssueList';

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

const App: React.FC = () => (
  <StylesProvider injectFirst>
    <MaterialThemeProvider theme={theme}>
      <StyledThemeProvider theme={theme}>
        <CssBaseline />
        <main>
          <IssueList />
        </main>
      </StyledThemeProvider>
    </MaterialThemeProvider>
  </StylesProvider>
);

export default App;
