const { whenProd } = require("@craco/craco");

module.exports = {
    webpack: {
      configure: (webpackConfig, { env, paths }) => {
      // This ensures the fallback is only added in production; adjust as necessary
      whenProd(() => {
        webpackConfig.resolve.fallback = {
          ...webpackConfig.resolve.fallback, // This spreads existing fallbacks to avoid overwriting them
          "crypto": require.resolve("crypto-browserify"),
          "stream": require.resolve("stream-browserify"),
          "http": require.resolve("stream-http"),
          "https": require.resolve("https-browserify"),
          "zlib": require.resolve("browserify-zlib"),
        };
      });

      return webpackConfig;
      },
    },
    // Add more custom configurations as needed
  };
  