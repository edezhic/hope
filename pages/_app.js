import { ThemeProvider } from '@mui/material/styles';
import CssBaseline from '@mui/material/CssBaseline';
import { createTheme, responsiveFontSizes } from "@mui/material/styles";
import { deepPurple, amber } from "@mui/material/colors";
import "../globals.css";

let theme = createTheme({
  palette: {
    primary: deepPurple,
    secondary: amber,
  },
});

theme = responsiveFontSizes(theme);

export default function Hope({ Component, pageProps }) {
    return (
        <ThemeProvider theme={theme}>
            <CssBaseline />
            <Component {...pageProps} />
        </ThemeProvider>
    );
}
