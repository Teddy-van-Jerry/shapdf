/** @type {import('tailwindcss').Config} */
export default {
  content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
    extend: {
      colors: {
        shapdf: {
          50: "#e8f6ef",
          100: "#c4ead6",
          200: "#9dddbc",
          300: "#6dce9f",
          400: "#3fbe80",
          500: "#1ea566",
          600: "#0f8a55",
          700: "#0a6d44",
          800: "#085238",
          900: "#053025",
        },
      },
      boxShadow: {
        glow: "0 20px 45px -20px rgba(31, 167, 105, 0.45)",
      },
      fontFamily: {
        mono: [
          "'Fira Code'",
          "ui-monospace",
          "SFMono-Regular",
          "Menlo",
          "Monaco",
          "Consolas",
          "'Liberation Mono'",
          "'Courier New'",
          "monospace",
        ],
      },
    },
  },
  plugins: [require("@tailwindcss/forms")],
};
