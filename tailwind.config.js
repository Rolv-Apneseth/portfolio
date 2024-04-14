module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
    },
    darkMode: "class",
    theme: {
        extend: {
            colors: {
                primary: "hsl(216, 65%, 11%)",
            },
            fontFamily: {
                sans: [
                    "Inter",
                    "-apple-system",
                    "BlinkMacSystemFont",
                    '"Segoe UI"',
                    "Roboto",
                    "Oxygen-Sans",
                    "Ubuntu",
                    "Cantarell",
                    '"Helvetica Neue"',
                    "sans-serif",
                ],
            },
        },
    },
    plugins: [],
};
