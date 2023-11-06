/** @type {import('tailwindcss').Config} */
export default {
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
    theme: {
        extend: {
            colors: {
                "green-primary": "#528265",
                "green-secondary": "#689F7E",
                "blue-primary": "#525782",
                "light-blue": "#ACECEF",
                "purple-primary": "#82526F",
                "brown-primary": "#B6A9C9",
                "yellow-primary": "#FFD00D",
                "orange-primary": "#FFAF10",
                "grey-primary": "#DDDDDD",
            },
            fontFamily: {
                title: ["Big Shoulders Inline Display"],
            },
        },
    },
    plugins: [],
};
