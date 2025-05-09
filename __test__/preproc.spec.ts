import { test, expect } from 'vitest'
import path from 'path'
import { fileURLToPath } from 'url'

import { crop, filterBinaryzation } from '../index'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const horsesJpg = path.join(__dirname, '../test/horses.jpg')
const horsesPng = path.join(__dirname, '../target/image/horses.png')
const horsesFilterPng = path.join(__dirname, '../target/image/horses_filter.png')

crop(horsesJpg, 0, 0, 500, 400, horsesPng)

test('filter binaryzation', () => {
  filterBinaryzation(horsesPng, horsesFilterPng, '0-128')
})
