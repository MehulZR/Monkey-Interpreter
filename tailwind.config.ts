import type { Config } from "tailwindcss";

const config: Config = {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    container: {
      center: true
    },
    extend: {
      colors: {
        icon: 'var(--color-icon)',
        terminal: 'var(--color-terminal)',
        hover: 'var(--color-hover)',
        'primary-accent': 'var(--color-primary-accent)',
        'secondary-accent': 'var(--color-secondary-accent)',
        'cta-text': 'var(--color-CTA-text)'
      },
      backgroundColor: {
        primary: 'var(--color-bg-primary)',
        secondary: 'var(--color-bg-secondary)'
      },
      textColor: {
        primary: 'var(--color-text)',
        placeholder: 'var(--color-placeholder)',
        terminal: 'var(--color-terminal-text)',
        'terminal-placeholder': 'var(--color-terminal-placeholder)'
      },
      borderColor: {
        primary: 'var(--color-border)',
        terminal: 'var(--color-border-terminal)'
      },
      borderRadius: {
        lg: 'var(--radius)',
        md: 'calc(var(--radius) - 2px)',
        sm: 'calc(var(--radius) - 4px)'
      }
    }
  },
  plugins: [require("tailwindcss-animate")],
  darkMode: ["class"],
};

export default config;
