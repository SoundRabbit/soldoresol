const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackInlineSourcePlugin = require("html-webpack-inline-source-plugin");

module.exports = {
    entry: "./assets",
    output: {
        path: path.join(__dirname, "./docs/dev")
    },
    resolve: {
        extensions: [".js"]
    },
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
                        loader: 'resolve-url-loader',
                    },
                    {
                        loader: "sass-loader",
                        options: {
                            sassOptions: {
                                includePaths: [path.resolve(__dirname, './style')]
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
            // forceMode: "development",
            forceMode: "production",
            target: "web",
            args: "--log-level error",
        }),
        new HtmlWebpackInlineSourcePlugin(),
    ],
    devServer: {
        historyApiFallback: true,
    }
};