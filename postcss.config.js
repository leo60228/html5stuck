module.exports = {
    map: {
        inline: false,
    },
    plugins: [
        require('postcss-advanced-variables')(),
        require('postcss-calc')(),
        require('postcss-preset-env')(),
        require('postcss-nested')(),
        process.env.NODE_ENV == 'production'
            ? require('cssnano')({
                  preset: 'default',
              })
            : require('stylefmt'),
    ],
};
