/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './index.md',
    './download/**/*.md',
    './features/**/*.md',
    './guide/**/*.md',
    './changelog.md',
    './.vitepress/**/*.{js,ts,vue}'
  ],
  theme: {
    extend: {}
  },
  plugins: []
};
