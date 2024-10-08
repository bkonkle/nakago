// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require("prism-react-renderer").themes.github;
const darkCodeTheme = require("prism-react-renderer").themes.okaidia;

/** @type {import('@docusaurus/types').Config} */
const config = {
    title: "Nakago (中子)",
    tagline: "A lightweight Rust toolkit for sharp dependency injection",
    favicon: "img/favicon.ico",

    url: "https://nakago.dev",
    baseUrl: "/",

    organizationName: "bkonkle",
    projectName: "nakago",
    trailingSlash: false,

    onBrokenLinks: "throw",
    onBrokenMarkdownLinks: "warn",

    i18n: {
        defaultLocale: "en",
        locales: ["en"],
    },

    presets: [
        [
            "classic",
            /** @type {import('@docusaurus/preset-classic').Options} */
            ({
                docs: {
                    sidebarPath: require.resolve("./sidebars.js"),
                    editUrl:
                        "https://github.com/bkonkle/nakago/tree/main/website/",
                },
                blog: {
                    showReadingTime: true,
                    editUrl:
                        "https://github.com/bkonkle/nakago/tree/main/website/",
                },
                theme: {
                    customCss: require.resolve("./src/css/custom.css"),
                },
            }),
        ],
    ],

    themeConfig:
        /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
        ({
            image: "img/nakago-social-card.jpg",
            navbar: {
                title: "Nakago",
                logo: {
                    alt: "Nakago Logo",
                    src: "img/favicon.ico",
                },
                items: [
                    {
                        type: "docSidebar",
                        sidebarId: "documentationSidebar",
                        position: "left",
                        label: "Documentation",
                    },
                    { to: "/blog", label: "Blog", position: "left" },
                    {
                        href: "https://github.com/bkonkle/nakago",
                        label: "GitHub",
                        position: "right",
                    },
                ],
            },
            footer: {
                style: "dark",
                links: [
                    {
                        title: "Docs",
                        items: [
                            {
                                label: "Documentation",
                                to: "/docs/intro",
                            },
                        ],
                    },
                    {
                        title: "Community",
                        items: [
                            {
                                label: "Stack Overflow",
                                href: "https://stackoverflow.com/questions/tagged/nakago",
                            },
                            {
                                label: "Discord",
                                href: "https://discord.gg/KasNDZSETK",
                            },
                            {
                                label: "Mastodon",
                                href: "https://fosstodon.org/@bkonkle",
                            },
                        ],
                    },
                    {
                        title: "More",
                        items: [
                            {
                                label: "Blog",
                                to: "/blog",
                            },
                            {
                                label: "GitHub",
                                href: "https://github.com/bkonkle/nakago",
                            },
                        ],
                    },
                ],
                copyright: `Copyright © ${new Date().getFullYear()} Brandon Konkle. Built with Docusaurus.`,
            },
            prism: {
                theme: lightCodeTheme,
                darkTheme: darkCodeTheme,
                additionalLanguages: ["toml", "rust"],
            },
        }),
};

module.exports = config;
