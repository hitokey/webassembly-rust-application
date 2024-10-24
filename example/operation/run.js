

import init, { add_two_ints } from './out/operation.js';

async function run() {
    await init();
        console.log('2 + 3 =', add_two_ints(2, 3));
}

run();