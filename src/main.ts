/*
 * The current contents of this file are for
 * sanity testing `vujio_client/mangle.rs`,
 * minification, and source map generation.
 */

import hello from "./components/hello";

var moduleName = 'main';
moduleName += '.ts';
console.log(moduleName);

if(false) {
    // dead code should be removed from bundled output,
    // but remain in source maps.
    alert('----------------dead code -------------------');
}

const test = () => "ok";
test();

hello();
