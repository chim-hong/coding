import {red} from 'fmt/colors.ts'
console.log('welcome, this is myself file!')

import {copy} from 'https://deno.land/std@0.171.0/fs/copy.ts'

const args = Deno.args;

copy('main.ts','main-back.ts')

console.log('red',red('222'))

console.log('Deno args is :', args)
