import { theme, type ThemeConfig } from 'antd'

export const appTheme: ThemeConfig = {
  algorithm: theme.darkAlgorithm,
  token: {
    colorBgLayout: 'rgba(50,19,37,1)',     // page background (was #f5f5f5)
    colorBgContainer: 'rgb(71, 24, 50)',  // inputs, select, Listy group headers
    colorBgElevated: '#2e041d',   // dropdowns, popovers, image preview
    colorPrimary: '#FFBA08',      // focus rings, primary buttons, selected items
    colorLink: '#FFBA08',         // links (a separate seed token; doesn't follow colorPrimary)
    colorTextSecondary: 'rgba(255, 255, 255, 0.75)',
    colorTextDescription: 'rgba(255, 255, 255, 0.75)',
    colorTextPlaceholder: 'rgba(255, 255, 255, 0.55)', // input/select placeholder text 
  },
  components: {
    Layout: { headerBg: 'rgb(86, 32, 62)' },
    Button: { primaryColor: '#03071E' }, // dark text on the gold primary button
  },
}
