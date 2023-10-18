/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../src/**/*.{html,rs}"],
  theme: {
    colors: {
      transparent: 'transparent',
      current: 'currentColor',
      primary: {
        0: '#000000',
        10: '#001a40',
        20: '#002f67',
        25: '#00397c',
        30: '#004492',
        35: '#0050a8',
        40: '#005bbe',
        // 45: '#0067d5',
        50: '#2074e4',
        60: '#488fff',
        70: '#7eabff',
        80: '#acc7ff',
        90:' #d7e2ff',
        95: '#edf0ff',
        98: '#f9f9ff',
        99: '#fefbff',
        100: '#ffffff'
      },
      secondary: {
        0: '#000000',
        10: '#131c2c',
        20: '#283041',
        25: '#333b4d',
        30: '#3f4759',
        35: '#4a5265',
        40: '#565e71',
        50: '#6f778b',
        60: '#8991a5',
        70: '#a3abc0',
        80: '#bec6dc',
        90: '#dae2f9',
        95: '#edf0ff',
        98: '#f9f9ff',
        99: '#fefbff',
        100: '#ffffff'
      },
      tertiary: {
        0: '#000000',
        10: '#29132e',
        20: '#402844',
        25: '#4b334f',
        30: '#573e5b',
        35: '#644967',
        40: '#705574',
        50: '#8a6d8d',
        60: '#a587a8',
        70: '#c1a1c3',
        80: '#debcdf',
        90: '#fbd7fc',
        95: '#ffebfd',
        98: '#fff7fa',
        99: '#fffbff',
        100: '#ffffff'
      },
      error: {
        0: '#000000',
        10: '#410002',
        40: '#ba1a1a',
        90: '#ffdad6',
        100: '#ffffff'
      },

    },
    extend: {
      keyframes: {
        loadbar: {
          '0%, 100%': { 'inset-inline-start': '0%' },
          '50%': { 'inset-inline-start': 'calc(100% - 4rem)' },
        }
      },
      animation: {
        loadbar: 'loadbar 1s ease-in-out infinite',
      }
    },
  },
  plugins: [],
}

