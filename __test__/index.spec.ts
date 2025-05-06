import test from 'ava'
import path from 'path'
import { fileURLToPath } from 'url'

import { matchTemplate, crop, findImage } from '../index'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const horsesJpg = path.join(__dirname, '../test/horses.jpg')
const horsesPng = path.join(__dirname, '../target/image/horses.png')
const horsesSectionPng = path.join(__dirname, '../target/image/horses_section.png')
const horsesOutPng = path.join(__dirname, '../target/image/horses_out.png')

crop(horsesJpg, 0, 0, 500, 400, horsesPng)
crop(horsesJpg, 100, 100, 100, 100, horsesSectionPng)
crop(horsesJpg, 500, 400, 10, 10, horsesOutPng)

test('match template', (t) => {
  let start = Date.now()
  const res = matchTemplate(horsesPng, horsesSectionPng)
  console.log('⏰ matchTemplate time:', Date.now() - start, 'ms')
  t.is(JSON.stringify({ x: 100, y: 100 }), JSON.stringify({ x: res.x, y: res.y }))
})

test('find image', (t) => {
  let start = Date.now()
  const res = findImage(horsesPng, horsesSectionPng)
  console.log('⏰ findImage time:', Date.now() - start, 'ms')
  t.is(JSON.stringify({ x: 100, y: 100 }), JSON.stringify({ x: res.x, y: res.y }))
})

test('find image out', (t) => {
  let start = Date.now()
  const res = findImage(horsesPng, horsesOutPng)
  console.log('⏰ findImage out time:', Date.now() - start, 'ms\n')
  t.is(JSON.stringify({ x: 0, y: 0, score: 0 }), JSON.stringify({ x: res.x, y: res.y, score: res.score }))
})
