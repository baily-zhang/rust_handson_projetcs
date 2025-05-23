/** @type {import('tailwindcss').Config} */

// tailwind.config.js

module.exports = {
  content: [
    "./src/**/*.{js,ts,jsx,tsx}", // 👈 告诉 Tailwind 去哪里找 class
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
