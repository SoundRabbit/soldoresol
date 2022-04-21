const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    mode: "production",
    experiments: { syncWebAssembly: true },
    entry: "./assets",
    output: {
        path: path.join(__dirname, "./bin"),
    },
    resolve: {
        extensions: [".js"]
    },
    // TODO: ファイルのダイナミック読み込み
    performance: { hints: false },
    module: {
        rules: [
            {
                test: /\.scss$/,
                use: [
                    {
                        loader: "style-loader",
                    },
                    {
                        loader: "css-loader",
                    },
                    {
                        loader: "sass-loader",
                        options: {
                            sassOptions: {
                                includePaths: [path.resolve(__dirname, './style/style.scss')]
                            },
                            sourceMap: true,
                        },
                    },
                ],
            },
            {
                test: /\.(png|jpg|gif)$/i,
                use: [
                    {
                        loader: "url-loader",
                        options: {
                            limit: 8192,
                        },
                    },
                ]
            },
            {
                test: /\.svg$/i,
                use: [
                    {
                        loader: "svg-url-loader",
                    },
                ]
            },
        ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            template: path.join(__dirname, "./assets/index.html"),
            inlineSource: ".(js|css)$"
        }),
        new WasmPackPlugin({
            crateDirectory: path.join(__dirname, "./"),
            forceMode: "production",
            target: "web",
            args: "--log-level error",
        }),
    ],
    devServer: {
        historyApiFallback: {
            rewrites: [
                {
                    from: /^\/rooms$/,
                    to: function () {
                        return '/';
                    }
                },
                {
                    from: /^\/rooms\/skyway\/([A - Za - z0 - 9@#]{24})$/,
                    to: function () {
                        return '/';
                    }
                },
                {
                    from: /^\/rooms\/drive\/([A-Za-z\-_]+)$/,
                    to: function () {
                        return '/';
                    }
                },
            ]
        },
        static: [
            {
                directory: path.join(__dirname, "./assets"),
                publicPath: "/",
            }
        ]
    }
};