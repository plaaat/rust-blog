module.exports = {
    mode: "jit",
    content: {
      files: ["src/**/*.rs", "**/*.html"],
    },
    darkMode: "class", // 'media' or 'class'
    theme: {
      container: {
        center: true,
        padding: '2rem',
        screens: {
          '2xl': '1400px'
        }
      },
      extend: {
        fontFamily: {
          'nanum-square': ['NanumSquare', 'sans-serif'],
          'nanum-gothic': ['Nanum Gothic', 'sans-serif'],
        },
      },
    },
    variants: {
      extend: {},
    },
    plugins: [],
  };