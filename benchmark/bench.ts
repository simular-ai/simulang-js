import { Bench } from 'tinybench'

import { Machine } from '../index.js'

const b = new Bench()

const machine = Machine.local()

b.add('Screenshot (machine screen)', () => {
  machine.screenFromMouse().screenshot(true)
})

await b.run()

console.table(b.table())
