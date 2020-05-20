import prettier from 'rollup-plugin-prettier';
import { terser } from 'rollup-plugin-terser';
import { getBabelOutputPlugin } from '@rollup/plugin-babel';

export default {
    input: 'js/main.js',
    output: {
        file: 'html5/js/scripts.js',
        format: 'esm',
        sourcemap: true,
    },
    plugins: [
        getBabelOutputPlugin({
            caller: { supportsDynamicImport: true },
            presets: ['@babel/preset-env'],
        }),
        process.env.NODE_ENV == 'production'
            ? terser()
            : prettier({
                  sourcemap: 'silent',
                  tabWidth: 4,
                  parser: 'babel',
                  singleQuote: true,
              }),
    ],
};
