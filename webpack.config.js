const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const HtmlWebpackInlineSourcePlugin = require("html-webpack-inline-source-plugin");
const history = require('connect-history-api-fallback');

history({
    rewrites: [
        {
            from: /([^/]*)$/,
            to: function (context) {
                return '/' + context.match[1];
            }
        }
    ]
});

module.exports = {
    entry: "./assets",
    output: {
        path: path.join(__dirname, "./bin"),
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
            forceMode: "production",
            target: "web",
            args: "--log-level error",
        }),
        new HtmlWebpackInlineSourcePlugin(),
    ],
    devServer: {
        historyApiFallback: {
            rewrites: [
                {
                    from: /([^/]*\.(js|wasm))$/,
                    to: function (context) {
                        return '/' + context.match[1];
                    }
                }
            ]
        },
        disableHostCheck: true,
    }
};