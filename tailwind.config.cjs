/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./index.html", "./src/frontend/**/*.{ts,tsx}"],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        background: "#050509",
        surface: "#111118",
        surfaceAlt: "#191927",
        accent: "#4f46e5",
        accentSoft: "#312e81",
        textPrimary: "#e5e7eb",
        textSecondary: "#9ca3af"
      },
      boxShadow: {
        launcher: "0 22px 70px rgba(0,0,0,0.7)"
      }
    }
  },
  plugins: []
};

