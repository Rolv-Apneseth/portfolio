module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                primary: "hsl(216, 65%, 11%)",
                secondary: "hsl(309, 91%, 80%)",
                tertiary: "hsl(289, 89%, 62%)",
            },
            fontFamily: {
                sans: [
                    'Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen-Sans, Ubuntu, Cantarell, "Helvetica Neue", sans-serif',
                ],
            },
        },
    },
    plugins: [],
};
