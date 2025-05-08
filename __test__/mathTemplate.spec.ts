import { test, expect } from 'vitest'
import path from 'path'
import { fileURLToPath } from 'url'

import { matchTemplate, crop, findImage, setAroundZero } from '../index'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const horsesJpg = path.join(__dirname, '../test/horses.jpg')
const horsesPng = path.join(__dirname, '../target/image/horses.png')
const horsesSectionPng = path.join(__dirname, '../target/image/horses_section.png')
const horsesOutPng = path.join(__dirname, '../target/image/horses_out.png')
const horsesSectionPngTransparent = path.join(__dirname, '../target/image/horses_section_transparent.png')

crop(horsesJpg, 0, 0, 500, 400, horsesPng)
crop(horsesJpg, 100, 100, 100, 100, horsesSectionPng)
crop(horsesJpg, 100, 100, 100, 100, horsesSectionPngTransparent)
crop(horsesJpg, 500, 400, 10, 10, horsesOutPng)
setAroundZero(horsesSectionPngTransparent, 10)

// test('match template', () => {
//   let start = Date.now()
//   const res = matchTemplate(horsesPng, horsesSectionPng)
//   console.log('⏰ matchTemplate time:', Date.now() - start, 'ms')
//   expect(JSON.stringify({ x: 100, y: 100 }), JSON.stringify({ x: res.x, y: res.y }))
// })

test('crop image', () => {
  crop(horsesJpg, 0, 0, 500, 400, horsesPng)
})

test('find image', () => {
  const res = findImage(horsesPng, horsesSectionPng)
  expect(JSON.stringify({ x: 100, y: 100 }), JSON.stringify({ x: res.x, y: res.y }))
})

test('find image out', () => {
  const res = findImage(horsesPng, horsesOutPng)
  expect(JSON.stringify({ x: 0, y: 0, score: 0 }), JSON.stringify({ x: res.x, y: res.y, score: res.score }))
})

test('find image with transparent border', () => {
  const res = findImage(horsesPng, horsesSectionPngTransparent)
  expect(JSON.stringify({ x: 100, y: 100 }), JSON.stringify({ x: res.x, y: res.y }))
})
