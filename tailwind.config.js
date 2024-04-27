/** @type {import('tailwindcss').Config} */
export default {
  darkMode: 'class',
  purge: {
    content: ["./src/**/*.{js,ts,jsx,tsx}"],
    options: {
      safelist: ['alert-success', 'alert-error', 'alert-info', 'alert-warning', 'animate-slide-up', 'animate-slide-down']
    }
    
  },
  theme: {
    extend: {
      fontSize: {
        '2xs': '.65rem',
      },
      fontFamily: {
        "noto": ["Noto Sans Javanese", "sans-serif"]
      },
      animation: {
        'slide-up': 'slide-up 0.5s forwards',
        'slide-down': 'slide-down 0.5s forwards',
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('daisyui')
  ],
  daisyui: {
    themes: ["dark"]
  }
}

