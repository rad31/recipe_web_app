import type { Config } from 'tailwindcss'

export default {
  content: [ './app/**/*.{js,jsx,ts,tsx}'],
  theme: {
    extend: {
      colors: {
        'green-primary': '#528265',
        'green-secondary': '#689F7E',
        'blue-primary': '#525782',
        'purple-primary': '#82526F',
        'brown-primary': '#B6A9C9',
        'yellow-primary': '#FFD00D',
        'orange-primary': '#FFAF10',
      },
      fontFamily: {
        title: [
          'Big Shoulders Inline Display'
        ]
      },
    },
  },
  plugins: [],
} satisfies Config

