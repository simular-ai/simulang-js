import { Bench } from 'tinybench'

import { Screen, screenshotFull } from '../index.js'

const b = new Bench()

b.add('Screenshot full (main screen)', () => {
  const screen = Screen.mainScreen()
  screenshotFull(true, screen)
})

await b.run()

console.table(b.table())
