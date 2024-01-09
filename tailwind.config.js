const { fontFamily } = require('tailwindcss/defaultTheme');
 
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ['./templates/**/*.html'],
    theme: {
        extend: {
            fontFamily: {
                mono: ['IBM Plex Mono', ...fontFamily.sans],
            },
            colors: {
                'accent': '#f09d50',
                'gable-green': {
                    '50': '#f3faf7',
                    '100': '#d8efe6',
                    '200': '#b0dfce',
                    '300': '#80c8b0',
                    '400': '#56ab92',
                    '500': '#3c9079',
                    '600': '#2e7362',
                    '700': '#285d50',
                    '800': '#244b43',
                    '900': '#1d3731',
                    '950': '#0f2421',
                },
                'tasman': {
                    '50': '#f5f8f5',
                    '100': '#e9f0e8',
                    '200': '#cbdac9',
                    '300': '#b1c7ae',
                    '400': '#86a682',
                    '500': '#648760',
                    '600': '#4f6d4c',
                    '700': '#41573e',
                    '800': '#364734',
                    '900': '#2d3b2c',
                    '950': '#151e15',
                },

            }
        },
    },
    plugins: [require('@tailwindcss/forms')],
};