/*
 * The current contents of this file are for
 * sanity testing `vujio_client/mangle.rs`,
 * minification, and source map generation.
 */

var moduleName = 'hello';
moduleName += '.ts';
console.log(moduleName);

var _0 = 'var _0 test';
_0 += '.';
console.log(_0);

export default async function hello(): Promise<String> {
    let moduleVar = 'redeclared local';

    return moduleVar;
}
