import { Bench } from 'tinybench'
import path from 'path'
import { fileURLToPath } from 'url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

import { matchTemplate, crop, findImage } from '../index.js'

const horsesJpg = path.join(__dirname, '../test/horses.jpg')
const horsesPng = path.join(__dirname, '../target/image/horses.png')
const horsesSectionPng = path.join(__dirname, '../target/image/horses_section.png')
// const horsesOutPng = path.join(__dirname, '../target/image/horses_out.png')

const b = new Bench()

crop(horsesJpg, 0, 0, 500, 400, horsesPng)
crop(horsesJpg, 400, 370, 30, 30, horsesSectionPng)

b.add('match', () => {
  matchTemplate(horsesPng, horsesSectionPng)
})

b.add('find', () => {
  findImage(horsesPng, horsesSectionPng)
})

await b.run()

console.table(b.table())
