import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/components/**/*.{js,ts,jsx,tsx,mdx}",
    "./src/app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      // Seshlog Brand Typography - Noto Sans (via Next.js font optimization)
      fontFamily: {
        sans: ['var(--font-noto-sans)', 'Noto Sans', 'sans-serif'],
      },
      colors: {
        background: "hsl(var(--background))",
        foreground: "hsl(var(--foreground))",
        // Seshlog Brand Colors - Amber palette
        primary: {
          DEFAULT: "hsl(var(--primary))",
          foreground: "hsl(var(--primary-foreground))",
        },
        secondary: {
          DEFAULT: "hsl(var(--secondary))",
          foreground: "hsl(var(--secondary-foreground))",
        },
        accent: {
          DEFAULT: "hsl(var(--accent))",
          foreground: "hsl(var(--accent-foreground))",
        },
        destructive: {
          DEFAULT: "hsl(var(--destructive))",
          foreground: "hsl(var(--destructive-foreground))",
        },
        muted: {
          DEFAULT: "hsl(var(--muted))",
          foreground: "hsl(var(--muted-foreground))",
        },
        card: {
          DEFAULT: "hsl(var(--card))",
          foreground: "hsl(var(--card-foreground))",
        },
        popover: {
          DEFAULT: "hsl(var(--popover))",
          foreground: "hsl(var(--popover-foreground))",
        },
        border: "hsl(var(--border))",
        input: "hsl(var(--input))",
        ring: "hsl(var(--ring))",
        // Status colors
        recording: "hsl(var(--recording))",
        processing: "hsl(var(--processing))",
        success: "hsl(var(--success))",
      },
      // Seshlog Typography Scale (from brandbook)
      fontSize: {
        'display': ['2.5rem', { lineHeight: '1.15', fontWeight: '700', letterSpacing: '-0.02em' }],
        'h1': ['1.75rem', { lineHeight: '1.25', fontWeight: '600', letterSpacing: '-0.015em' }],
        'h2': ['1.25rem', { lineHeight: '1.35', fontWeight: '600' }],
        'body': ['1.0625rem', { lineHeight: '1.75', fontWeight: '400' }],
        'ui': ['1rem', { lineHeight: '1.6', fontWeight: '400' }],
        'small': ['0.875rem', { lineHeight: '1.5', fontWeight: '400' }],
        'caption': ['0.75rem', { lineHeight: '1.4', fontWeight: '500' }],
        'speaker': ['0.9375rem', { lineHeight: '1.6', fontWeight: '500' }],
      },
      borderRadius: {
        lg: "var(--radius)",
        md: "calc(var(--radius) - 2px)",
        sm: "calc(var(--radius) - 4px)",
      },
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ],
} satisfies Config;
